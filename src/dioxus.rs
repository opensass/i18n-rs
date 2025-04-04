#![doc = include_str!("../DIOXUS.md")]

use crate::config::{I18n, I18nConfig, StorageType};
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use std::collections::HashMap;
use web_sys::window;

/// Properties for the `I18nProvider` component.
///
/// This configuration struct allows you to specify supported translations,
/// storage preferences, and define callbacks for handling language changes and errors.
#[derive(Props, PartialEq, Clone)]
pub struct I18nProviderProps {
    /// The translations raw content.
    ///
    /// A map where keys are language codes (e.g. `"en"`, `"fr"`) and values are the corresponding translation strings or raw content.
    /// Defaults to an empty `HashMap` if not provided.
    #[props(default)]
    pub translations: HashMap<&'static str, &'static str>,

    /// The child components wrapped with the `I18n` context.
    ///
    /// These elements will have access to the internationalization features provided by the `I18nProvider`.
    pub children: Element,

    /// The type of browser storage to use.
    ///
    /// Determines how the selected language is persisted in the user's browser.
    /// Options typically include `StorageType::LocalStorage` or `StorageType::SessionStorage`.
    /// Defaults to `StorageType::LocalStorage`.
    #[props(default)]
    pub storage_type: StorageType,

    /// The key for storing the selected language.
    ///
    /// Used as the key in the browser storage system to persist the selected language.
    /// Defaults to `"i18nrs"`.
    #[props(default = "i18nrs".to_string())]
    pub storage_name: String,

    /// Default language if no language is found in storage.
    ///
    /// This will be used as the fallback language when there is no saved language in storage.
    /// Defaults to `"en"`.
    #[props(default = "en".to_string())]
    pub default_language: String,

    /// Callback when the language changes.
    ///
    /// Invoked whenever the language is updated.
    /// Receives the new language code as a `String`.
    #[props(default)]
    pub onchange: EventHandler<String>,

    /// Callback for handling errors.
    ///
    /// Triggered when an error occurs during the internationalization process.
    /// Receives an error message as a `String`.
    #[props(default)]
    pub onerror: EventHandler<String>,
}

/// The context provided to children by the `I18nProvider`.
///
/// Contains the current `I18n` instance and a method to change the language.
#[derive(Clone)]
pub struct I18nContext {
    /// Reactive signal containing the current internationalization state.
    pub i18n: Signal<I18n>,

    /// Function to change the current language.
    ///
    /// Triggers re-rendering of any components using the `i18n` signal.
    pub set_language: EventHandler<String>,
}

/// I18nProvider Component
///
/// A Dioxus component that provides internationalization (i18n) context to its child components.
/// It initializes translations, manages language persistence using browser storage, and provides
/// reactive access to the current language and i18n instance. It also handles text direction (RTL/LTR)
/// updates based on the selected language.
///
/// # Properties
/// The component accepts an `I18nProviderProps` struct with the following fields:
///
/// - **translations**: A map of language codes to raw translation content (`HashMap<&'static str, &'static str>`).
///   Used to initialize the i18n system. Defaults to an empty map.
/// - **children**: The nested elements that will have access to the i18n context (`Element`).
/// - **storage_type**: Specifies whether to use `LocalStorage` or `SessionStorage` for saving the selected language.
///   Defaults to `LocalStorage`.
/// - **storage_name**: The key used in browser storage to persist the selected language. Default: `"i18nrs"`.
/// - **default_language**: The fallback language if none is found in storage. Default: `"en"`.
/// - **onchange**: Callback invoked when the language changes. Receives the new language code as a `String`.
/// - **onerror**: Callback invoked when an error occurs (e.g., failed initialization or language switch).
///
/// # Features
/// - Loads and provides translations via a context.
/// - Dynamically updates the `dir` attribute of the HTML document based on RTL/LTR languages.
/// - Handles language change events and provides reactive updates to subscribers.
/// - Persists user-selected language in browser storage.
/// - Emits callbacks for changes and errors.
///
/// # Behavior
/// - Reads the initial language from browser storage using the configured `storage_type` and `storage_name`.
/// - Falls back to `default_language` if no language is stored.
/// - Initializes the i18n instance using provided translations.
/// - If the language cannot be set, the `onerror` callback is triggered with the error message.
/// - On language change:
///   - Updates the browser storage.
///   - Applies text direction (`dir="rtl"` or `dir="ltr"`) on the `<html>` element.
///   - Calls the `onchange` callback.
///   - Updates the context state.
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use i18nrs::dioxus::I18nProvider;
/// use i18nrs::StorageType;
/// use std::collections::HashMap;
///
/// fn app() -> Element {
///     let translations = HashMap::from([
///         ("en", r#"{"hello": "Hello!"}"#),
///         ("ar", r#"{"hello": "مرحبا!"}"#),
///     ]);
///
///     rsx! {
///         I18nProvider {
///             translations: translations,
///             storage_type: StorageType::LocalStorage,
///             storage_name: "my_i18n_key".to_string(),
///             default_language: "en".to_string(),
///             onchange: move |lang| log::info!("Language changed to {lang}"),
///             onerror: move |err| log::error!("i18n error: {err}"),
///             children: rsx! {
///                 div { "Hello, world!" }
///             }
///         }
///     }
/// }
/// ```
///
/// # Notes
/// - Right-to-left (RTL) languages like Arabic, Hebrew, Persian, and Urdu automatically set the HTML `dir` attribute.
/// - If initialization fails (e.g., missing or malformed translation data), the `onerror` callback is triggered.
/// - The `I18nContext` with `i18n` and `set_language` is made available via Dioxus's context API.
#[component]
pub fn I18nProvider(props: I18nProviderProps) -> Element {
    let initial_language = match props.storage_type {
        StorageType::LocalStorage => LocalStorage::get(&props.storage_name)
            .ok()
            .unwrap_or(Some(props.default_language.clone())),
        StorageType::SessionStorage => SessionStorage::get(&props.storage_name)
            .ok()
            .unwrap_or(Some(props.default_language.clone())),
    };

    fn is_rtl_language(lang: &str) -> bool {
        matches!(lang, "ar" | "he" | "fa" | "ur" | "ps" | "ku" | "sd")
    }

    let update_text_direction = |lang: &str| {
        if let Some(document) = window().and_then(|win| win.document()) {
            let dir = if is_rtl_language(lang) { "rtl" } else { "ltr" };
            if let Some(html_element) = document.document_element() {
                let _ = html_element.set_attribute("dir", dir);
            }
        }
    };

    update_text_direction(&initial_language.clone().unwrap_or_else(|| "en".to_string()));

    let mut i18n = use_signal(|| {
        I18n::new(
            I18nConfig {
                translations: props.translations.clone(),
            },
            props.translations.clone(),
        )
        .map(|mut instance| {
            if let Err(err) = instance.set_translation_language(
                &initial_language.clone().unwrap_or_default(),
                &props.storage_type,
                &props.storage_name,
            ) {
                props.onerror.call(err.clone());
            }
            instance
        })
        .unwrap_or_else(|err| {
            props.onerror.call(err.clone());
            panic!("Failed to initialize I18n: {}", err);
        })
    });

    let set_language = EventHandler::new({
        move |language: String| {
            let mut i18n_val = i18n();
            update_text_direction(&language);

            if i18n_val
                .set_translation_language(&language, &props.storage_type, &props.storage_name)
                .is_ok()
            {
                i18n.set(i18n_val);
                props.onchange.call(language);
            }
        }
    });
    let context = I18nContext {
        i18n,
        set_language,
    };

    use_context_provider(|| context);

    rsx! { {props.children} }
}
