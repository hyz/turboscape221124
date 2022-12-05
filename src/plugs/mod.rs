use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod reqwest;
pub fn reqwest<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("reqwest")
        .invoke_handler(tauri::generate_handler![reqwest::invoke])
        .setup(|_app| Ok(()))
        .build()
}
