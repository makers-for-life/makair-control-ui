// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::sync::RwLock;

use fluent::bundle::FluentBundle;
use fluent::{FluentArgs, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;

pub struct LocaleAccessor {
    bundle: RwLock<FluentBundle<FluentResource, IntlLangMemoizer>>,
}

impl LocaleAccessor {
    pub fn new(bundle: FluentBundle<FluentResource, IntlLangMemoizer>) -> Self {
        LocaleAccessor {
            bundle: RwLock::new(bundle),
        }
    }

    pub fn replace(&self, bundle: FluentBundle<FluentResource, IntlLangMemoizer>) {
        *self.bundle.write().unwrap() = bundle;
    }

    pub fn t(&self, key: &str) -> String {
        self.process(key, None)
    }

    fn process(&self, key: &str, arguments: Option<&FluentArgs>) -> String {
        let bundle = self.bundle.read().unwrap();

        let message = bundle
            .get_message(key)
            .unwrap_or_else(|| panic!("locale key not found: {}", key));

        // Notice: return the key if the message has no value (eg. not yet translated)
        if let Some(pattern) = message.value() {
            let mut errors = vec![];

            let formatted = bundle.format_pattern(&pattern, arguments, &mut errors);

            // Any error? Panic
            if !errors.is_empty() {
                panic!("could not format pattern: {:?}", errors);
            }

            formatted.to_string()
        } else {
            key.to_string()
        }
    }
}
