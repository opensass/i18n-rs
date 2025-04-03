use crate::components::landing::Examples;
use i18nrs::yew::I18nProvider;
use i18nrs::StorageType;
use std::collections::HashMap;
use yew::prelude::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let translations = HashMap::from([
        ("en", include_str!("../../i18n/en/base.json")),
        ("es", include_str!("../../i18n/es/base.json")),
        ("fr", include_str!("../../i18n/fr/base.json")),
        ("ar", include_str!("../../i18n/ar/base.json")),
    ]);

    let onchange = Callback::from(|language: String| {
        log::info!("Language changed to: {}", language);
    });

    html! {
        <I18nProvider
            translations={translations}
            storage_type={StorageType::LocalStorage}
            storage_name={"i18nrs".to_string()}
            default_language={"en".to_string()}
            onchange={onchange}
        >
            <Examples />
        </I18nProvider>
    }
}
