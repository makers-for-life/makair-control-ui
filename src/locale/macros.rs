// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

macro_rules! gen_locale_impls {
    (default: $default:tt, { $($identifier:tt -> [$code:expr, $name:expr, $available: expr]),+ }) => {
        #[derive(PartialEq, Debug, Clone)]
        pub enum LocaleCode {
            $(
                $identifier,
            )+
        }

        impl LocaleCode {
            pub fn list_available() -> Vec<Self> {
                let mut locales = vec![];

                $(
                    if $available {
                        locales.push(Self::$identifier);
                    }
                )+

                locales
            }

            pub fn from_code(code: &str) -> Option<Self> {
                match code {
                    $(
                        $code => Some(Self::$identifier),
                    )+
                    _ => None,
                }
            }

            pub fn to_code(&self) -> &str {
                match self {
                    $(
                        Self::$identifier => $code,
                    )+
                }
            }

            pub fn to_name(&self) -> &str {
                match self {
                    $(
                        Self::$identifier => $name,
                    )+
                }
            }
        }

        impl Default for LocaleCode {
            fn default() -> Self {
                Self::$default
            }
        }
    }
}
