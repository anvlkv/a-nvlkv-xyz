#![cfg_attr(not(feature = "ssr"), allow(unused))]

use std::str::FromStr;

use leptos::*;
use uuid::Uuid;

use crate::app::state::{Contact, InqueryOption, WorkSheets};

fn sanitize_input(value: String) -> String {
    const CLEAR_OUT: &[&str] = &["[INST]", "[/INST]", "<<SYS>>", "<</SYS>>"];

    let mut sanitized = value;
    for tok in CLEAR_OUT {
        sanitized = sanitized.replace(tok, "")
    }
    sanitized
}

#[cfg_attr(debug_assertions, allow(unreachable_code, unused))]
#[server(InquireInferrence, "/api")]
pub async fn inquire_inferrence(
    wk: WorkSheets,
    tracking_id: Option<Uuid>,
) -> Result<String, ServerFnError<String>> {
    println!("inquire inferrence");

    #[cfg(debug_assertions)]
    {
        return Ok("Helpful answer".to_string());
    }

    use crate::{app::tracking::complete_inferrence, server::safe_error};
    use spin_sdk::llm;

    let WorkSheets {
        problem,
        solutions,
        compromise,
        implement,
        iterate,
        inquire,
    } = wk;

    let problem = sanitize_input(serde_json::to_string(&problem).map_err(safe_error)?);
    let solutions = sanitize_input(serde_json::to_string(&solutions).map_err(safe_error)?);
    let compromise = sanitize_input(serde_json::to_string(&compromise).map_err(safe_error)?);
    let implement = sanitize_input(serde_json::to_string(&implement).map_err(safe_error)?);
    let iterate = sanitize_input(serde_json::to_string(&iterate).map_err(safe_error)?);

    let (inst, i_ctx, max_tokens, temperature) = match InqueryOption::from_str(inquire.inquery_option.as_str()).map_err(safe_error)? {
        InqueryOption::FirstTime => (
            "It is a first time entry. How to improve it?".to_string(),
            Some("Some common mistakes are: choosing a problem which is too intrinsic, forgeting some important stakeholders, confusing stakeholders to shareholders, defining a solution too technically or too vaguely, not having outlined the research, forgetting some necessary resources, showing signs of change avoidance."),
            1024,
            0.85
        ),
        InqueryOption::ScopeAndTime => (
            "How to adjust the scope and timeframe of this iteration?".to_string(),
            Some("It is common to wish for too much at once, underestimate time for implementation and testing a solution."),
            512,
            0.65
        ),
        InqueryOption::EthicalDesign => (
            "Suggest an ethical approach to implementing and testing the proposed solution.".to_string(),
            Some("Ethical design would be concerned with feelings and future wellbeing of enlisted or other potential stakeholders."),
            512,
            0.5
        ),
        InqueryOption::Narrative => (
            "Suggest a narrative communicating the main idea of this iteration to a broader audience.".to_string(),
            Some("A helpfull narrative would illustrate a usecase, use a presona, relatable wording."),
            2056,
            0.9
        ),
        InqueryOption::Custom => (
            sanitize_input(inquire.custom_prompt),
            None,
            2056,
            0.6
        ),
    };

    let prompt = format!(
        r#"
<<SYS>>
    You are helping the user to complete and get preliminary feedback on a problem-solving exercise.
<</SYS>>
<<SYS>>
    Following is the worksheets completed by the user:
    Problem:
    ```json
        {problem}
    ```
    Solutions:
    ```json
        {solutions}
    ```
    Compromise:
    ```json
        {compromise}
    ```
    Implement:
    ```json
        {implement}
    ```
    Test & Iterate:
    ```json
        {iterate}
    ```
<</SYS>>
{}
[INST]
    {inst}
[/INST]
    "#,
        if let Some(i_ctx) = i_ctx {
            format!("<<SYS>>{i_ctx}<</SYS>>")
        } else {
            String::default()
        }
    );

    let response = llm::infer_with_options(
        llm::InferencingModel::Llama2Chat,
        prompt.as_str(),
        llm::InferencingParams {
            max_tokens,
            repeat_penalty: 1.1,
            repeat_penalty_last_n_token_count: 64,
            temperature,
            top_k: 40,
            top_p: 0.82,
        },
    )
    .map_err(safe_error)?;

    if let Some(tracking_id) = tracking_id {
        _ = complete_inferrence(tracking_id, response.text.clone());
    }

    Ok(response.text)
}

#[server(InquirePersonal, "/api")]
pub async fn inquire_personal(
    wk: Option<WorkSheets>,
    contact: Contact,
    tracking_id: Option<Uuid>,
) -> Result<(), ServerFnError<String>> {
    println!("inquire personal");

    use spin_sdk::pg::{Decode, ParameterValue};

    use crate::{
        app::tracking::complete_personal,
        server::{get_db_conn, safe_error},
    };

    let conn = get_db_conn().map_err(safe_error)?;

    let wk_data = serde_json::to_string(&wk).map_err(safe_error)?;
    let name = contact.name;
    let email = contact.email;
    let message = contact.message;

    let sql = r#"
        INSERT INTO "personal_inquery" (name, email, message, wk)
        VALUES($1, $2, $3, $4::text::json)
        RETURNING xata_id;
    "#;

    let params = [
        ParameterValue::Str(name),
        ParameterValue::Str(email),
        ParameterValue::Str(message),
        ParameterValue::Str(wk_data),
    ];

    let data = conn.query(sql, &params).map_err(safe_error)?;

    let id = String::decode(&data.rows[0][0]).map_err(safe_error)?;

    if let Some(tracking_id) = tracking_id {
        _ = complete_personal(tracking_id, id);
    }

    Ok(())
}

#[server(InquireContact, "/api")]
pub async fn inquire_contact(
    name: String,
    email: String,
    message: String,
    session_id: Option<Uuid>,
) -> Result<String, ServerFnError<String>> {
    inquire_personal(
        None,
        Contact {
            name,
            email,
            message,
        },
        session_id,
    )
    .await
    .map(|_| "Message sent".to_string())
}
