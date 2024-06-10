use leptos::*;
use leptos_meta::*;

use crate::app::{components::use_wk_state, state::WorkSheets};

#[component]
pub fn WorksheetsDownload() -> impl IntoView {
    let ctx = use_wk_state();
    let data = Signal::derive(move || {
        let wk: WorkSheets = ctx.get().get();
        wk
    });
    let problem_statement = move || data.get().problem.problem_statement;
    let solutions = move || data.get().compromise.solution_choices;
    let stakeholders = move || data.get().compromise.stakeholder_choices;
    let question = move || data.get().compromise.question;
    let now = move || data.get().implement.now;
    let best = move || data.get().implement.best;

    view! {
        <Script src="https://unpkg.com/pdfjs-dist@4.3.136/build/pdf.min.mjs" attr:async=true attr:type="module" />
        <div>
            <h1>
                {t!("about.title")}
            </h1>
            <ul class="list-[square]">
                <li>
                    <p>{t!("worksheets.download.instruction_1")}</p>
                </li>
                <li>
                    <p>{t!("worksheets.download.instruction_2")}</p>
                </li>
                <li>
                    <p>{t!("worksheets.download.instruction_3")}</p>
                </li>
                <li>
                    <p>{t!("worksheets.download.instruction_4")}</p>
                </li>
            </ul>
            <h2>
                {t!("worksheets.problem.label_statement")}
            </h2>
            <p>
                {problem_statement}
            </p>
            <div class="grid grid-cols-2">
                <div>
                    <h3>
                        {t!("worksheets.compromise.label_solutions")}
                    </h3>
                    <ul>
                        {move || solutions().iter().map(|s| view!{<li>{s}</li>}).collect_view()}
                    </ul>
                </div>
                <div>
                    <h3>
                        {t!("worksheets.compromise.label_stakeholders")}
                    </h3>
                    <ul>
                        {move || stakeholders().iter().map(|s| view!{<li>{s}</li>}).collect_view()}
                    </ul>
                </div>
            </div>
            <h2>
                {t!("worksheets.compromise.label_question")}
            </h2>
            <p>
                {question}
            </p>
            <h2>
                {t!("worksheets.implement.title")}
            </h2>
            <div class="grid grid-cols-2">
                <div>
                    <h3>
                        {t!("worksheets.implement.label_now")}
                    </h3>
                    <ul>
                        {move || now().iter().map(|s| view!{<li>{s}</li>}).collect_view()}
                    </ul>
                </div>
                <div>
                    <h3>
                        {t!("worksheets.implement.label_best")}
                    </h3>
                    <ul>
                        {move || best().iter().map(|s| view!{<li>{s}</li>}).collect_view()}
                    </ul>
                </div>
            </div>
        </div>
    }
}
