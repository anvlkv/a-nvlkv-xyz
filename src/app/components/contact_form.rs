use leptos::*;

#[component]
pub fn ContactFormView() -> impl IntoView {
    view! {
        <h2 class="w-full text-center text-2xl md:text-3x xl:text-4xl pb-6">{t!("contact.title")}</h2>
        <div class="grid lg:grid-cols-2 gap-10">
            <div class="max-w-prose">
                <p class="pb-4">{t!("contact.description")}</p>
            </div>
            <form class="flex flex-col">
                <label>
                    <p>{t!("contact.name.label")}</p>
                    <input placeholder={t!("contact.name.placeholder")}/>
                </label>
                <label>
                    <p>{t!("contact.email.label")}</p>
                    <input type="email" placeholder={t!("contact.email.placeholder")}/>
                </label>
                <label>
                    <p>{t!("contact.message.label")}</p>
                    <textarea placeholder={t!("contact.message.placeholder")}/>
                </label>
                <button type="submit">{t!("contact.send")}</button>
            </form>
        </div>
    }
}
