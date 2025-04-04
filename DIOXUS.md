# 🧬 I18N RS Dioxus Usage

Adding I18N RS to your project is simple:

1. Make sure your project is set up with **Dioxus**. Refer to the [Dioxus Getting Started Guide](https://dioxuslabs.com/learn/0.6/getting_started) for setup instructions.

1. Add the **i18nrs** library to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add i18nrs --features=dio
   ```

1. Import the `I18nProvider` component into your Dioxus application and wrap it around your app's main component to provide translations.

## 🛠️ Usage

Follow these steps to integrate i18nrs into your Dioxus application:

### Step 1: Import the Required Components

Import the `I18nProvider` and related types into your Dioxus project:

```rust
use dioxus::prelude::*;
use i18nrs::dioxus::I18nProvider;
use std::collections::HashMap;
```

### Step 2: Define Translations

Define your translations in a `HashMap` where keys are language codes (e.g., `en`, `fr`), and values are the translation strings in JSON format:

```rust
use dioxus::prelude::*;
use std::collections::HashMap;

fn app() -> Element {
    let translations = HashMap::from([
        ("en", r#"{"greeting": "Hello", "farewell": "Goodbye"}"#),
        ("fr", r#"{"greeting": "Bonjour", "farewell": "Au revoir"}"#),
    ]);

    rsx! {}
}
```

### Step 3: Wrap Your App with the `I18nProvider`

Wrap your main app component inside the `I18nProvider` to give it access to the internationalization context:

```rust
use dioxus::prelude::*;
use i18nrs::dioxus::I18nProvider;
use i18nrs::StorageType;
use std::collections::HashMap;

fn app() -> Element {
    let translations = HashMap::from([
        ("en", r#"{"greeting": "Hello", "farewell": "Goodbye"}"#),
        ("fr", r#"{"greeting": "Bonjour", "farewell": "Au revoir"}"#),
    ]);

    rsx! {
        I18nProvider {
            translations: translations,
            default_language: "en".to_string(),
            MainApp {}
        }
    }
}

#[component]
fn MainApp() -> Element {
    rsx! {
        h1 { "Welcome to i18nrs Dioxus Example!" }
    }
}
```

### Step 4: Access Translations via Context

Use the `use_context` hook to access the i18n instance and call translations inside your components:

```rust
use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
fn MainApp() -> Element {
    let I18nContext { i18n, set_language } = use_context::<I18nContext>();

    let greeting = i18n().t("greeting");

    rsx! {
        div {
            h1 { "{greeting}" }
            button {
                onclick: move |_| set_language.call("fr".to_string()),
                "Switch to French"
            }
        }
    }
}
```

## 🔧 Props

### `I18nProviderProps` Props

#### Main Props

| Property           | Type                                  | Description                                                                                        | Default        |
| ------------------ | ------------------------------------- | -------------------------------------------------------------------------------------------------- | -------------- |
| `translations`     | `HashMap<&'static str, &'static str>` | Mapping of language codes to translation JSON content. Defaults to an empty map.                   | `{}`           |
| `children`         | `Element`                             | Child components that will have access to the i18n context.                                        | **Required**   |
| `storage_type`     | `StorageType`                         | Type of browser storage for persisting the selected language (`LocalStorage` or `SessionStorage`). | `LocalStorage` |
| `storage_name`     | `String`                              | Key name in browser storage for saving the selected language.                                      | `"i18nrs"`     |
| `default_language` | `String`                              | Language to fall back to if none is found in storage.                                              | `"en"`         |

#### Behavioral Props

| Property   | Type               | Description                                                                                    | Default |
| ---------- | ------------------ | ---------------------------------------------------------------------------------------------- | ------- |
| `onchange` | `EventHandler<String>` | Callback triggered when the language is changed. Receives the new language code as a `String`. | No-op   |
| `onerror`  | `EventHandler<String>` | Callback triggered when an error occurs in the i18n process. Receives the error message.       | No-op   |

## 💡 Notes

1. **Translation Keys**: Use dot-separated keys to organize translations hierarchically, e.g., `menu.file.open`. Translation files use a JSON format and can include nested keys for better organization.

   - Example:

     ```json
     {
       "menu": {
         "file": {
           "open": "Open",
           "save": "Save"
         },
         "edit": "Edit"
       }
     }
     ```

1. **Language Switching**: Use the `set_language` callback from `I18nContext` to dynamically update the language and persist it using the specified storage type.

1. **Fallback Mechanism**: If a translation is not found for the current language, the default language is used.
