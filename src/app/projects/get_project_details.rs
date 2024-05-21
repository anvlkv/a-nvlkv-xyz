use leptos::*;

use crate::app::{state::ExtendedProjectData, util::coalesce_translations, Language};

/// returns project with id, in selected langugae
#[server(GetProjectDetails, "/api")]
pub async fn get_project_details(
    lang: Language,
    id: String,
) -> Result<ExtendedProjectData, ServerFnError<String>> {
    use crate::{
        app::state::WorkSheets,
        server::{get_db_conn, xata_rest_builder},
    };
    use spin_sdk::{
        http::{run, send, Method, Response},
        pg::{Decode, ParameterValue},
    };

    let conn = get_db_conn().map_err(|e| e.to_string())?;

    let sql = format!(
        r#"
    SELECT projects.xata_id as id,
    {} AS title,
    {} AS description,
    {} AS description_2,
    {} #>> '{{}}' AS wk
            FROM "projects"
            LEFT JOIN "localized_text" AS lt_title ON lt_title.xata_id = projects.title
            LEFT JOIN "localized_text" AS lt_description ON lt_description.xata_id = projects.description
            LEFT JOIN "localized_text" AS lt_description_2 ON lt_description_2.xata_id = projects.description_2
            LEFT JOIN "localized_json" AS lj_worksheets ON lj_worksheets.xata_id = projects.worksheets
            WHERE projects.xata_id = $1;
        "#,
        coalesce_translations("lt_title", &lang),
        coalesce_translations("lt_description", &lang),
        coalesce_translations("lt_description_2", &lang),
        coalesce_translations("lj_worksheets", &lang),
    );

    let params = vec![ParameterValue::Str(id.into())];

    let data = conn
        .query(sql.as_str(), params.as_slice())
        .map_err(|e| e.to_string())?;

    let row = data
        .rows
        .first()
        .ok_or(ServerFnError::ServerError("Not found".to_string()))?;

    let id = String::decode(&row[0]).map_err(|e| e.to_string())?;
    let title = String::decode(&row[2]).map_err(|e| e.to_string())?;
    let description = String::decode(&row[4]).map_err(|e| e.to_string())?;
    let description_2 = String::decode(&row[6]).unwrap_or_default();
    let article = description_2.lines().map(|l| l.to_string()).collect();

    let wk = String::decode(&row[8])
        .map(|s| serde_json::from_str::<WorkSheets>(s.as_str()).ok())
        .ok()
        .flatten();
    let translation_warning = bool::decode(&row[1]).map_err(|e| e.to_string())?
        || bool::decode(&row[3]).map_err(|e| e.to_string())?
        || bool::decode(&row[5]).map_err(|e| e.to_string())?
        || bool::decode(&row[7]).map_err(|e| e.to_string())?;

    let mut images_req =
        xata_rest_builder("tables/projects/query").map_err(|e: anyhow::Error| e.to_string())?;

    images_req.method(Method::Post).body(format!(
        r#"{{
        "columns": ["main_image.url", "id", "images.url"],
        "filter": {{ "id" : "{id}" }}
        }}"#
    ));

    let res = run(async move {
        let res: Response = send(images_req).await?;
        Ok(res.into_body())
    })
    .map_err(|e: anyhow::Error| e.to_string())?;

    let json_string = String::from_utf8_lossy(&res).to_string();
    let images_data = jzon::parse(json_string.as_str()).map_err(|e| e.to_string())?;

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

    let images = record["images"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|i| i["url"].as_str().map(|s| s.to_string()))
        .collect::<Vec<_>>();

    Ok(ExtendedProjectData {
        id,
        wk,
        title,
        description,
        article,
        translation_warning,
        main_image_url,
        images,
    })
}
