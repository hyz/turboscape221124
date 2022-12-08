#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
extern crate chrono;

use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serialize_to_javascript::DefaultTemplate;
use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};
use tauri::{Manager, State};

mod commands;
mod csapi;
mod menu;
mod plugs;
mod protocols;
mod scripts;
mod watch;

#[derive(Default)]
pub struct Database(pub Arc<Mutex<HashMap<String, HashSet<i32>>>>);
//HashMap<String, >

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("signal::ctrl_c");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("SignalKind::terminate")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::debug!("signal received, starting graceful shutdown");
}

use tokio::sync::broadcast;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "arm")]
enum Trigger {
    OpenDevtools,
    Location { href: Url },
    WebsocketChat { url: Url },
}
const EV_TRIGGER: &str = "ev_trigger";
struct TriggerState {
    users: Mutex<HashSet<String>>,
    broadcast: broadcast::Sender<String>,
}
impl TriggerState {
    pub(crate) fn new() -> Self {
        let users = Mutex::new(HashSet::new());
        let (broadcast, _rx) = broadcast::channel(100);
        Self { users, broadcast }
    }
}

enum ScriptTrigger {
    Script(String, Vec<u8>),
    ScriptUrl(String),
    Trigger(Trigger),
}

pub fn build() -> std::result::Result<tauri::App, tauri::Error> {
    tauri::Builder::default()
        //.manage(Connection(Default::default()))
        .manage(Database(Default::default()))
        .manage(TriggerState::new())
        .invoke_handler(tauri::generate_handler![
            commands::sample,
            commands::numb_exists,
            commands::greet,
            commands::monster,
            commands::ask,
            commands::help,
            protocols::video_uri
        ])
        .register_uri_scheme_protocol("bat", protocols::bat_protocol)
        .on_page_load(move |window, payload| {
            _ = commands::on_page_load(payload.url(), &window);

            let now = chrono::offset::Local::now();
            let today = now.format("%y%m%d.%h %F");
            tracing::debug!("on_page_load {today} {} {}", window.label(), payload.url());

            // if payload.url().contains("tauri.localhost") {
            //     // _ = window.eval(r#"document.location.href = 'https://baidu.com';"#);
            //     // window.emit("jedi", Some(json!({"goto":"https://baidu.com"}))).expect("emit joyful");
            // }

            // await __TAURI__.event.emit('ev_trigger', {arm:"Location",href:"https://qq.com"})
            // await __TAURI__.event.emit('ev_trigger', {arm:"WebsocketChat",url:"ws://u3000.de/websocket"})
            window.listen(EV_TRIGGER, {
                let win_ = window.clone();
                move |event| {
                    let app_ = win_.app_handle();
                    tracing::debug!("{EV_TRIGGER} ___ {:?}", event.payload());
                    let state = win_.state::<TriggerState>();

                    if let Some(payload) = event.payload() {
                        match serde_json::from_str::<Trigger>(payload) {
                            Ok(Trigger::OpenDevtools) => {
                                win_.open_devtools();
                            }
                            Ok(Trigger::Location { href }) => {
                                let js = format!("document.location.href = {};", json!(href));
                                _ = win_.eval(&js);
                            }
                            Ok(Trigger::WebsocketChat { url }) => {
                                // let url = match url::Url::parse(&url) {
                                //     Err(err) => {
                                //         tracing::error!("Url::parse {}: {:?}", url, err);
                                //         return;
                                //     }
                                //     Ok(url) => url,
                                // };
                                let sub_rx = state.broadcast.subscribe();
                                tauri::async_runtime::spawn(crate::csapi::ws_chat_test(
                                    url, sub_rx, app_,
                                ));
                            }
                            Err(err) => tracing::error!("{EV_TRIGGER} {}: {:?}", payload, err),
                        }

                        // let payload = format!("console.log('{}')", serde_json::json!(payload));
                        // win_.emit("console", Some(payload)).expect("emit-console");
                    }
                }
            });
        })
        .setup(|app| {
            app.wry_plugin(tauri_egui::EguiPluginBuilder::new(app.handle()));
            let window = app.get_window("main").unwrap();
            //window.manage(state)
            //window.app_handle();
            //window.emit(event, payload)//window.listen(event, handler)
            //window.run_on_main_thread(|| ()).unwrap();
            //window.with_webview(f)
            menu::create_tray(app)?;
            //create_egui_window(app.state::<tauri_egui::EguiPluginHandle>()).unwrap();

            window.listen("rush", {
                let win_ = window.clone();
                move |event| {
                    tracing::debug!("rush ___ {:?}", event.payload());
                    // let payload = { "location.href='https://baidu.com'" };
                    if let Some(payload) = event.payload() {
                        let payload = format!("console.log('{}')", serde_json::json!(payload));
                        win_.emit("jedi", Some(payload)).expect("emit-jedi");
                    }
                }
            });

            #[cfg(debug_assertions)]
            tauri::async_runtime::spawn({
                let win_ = window.clone();
                async move {
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    let trigger = serde_json::to_string(&Trigger::OpenDevtools).unwrap();
                    win_.trigger(EV_TRIGGER, Some(trigger));
                }
            });

            // tauri::async_runtime::spawn(async move {
            //     tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            //     window.emit("jedi", Some(json!(1))).expect("emit-jedi");

            //     // let (mut rx, mut child) = tauri::api::process::Command::new_sidecar("app")
            //     //     .expect("setup `app` sidecar")
            //     //     .spawn()
            //     //     .expect("spawn packaged node");
            //     //
            //     // let mut i = 0;
            //     // while let Some(event) = rx.recv().await {
            //     //     if let tauri::api::process::CommandEvent::Stdout(line) = event {
            //     //         window
            //     //             .emit("jedi", Some(format!("'{}'", line)))
            //     //             .expect("emit-jedi");
            //     //         i += 1;
            //     //         if i == 4 {
            //     //             child.write("message from Rust\n".as_bytes()).unwrap();
            //     //             i = 0;
            //     //         }
            //     //     }
            //     // }
            // });

            Ok(())
        })
        .menu(menu::build_menu())
        .on_menu_event(menu::menu_event_handler)
        .plugin(plugs::reqwest())
        .build(tauri::generate_context!("./tauri.conf.json"))
}

#[tauri::command]
async fn open_native_window(
    egui_handle: State<'_, tauri_egui::EguiPluginHandle>,
) -> Result<(), ()> {
    menu::create_egui_window(egui_handle)
}

// "baidu.com" | "qq.com" | "jd.com" =>
//     let js = format!(r#"document.location.href = "https://{}";"#, id);
//                         r#"
//   console.log(">>>xstep#{xix:02}<<<");
//   news = document.createElement('script');
//   news.type = 'text/javascript';
//   news.src = '//custom.protocol.tauri_localhost/_{xix:02}__.js';
//   news.async = true;
//   document.head.appendChild(news);
//   "#,
//jsenv_reset(window);
//         let js = format!(
//             r#"
//   var xsold = document.getElementById("_00__");
//   if (xsold) {{
//     xsold.parentNode.removeChild(xsold);
//   }}
//   var xs = document.createElement('script');
//   xs.id = "_00__";
//   xs.type = 'text/javascript';
//   xs.src = '//bat_localhost/_00__.js';
//   xs.async = true;
//   document.head.appendChild(xs);
//   console.log(`<script src=${{xs.src}} ... />`);
//   "#,

// returns the scheme and the path of the video file
// we're using this just to allow using the custom `bat` protocol or tauri built-in `asset` protocol

// fs::OpenOptions::new().write(true).create_new(true).open(&path)

//let f = (k,v)=> ((typeof v ==='string') ? v.replace('<em>','').replace('</em>','') : v);
//let r = JSON.parse(response.response, f)
//script.src="http://localhost:5000/dist/ajaxhook.min.js"
