use std::marker::PhantomData;

use serde::Serialize;
use serialize_to_javascript::{default_template, DefaultTemplate, Template};
use tauri::Assets;

#[derive(Template)]
#[default_template("scripts/forward.js")]
pub(crate) struct Forward {
    pub host: String,
    pub uuid: String,
    pub site_dir: String,
    pub index: String,
    pub index_bg: String,
}

#[derive(Template)]
#[default_template("scripts/wasm-bootstrap.js")]
pub(crate) struct WasmBootstrap {
    pub host: String,
    pub uuid: String,
}

#[derive(Template)]
#[default_template("scripts/search.baidu.com.js")]
pub(crate) struct SearchBaidu {
    pub hash_index: String,
    pub uuid: String,
}

#[derive(Template)]
#[default_template("scripts/search.qcc.com.js")]
pub(crate) struct SearchQcc {
    pub numb: i32,
    pub keyword: String,
    pub hash_index: String,
    pub uuid: String,
}

#[derive(Template)]
#[default_template("scripts/page-on-load.js")]
pub(crate) struct PageOnLoad;

#[derive(Template)]
#[default_template("../third-party/dist/ajaxhook.min.js")]
pub(crate) struct AjaxHook;
//{ pub(crate) isolation_origin: &'a str,}

#[derive(Template)]
#[default_template("../tauri-1.2.0/scripts/ipc.js")]
pub(crate) struct IpcJavascript<'a> {
    pub(crate) isolation_origin: &'a str,
}

#[derive(Template)]
#[default_template("../tauri-1.2.0/scripts/pattern.js")]
pub(crate) struct PatternJavascript {
    pub(crate) pattern: PatternObject,
}
#[derive(Debug, Clone)]
enum Pattern<A: Assets = tauri::utils::assets::EmbeddedAssets> {
    /// The brownfield pattern.
    Brownfield(PhantomData<A>),
}
/// The shape of the JavaScript Pattern config
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase", tag = "pattern")]
pub enum PatternObject {
    /// Brownfield pattern.
    Brownfield,
    /// Isolation pattern. Recommended for security purposes.
    #[cfg(feature = "isolation")]
    Isolation {
        /// Which `IsolationSide` this `PatternObject` is getting injected into
        side: IsolationSide,
    },
}

impl From<&Pattern> for PatternObject {
    fn from(pattern: &Pattern) -> Self {
        match pattern {
            Pattern::Brownfield(_) => Self::Brownfield,
            #[cfg(feature = "isolation")]
            Pattern::Isolation { .. } => Self::Isolation {
                side: IsolationSide::default(),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use serialize_to_javascript::{Options, Serialized};

    use super::*;
    #[test]
    fn check_patternobject_serde() {
        let raw_value = serde_json::value::to_raw_value(&PatternObject::Brownfield);
        let serialized = Serialized::new(&raw_value.unwrap(), &Options::default());
        dbg!(serialized.into_string());
    }
    #[test]
    fn check_pattern_init() {
        let pattern = &Pattern::Brownfield(PhantomData);
        let pattern_init = PatternJavascript {
            pattern: pattern.into(),
        }
        .render_default(&Default::default())
        .unwrap()
        .into_string();
        dbg!(pattern_init);
    }
    #[test]
    fn check_ipc() {
        use tauri::utils::assets::EmbeddedAssets;
        let ipc_init = IpcJavascript {
            isolation_origin: &match Pattern::Brownfield::<EmbeddedAssets>(PhantomData) {
                #[cfg(feature = "isolation")]
                Pattern::Isolation { schema, .. } => crate::pattern::format_real_schema(schema),
                _ => "_foo_________".to_string(),
            },
        }
        .render_default(&Default::default())
        .unwrap()
        .into_string();
        dbg!(ipc_init);
    }
}
// #[derive(Template)]
// #[default_template("../tauri-1.1.1/scripts/init.js")]
// struct InitJavascript<'a> {
//     origin: String,
//     #[raw]
//     pattern_script: &'a str,
//     #[raw]
//     ipc_script: &'a str,
//     #[raw]
//     bundle_script: &'a str,
//     // A function to immediately listen to an event.
//     #[raw]
//     listen_function: &'a str,
//     #[raw]
//     core_script: &'a str,
//     #[raw]
//     event_initialization_script: &'a str,
//     #[raw]
//     plugin_initialization_script: &'a str,
//     #[raw]
//     freeze_prototype: &'a str,
//     #[raw]
//     hotkeys: &'a str,
// }
// InitJavascript {
//     origin: "",
//     pattern_script,
//     ipc_script,
//     bundle_script,
//     listen_function: &format!(
//         "function listen(eventName, cb) {{ {} }}",
//         crate::event::listen_js(
//             self.event_listeners_object_name(),
//             "eventName".into(),
//             0,
//             None,
//             "window['_' + window.__TAURI__.transformCallback(cb) ]".into()
//         )
//     ),
//     core_script: include_str!("../scripts/core.js"),
//     event_initialization_script: &self.event_initialization_script(),
//     plugin_initialization_script,
//     freeze_prototype,
//     hotkeys,
// }
// .render_default(&Default::default())
// .map(|s| s.into_string())
// .map_err(Into::into)
