use leptos::*;

use crate::app::state::WorkSheets;

#[server(InquireInferrence, "/api")]
pub async fn inquire_inferrence(wk: WorkSheets) -> Result<String, ServerFnError<String>> {
    use spin_sdk::llm;

    println!("wk: {wk:#?}");

    let wk_json = serde_json::to_string(&wk).map_err(|e| e.to_string())?;

    let response = llm::infer(llm::InferencingModel::Llama2Chat, "2+2 = ")
        .map_err(|e| e.to_string())?;

    Ok(response.text)
}

#[server(InquirePersonal, "/api")]
pub async fn inquire_personal(wk: WorkSheets) -> Result<(), ServerFnError<String>> {
    use spin_sdk::pg::ParameterValue;

    use crate::server::get_db_conn;

    let conn = get_db_conn().map_err(|e| e.to_string())?;

    let wk_data = serde_json::to_string(&wk).map_err(|e| e.to_string())?;
    let name = wk.inquire.contact.name;
    let email = wk.inquire.contact.email;
    let message = wk.inquire.contact.message;

    let sql = r#"
        INSERT INTO "personal_inquery" (name, email, message, wk)
        VALUES($1, $2, $3, $4::json));
    "#;

    let params = [
        ParameterValue::Str(name),
        ParameterValue::Str(email),
        ParameterValue::Str(message),
        ParameterValue::Str(wk_data),
    ];

    conn.execute(sql, &params).map_err(|e| e.to_string())?;

    Ok(())
}
