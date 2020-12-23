// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

gen_locale_impls!(
    default: En,

    {
        En -> ["en", "English", true],
        Fr -> ["fr", "Français", true],
        De -> ["de", "Deutsche", true],
        Es -> ["es", "Español", true],
        It -> ["it", "Italiano", true],
        Lv -> ["lv", "Latviešu", true],
        Pt -> ["pt", "Português", true],
        Ru -> ["ru", "Русский", true],
        Uk -> ["uk", "Українська", true],
        Zh -> ["zh", "汉语", cfg!(feature = "fonts-cjk")]
    }
);
