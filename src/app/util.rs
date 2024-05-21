use std::collections::HashMap;

use super::Language;

pub fn coalesce_translations(table: &str, lang: &Language) -> String {
    let available_langs = rust_i18n::available_locales!();

    format!(
        r#"
        CASE
            WHEN {table}.{lang} IS NULL THEN TRUE
            ELSE FALSE
        END AS {table}_translation_fallback,
        COALESCE({table}.{lang}{})
            "#,
        available_langs
            .into_iter()
            .fold(String::default(), |acc, ln| format!("{acc},{table}.{ln}"))
    )
}

pub fn transform_xata_image(url: &str, transform: HashMap<&'static str, &'static str>) -> String {
    let mut split = url.split(".xata.sh/");
    let base = split.next().unwrap();
    let id = split.next().unwrap();

    let transform = transform
        .into_iter()
        .fold(String::default(), |acc, (key, value)| {
            format!(
                "{acc}{}{key}={value}",
                if acc.is_empty() { "" } else { "," }
            )
        });

    format!("{base}.xata.sh/transform/{transform}/{id}")
}
