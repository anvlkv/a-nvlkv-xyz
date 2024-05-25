use leptos::*;

use crate::app::{state::ProjectData, util::coalesce_translations, Language};

/// returns projects with translations for a selected langugae
///
/// paged with count and offset
///
/// returns an estimate for number of projects in db
#[server(GetProjects, "/api")]
pub async fn get_projects(
    lang: Language,
    count: usize,
    offset: usize,
    wk_only: bool,
) -> Result<(Vec<ProjectData>, f32), ServerFnError<String>> {
    use crate::{
        app::state::WorkSheets,
        server::{get_db_conn, xata_rest_builder},
    };
    use spin_sdk::{
        http::{run, send, Method, Response},
        pg::{Decode, ParameterValue},
    };

    log::debug!("Getting examples {lang:?} {count} : {offset}");

    let conn = get_db_conn().map_err(|e| e.to_string())?;

    let sql = r#"
        SELECT reltuples AS estimate FROM pg_class where relname = 'projects';
        "#;

    let projects_count = conn
        .query(sql, &[])
        .map(|d| f32::decode(&d.rows[0][0]).unwrap())
        .map_err(|e| e.to_string())?;

    let sql = format!(
        r#"
    SELECT projects.xata_id as id,
    {} AS title,
    {} AS description,
    {} #>> '{{}}' AS wk
            FROM {}
            LEFT JOIN "localized_text" AS lt_title ON lt_title.xata_id = projects.title
            LEFT JOIN "localized_text" AS lt_description ON lt_description.xata_id = projects.description
            LEFT JOIN "localized_json" AS lj_worksheets ON lj_worksheets.xata_id = projects.worksheets
            ORDER BY projects.weight ASC
            LIMIT $1 OFFSET $2;
        "#,
        coalesce_translations("lt_title", &lang),
        coalesce_translations("lt_description", &lang),
        coalesce_translations("lj_worksheets", &lang),
        if wk_only {
            r#"(
                SELECT * FROM "projects"
                WHERE worksheets IS NOT NULL
            ) AS projects"#
        } else {
            r#""projects""#
        }
    );

    let params = vec![
        ParameterValue::Int64(count as i64),
        ParameterValue::Int64(offset as i64),
    ];

    let data = conn
        .query(sql.as_str(), params.as_slice())
        .map_err(|e| e.to_string())?;

    let examples_data: Vec<_> = data
        .rows
        .into_iter()
        .try_fold(vec![], |mut acc, row| {
            let id = String::decode(&row[0])?;
            let title = String::decode(&row[2])?;
            let description = String::decode(&row[4])?;
            let wk = String::decode(&row[6])
                .map(|s| serde_json::from_str::<WorkSheets>(s.as_str()).ok())
                .ok()
                .flatten();
            let translation_warning =
                bool::decode(&row[1])? || bool::decode(&row[3])? || bool::decode(&row[5])?;
            acc.push((id, title, description, wk, translation_warning));
            Ok(acc)
        })
        .map_err(|e: anyhow::Error| e.to_string())?;

    let project_ids = examples_data
        .iter()
        .fold(String::default(), |acc, (id, ..)| {
            format!(r#"{acc}{}"{id}""#, if acc.is_empty() { "" } else { "," })
        });

    let mut images_req =
        xata_rest_builder("tables/projects/query").map_err(|e: anyhow::Error| e.to_string())?;

    images_req.method(Method::Post).body(format!(
        r#"{{
        "columns": ["main_image.url", "id"],
        "filter": {{ "id" : {{ "$any": [{project_ids}] }} }}
        }}"#
    ));

    let res = run(async move {
        let res: Response = send(images_req).await?;
        Ok(res.into_body())
    })
    .map_err(|e: anyhow::Error| e.to_string())?;

    let json_string = String::from_utf8_lossy(&res).to_string();
    let images_data = jzon::parse(json_string.as_str()).map_err(|e| e.to_string())?;

    let examples = examples_data
        .into_iter()
        .map(|(id, title, description, wk, translation_warning)| {
            let record = images_data["records"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| {
                    r["id"]
                        .as_str()
                        .map(|d| d == id.as_str())
                        .unwrap_or_default()
                })
                .unwrap();

            let main_image_url = record["main_image"]["url"].as_str().map(|s| s.to_string());

            ProjectData {
                id,
                wk,
                title,
                description,
                translation_warning,
                main_image_url,
            }
        })
        .collect::<Vec<_>>();

    Ok((examples, projects_count))
}
