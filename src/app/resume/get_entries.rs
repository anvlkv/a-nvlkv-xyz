use leptos::*;

use crate::app::{state::CvEntry, Language};

#[server(GetCvEntries, "/api")]
pub async fn get_cv_entries(lang: Language) -> Result<Vec<CvEntry>, ServerFnError<String>> {
    use std::str::FromStr;

    use spin_sdk::pg::Decode;

    use crate::{
        app::util::coalesce_translations,
        server::{get_db_conn, safe_error},
    };

    println!("Getting cv entries {lang:?}");

    let conn = get_db_conn().map_err(safe_error)?;

    let locale = match lang {
        Language::En => chrono::Locale::en_US,
        Language::Nl => chrono::Locale::nl_NL,
        Language::Ru => chrono::Locale::ru_RU,
        Language::Ja => chrono::Locale::ja_JP,
    };

    let sql = format!(
        r#"
    SELECT cv.xata_id as id,
    to_json(cv.start_date) #>> '{{}}' as start_date,
    to_json(cv.end_date) #>> '{{}}' as end_date,
    cv.org_name,
    {} AS title,
    {} AS description,
    {} #>> '{{}}' AS skills
            FROM "cv"
            LEFT JOIN "localized_text" AS lt_title ON lt_title.xata_id = cv.title
            LEFT JOIN "localized_text" AS lt_description ON lt_description.xata_id = cv.description
            LEFT JOIN "localized_json" AS lj_skills ON lj_skills.xata_id = cv.skills
            ORDER BY cv.start_date DESC;
        "#,
        coalesce_translations("lt_title", &lang),
        coalesce_translations("lt_description", &lang),
        coalesce_translations("lj_skills", &lang),
    );

    let data = conn.query(sql.as_str(), &[]).map_err(safe_error)?;

    let entries_data: Vec<_> = data
        .rows
        .into_iter()
        .try_fold(vec![], |mut acc, row| {
            let id = String::decode(&row[0])?;
            let start_date = String::decode(&row[1])?;
            let end_date = Option::<String>::decode(&row[2])?;
            let org_name = String::decode(&row[3])?;

            let title = String::decode(&row[5])?;
            let description = String::decode(&row[7])?;
            let skills = String::decode(&row[9])
                .map(|s| serde_json::from_str::<Vec<String>>(s.as_str()).ok())
                .ok()
                .flatten()
                .unwrap_or_default();

            let translation_warning =
                bool::decode(&row[4])? || bool::decode(&row[6])? || bool::decode(&row[8])?;

            acc.push(CvEntry {
                id,
                start_date: chrono::DateTime::<chrono::Utc>::from_str(start_date.as_str())
                    .unwrap()
                    .format_localized("%b - %Y", locale)
                    .to_string(),
                end_date: end_date.map(|d| {
                    chrono::DateTime::<chrono::Utc>::from_str(d.as_str())
                        .unwrap()
                        .format_localized("%b - %Y", locale)
                        .to_string()
                }),
                title,
                description,
                skills,
                org_name,
                translation_warning,
            });
            Ok(acc)
        })
        .map_err(|e: anyhow::Error| e.to_string())?;

    Ok(entries_data)
}
