// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

fn main() {
    // This is only useful when compiling to a Windows target
    if std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_else(|_| "".to_owned()) == "windows"
        && std::env::var("DISABLE_WINRES").is_err()
    {
        const ICON_PATH: &str = "res/images/app-icon.ico";
        println!("cargo:rerun-if-changed={}", ICON_PATH);

        let features = std::env::vars_os()
            .flat_map(|(k, v)| {
                let var_name = k.to_string_lossy();
                match var_name.strip_prefix("CARGO_FEATURE_") {
                    Some(name) if v == "1" => Some(name.to_lowercase()),
                    _ => None,
                }
            })
            .collect::<Vec<_>>()
            .join(", ");

        let mut res = winres::WindowsResource::new();
        res.set_icon(ICON_PATH);
        res.set(
            "FileDescription",
            &format!(
                "{} Compiled with features: {}.",
                env!("CARGO_PKG_DESCRIPTION"),
                &features
            ),
        );
        res.compile()
            .expect("could not compile with Windows icon and metadata");
    }
}
