#![doc = include_str!("../YEW.md")]

use crate::config::{I18n, I18nConfig, StorageType};
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use std::collections::HashMap;
use yew::prelude::*;

/// Properties for the `I18nProvider` component.
///
/// This configuration struct allows you to specify supported languages, translation paths, storage options,
/// and callbacks for language change or error handling.
#[derive(Debug, Clone, PartialEq, Properties, Default)]
pub struct I18nProviderConfig {
    /// List of supported languages.
    ///
    /// Defines the languages that the application supports for translations. Defaults to `["en", "fr"]` if not specified.
    #[prop_or(vec!["en", "fr"])]
    pub languages: Vec<&'static str>,

    /// The translations raw content.
    ///
    /// Specifies the mapping of language codes to file contents.
    /// Defaults to an empty `HashMap` if not provided.
    #[prop_or_default]
    pub translations: HashMap<&'static str, &'static str>,

    /// The child components to be wrapped with the `I18n` context.
    ///
    /// This property allows you to pass child components that will have access to the internationalization context.
    pub children: Html,

    /// The type of browser storage to use.
    ///
    /// Determines where the selected language is stored in the browser. Options include:
    /// - `StorageType::LocalStorage`: Use the browser's local storage.
    /// - `StorageType::SessionStorage`: Use the browser's session storage.
    ///
    /// Defaults to `StorageType::LocalStorage`.
    #[prop_or_default]
    pub storage_type: StorageType,

    /// The key for storing the selected language.
    ///
    /// This string represents the key used in the browser's storage to save the selected language.
    ///
    /// Defaults to `"i18nrs"`.
    #[prop_or("i18nrs".to_string())]
    pub storage_name: String,

    /// Default language if no language is found in storage.
    ///
    /// Specifies the fallback language that will be used if no language is set in the browser's storage.
    ///
    /// Defaults to `"en"`.
    #[prop_or("en".to_string())]
    pub default_language: String,

    /// Callback when the language changes.
    ///
    /// This callback is triggered whenever the language is changed. It receives the new language code as a `String`.
    #[prop_or_default]
    pub onchange: Callback<String>,

    /// Callback for handling errors.
    ///
    /// This callback is triggered whenever an error occurs in the internationalization process.
    /// It receives an error message as a `String`.
    #[prop_or_default]
    pub onerror: Callback<String>,
}

/// I18nProvider Component
///
/// A Yew component that provides internationalization (i18n) context to its child components.
/// This component manages the supported languages, translations, storage for the selected language,
/// and callbacks for language changes and error handling.
///
/// # Properties
/// The component uses the `I18nProviderConfig` struct for its properties. Key properties include:
///
/// - **languages**: A list of supported languages (`Vec<&'static str>`). Default: `["en", "fr"]`.
/// - **translations**: A mapping of language codes to raw translation content (`HashMap<&'static str, &'static str>`). Default: empty.
/// - **children**: The child components wrapped within the `I18nProvider` to access the i18n context (`Html`).
/// - **storage_type**: The type of browser storage for the selected language (`StorageType`). Options:
///   - `StorageType::LocalStorage`: Uses the browser's local storage (default).
///   - `StorageType::SessionStorage`: Uses the browser's session storage.
/// - **storage_name**: The key for storing the selected language in the browser's storage (`String`). Default: `"i18nrs"`.
/// - **default_language**: The fallback language if no language is found in storage (`String`). Default: `"en"`.
/// - **onchange**: An optional callback triggered when the language changes (`Option<Callback<String>>`).
/// - **onerror**: An optional callback triggered when an error occurs in the i18n process (`Option<Callback<String>>`).
///
/// # Features
/// - Provides i18n context with support for dynamic language switching.
/// - Manages translations and language selection using browser storage.
/// - Allows handling language change and error events via callbacks.
/// - Ensures fallback behavior with a default language.
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use yew::prelude::*;
/// use i18nrs::yew::I18nProvider;
/// use std::collections::HashMap;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let translations = HashMap::from([
///         ("en", r#"{"greeting": "Hello"}"#),
///         ("fr", r#"{"greeting": "Bonjour"}"#),
///     ]);
///
///     html! {
///         <I18nProvider
///             languages={vec!["en", "fr"]}
///             translations={translations}
///         >
///             <ChildComponent />
///         </I18nProvider>
///     }
/// }
///
/// #[function_component(ChildComponent)]
/// pub fn child_component() -> Html {
///     html! {
///         <div>{ "Content that supports i18n!" }</div>
///     }
/// }
/// ```
///
/// ## Handling Language Changes
/// ```rust
/// use yew::prelude::*;
/// use i18nrs::yew::I18nProvider;
/// use i18nrs::StorageType;
/// use std::collections::HashMap;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let translations = HashMap::from([
///         ("en", r#"{"greeting": "Hello"}"#),
///         ("fr", r#"{"greeting": "Bonjour"}"#),
///     ]);
///
///     let on_language_change = Callback::from(|language: String| {
///         log::info!("Language changed to: {}", language);
///     });
///
///     html! {
///         <I18nProvider
///             languages={vec!["en", "fr"]}
///             translations={translations}
///             onchange={on_language_change}
///         >
///             <ChildComponent />
///         </I18nProvider>
///     }
/// }
///
/// #[function_component(ChildComponent)]
/// pub fn child_component() -> Html {
///     html! {
///         <div>{ "Dynamic language change!" }</div>
///     }
/// }
/// ```
///
/// ## Custom Storage and Fallback Language
/// ```rust
/// use yew::prelude::*;
/// use i18nrs::yew::I18nProvider;
/// use i18nrs::StorageType;
/// use std::collections::HashMap;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let translations = HashMap::from([
///         ("en", r#"{"greeting": "Hello"}"#),
///         ("fr", r#"{"greeting": "Bonjour"}"#),
///     ]);
///
///     html! {
///         <I18nProvider
///             languages={vec!["en", "fr"]}
///             translations={translations}
///             storage_type={StorageType::SessionStorage}
///             storage_name={"custom_i18n_key".to_string()}
///             default_language={"fr".to_string()}
///         >
///             <ChildComponent />
///         </I18nProvider>
///     }
/// }
///
/// #[function_component(ChildComponent)]
/// pub fn child_component() -> Html {
///     html! {
///         <div>{ "Using custom storage and fallback language!" }</div>
///     }
/// }
/// ```
///
/// # Behavior
/// - Retrieves the selected language from browser storage based on the `storage_type` and `storage_name`.
/// - Uses the `default_language` if no language is found in storage.
/// - Initializes and provides the i18n context with translations and language selection capabilities.
/// - Emits the `onchange` callback when the language changes, passing the new language code.
/// - Emits the `onerror` callback in case of initialization or runtime errors.
///
/// # Notes
/// - The `children` property wraps the components that will have access to the i18n context.
/// - If a translation error occurs, the `onerror` callback (if provided) is triggered with the error message.
/// - The `set_language` callback is available via context to dynamically change the selected language.
#[function_component(I18nProvider)]
pub fn i18n_provider(props: &I18nProviderConfig) -> Html {
    let initial_language = match props.storage_type {
        StorageType::LocalStorage => LocalStorage::get(&props.storage_name)
            .ok()
            .unwrap_or_else(|| Some(props.default_language.clone())),
        StorageType::SessionStorage => SessionStorage::get(&props.storage_name)
            .ok()
            .unwrap_or_else(|| Some(props.default_language.clone())),
    };

    let i18n = I18n::new(
        I18nConfig {
            languages: props.languages.clone(),
            translations: props.translations.clone(),
        },
        props.translations.clone(),
    )
    .map(|mut instance| {
        instance
            .set_translation_language(
                &initial_language.unwrap_or_default(),
                &props.storage_type,
                &props.storage_name,
            )
            .unwrap_or_else(|err| {
                props.onerror.emit(err);
            });
        instance
    })
    .unwrap_or_else(|err| {
        props.onerror.emit(err.clone());
        panic!("Failed to initialize I18n: {}", err);
    });

    let ctx = use_state(|| i18n);

    let onchange = props.onchange.clone();
    let storage_type = props.storage_type.clone();
    let storage_name = props.storage_name.clone();

    let set_language = {
        let ctx = ctx.clone();
        Callback::from(move |language: String| {
            let mut i18n = (*ctx).clone();
            if i18n
                .set_translation_language(&language, &storage_type, &storage_name)
                .is_ok()
            {
                ctx.set(i18n);

                onchange.emit(language);
            }
        })
    };

    html! {
        <ContextProvider<I18n> context={(*ctx).clone()}>
            <ContextProvider<Callback<String>> context={set_language}>
                { props.children.clone() }
            </ContextProvider<Callback<String>>>
        </ContextProvider<I18n>>
    }
}

#[hook]
pub fn use_translation() -> (I18n, Callback<String>) {
    let i18n = use_context::<I18n>().expect("No I18n context provided");
    let set_language = use_context::<Callback<String>>().expect("No set_language context found");
    (i18n, set_language)
}
