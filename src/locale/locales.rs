// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

pub const LOCALE_DEFAULT: &str = "en";

pub const LOCALES: [&str; 10] = ["en", "fr", "de", "es", "it", "lv", "pt", "ru", "uk", "zh"];

pub fn locale_code_to_name(code: &str) -> String {
    let name = match code {
        "en" => Some("English"),
        "fr" => Some("Français"),
        "de" => Some("Deutsche"),
        "es" => Some("Español"),
        "it" => Some("Italiano"),
        "lv" => Some("Latviešu"),
        "pt" => Some("Português"),
        "ru" => Some("Русский"),
        "uk" => Some("Українська"),
        "zh" => Some("汉语"),
        _ => None,
    };

    name.map(|name| name.to_string())
        .unwrap_or_else(|| code.to_uppercase())
}
