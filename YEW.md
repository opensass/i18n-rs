# Y i18nrs Yew Usage

i18nrs' Yew integration targets Yew 0.22.0 and later.

Adding **i18nrs** to your Yew project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the **i18nrs** library to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add i18nrs --features=yew
   ```

1. Import the `I18nProvider` component into your Yew application and wrap it around your app's main component to provide translations.

## 🛠️ Usage

There exists two ways to integrate `i18n-rs` into your Yew application:

### A. Function Based

Follow these steps to integrate i18nrs into your Yew application:

#### Step 1: Import the Required Components

Import the `I18nProvider` and any related types into your Yew project:

```rust
use yew::prelude::*;
use i18nrs::yew::I18nProvider;
use i18nrs::yew::I18nProviderConfig;
use std::collections::HashMap;
```

#### Step 2: Define Translations

Define your translations in a `HashMap` where keys are language codes (e.g., `en`, `fr`), and values are the translation strings in JSON format:

```rust
use yew::prelude::*;
use std::collections::HashMap;

#[function_component(App)]
pub fn app() -> Html {
    let translations = HashMap::from([
        ("en", r#"{"greeting": "Hello", "farewell": "Goodbye"}"#),
        ("fr", r#"{"greeting": "Bonjour", "farewell": "Au revoir"}"#),
    ]);
    html! {
    }
}
```

#### Step 3: Wrap Your App with the `I18nProvider`

Wrap your main app component inside the `I18nProvider` to give it access to the internationalization context:

```rust
use yew::prelude::*;
use i18nrs::yew::I18nProvider;
use i18nrs::yew::I18nProviderConfig;
use std::collections::HashMap;

#[function_component(App)]
pub fn app() -> Html {
    let translations = HashMap::from([
        ("en", r#"{"greeting": "Hello", "farewell": "Goodbye"}"#),
        ("fr", r#"{"greeting": "Bonjour", "farewell": "Au revoir"}"#),
    ]);

    let config = I18nProviderConfig {
        translations: translations,
        default_language: "en".to_string(),
        ..Default::default()
    };

    html! {
        <I18nProvider ..config>
            <MainApp />
        </I18nProvider>
    }
}

#[function_component(MainApp)]
pub fn main_app() -> Html {
    html! {
        <h1>{ "Welcome to i18nrs Example!" }</h1>
    }
}

fn main() {
    // yew::Renderer::<App>::new().render();
}
```

#### Step 4: Access Translations with the `use_translation` Hook

Use the `use_translation` hook to access translation functions within your components:

```rust
use yew::prelude::*;
use i18nrs::yew::use_translation;

#[function_component(MainApp)]
pub fn main_app() -> Html {
    let (i18n, set_language) = use_translation();

    let greeting = i18n.t("greeting"); // Retrieves translation for key "greeting"

    html! {
        <div>
            <h1>{ greeting }</h1>
            <button onclick={Callback::from(move |_| set_language.emit("fr".to_string()))}>
                { "Switch to French" }
            </button>
        </div>
    }
}

fn main() {
    // yew::Renderer::<MainApp>::new().render();
}
```

### B. Component Based

See the complete example in [struct_components.rs](examples/yew/src/pages/struct_components.rs).
Follow these steps to integrate i18nrs into your Yew component-based application:

#### Step 1: Define main struct-component

Use a struct-component for the entry-point of the application. Configure some languages via `I18nProviderConfig` and use a `I18nProvider`:

```rust
use i18nrs::yew::use_translation;
use i18nrs::yew::I18nProvider;
use i18nrs::yew::I18nProviderConfig;
use std::collections::HashMap;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct StructComponents {}

pub struct StructComponentsMsg {}
#[derive(Properties, PartialEq, Default)]
pub struct StructComponentsProps {}

impl Component for StructComponents {
    type Message = StructComponentsMsg;
    type Properties = StructComponentsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let translations = HashMap::from([
            ("en", r#"{"greeting": "Hello", "farewell": "Goodbye"}"#),
            ("fr", r#"{"greeting": "Bonjour", "farewell": "Au revoir"}"#),
        ]);

        let config = I18nProviderConfig {
            translations,
            default_language: "en".to_string(),
            ..Default::default()
        };

        html! {
            <I18nProvider ..config>
                <FMainStructComponents />
            </I18nProvider>
        }
    }
}

fn main() {
    yew::Renderer::<StructComponents>::new().render();
}
```

#### Step 2: Define a wrapper function-component

Define a function-component to wrap your final struct-component. This is needed because you cannot call `use_translation` from a struct-component directly:

```rust
#[yew::function_component(FMainStructComponents)]
pub fn f_main_struct_components() -> yew::Html {
    let (i18n, set_language) = use_translation();
    yew::html! {<MainStructComponents i18n={i18n} set_language={set_language} />}
}
```

#### Step 3: Define the final struct-component

Define the final struct-component. Use `MainProps` to propagate the `i18n` and `set_language` properties from the function-component to the struct-component:

```rust
pub struct MainStructComponents {}

pub struct MainMsg {}
#[derive(Properties, PartialEq)]
pub struct MainProps {
    i18n: i18nrs::I18n,
    set_language: Callback<String>,
}

impl Component for MainStructComponents {
    type Message = MainMsg;
    type Properties = MainProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let greeting = ctx.props().i18n.t("greeting"); // Retrieves translation for key "greeting"
        let language = (&ctx.props().set_language).clone();
        let callback = Callback::from(move |e: MouseEvent| {
            let id = JsCast::unchecked_into::<HtmlInputElement>(e.target().unwrap()).id();
            language.emit(id.to_string())
        });

        html! {
            <div>
                <h1>{ greeting }</h1>
                <button id={"fr"} onclick={&callback}>
                    { "Switch to French" }
                </button>
                <button id={"en"} onclick={&callback}>
                    { "Switch to English" }
                </button>
            </div>
        }
    }
}
```

## 🔧 Props

### `I18nProviderConfig` Props

#### Main Props

| Property           | Type                                  | Description                                                                                        | Default        |
| ------------------ | ------------------------------------- | -------------------------------------------------------------------------------------------------- | -------------- |
| `languages`        | `Vec<&'static str>`                   | List of supported languages.                                                                       | `["en", "fr"]` |
| `translations`     | `HashMap<&'static str, &'static str>` | Mapping of language codes to translation JSON content. Defaults to an empty map.                   | `{}`           |
| `children`         | `Html`                                | Child components that will have access to the i18n context.                                        | **Required**   |
| `storage_type`     | `StorageType`                         | Type of browser storage for persisting the selected language (`LocalStorage` or `SessionStorage`). | `LocalStorage` |
| `storage_name`     | `String`                              | Key name in browser storage for saving the selected language.                                      | `"i18nrs"`     |
| `default_language` | `String`                              | Language to fall back to if none is found in storage.                                              | `"en"`         |

#### Behavioral Props

| Property   | Type               | Description                                                                                    | Default |
| ---------- | ------------------ | ---------------------------------------------------------------------------------------------- | ------- |
| `onchange` | `Callback<String>` | Callback triggered when the language is changed. Receives the new language code as a `String`. | No-op   |
| `onerror`  | `Callback<String>` | Callback triggered when an error occurs in the i18n process. Receives the error message.       | No-op   |

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

1. **Language Switching**: The `set_language` callback dynamically updates the language and persists it using the specified storage type.
1. **Fallback Mechanism**: If a translation is not found for the current language, the default language is used.
