#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
extern crate chrono;

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
mod menu;
mod monster_generated;
mod plugs;
mod protocols;
mod scripts;

#[derive(Default)]
pub struct Database(pub Arc<Mutex<HashMap<String, HashSet<i32>>>>);
//HashMap<String, >

// println!("Timestamp in local {:?}", chrono::offset::Local::today().format("%y%m%d"));

pub fn build() -> std::result::Result<tauri::App, tauri::Error> {
    tauri::Builder::default()
        //.manage(Connection(Default::default()))
        .manage(Database(Default::default()))
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
            let today = now.format("%y-%m-%d.%h");
            dbg!(format!("___ on_page_load {} {today}", payload.url()));

            if payload.url().contains("tauri.localhost") {
                // _ = window.eval(r#"document.location.href = 'https://baidu.com';"#);
                // window
                //     .emit("jedi", Some(json!({"goto":"https://baidu.com"})))
                //     .expect("emit joyful");
            }
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

            let window_ = window.clone();
            window.listen("rush", move |event| {
                dbg!(("rush ___", event.payload()));
                let payload = json!({"goto":"https://baidu.com"});
                window_.emit("jedi", Some(payload)).expect("emit-jedi");
            });

            window.open_devtools();

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
