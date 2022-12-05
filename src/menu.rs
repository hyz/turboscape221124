use serialize_to_javascript::DefaultTemplate;
use tauri::{
    CustomMenuItem, Manager, Menu, MenuItem, State, Submenu, SystemTray, SystemTrayEvent,
    SystemTrayMenu, WindowBuilder, WindowMenuEvent, WindowUrl,
};
use tauri_egui::eframe;

const SCRAPING_STOP: &str = r#"(function (){
    window.__page_.scraping = false;
    window.__page_.uuid = undefined;
})();"#;

const SCRAPING_QCC: &str = r#"(function (selector){
window.__page_.scraping = true;
let lie = document.querySelector(selector);
if (lie && lie.nextElementSibling) {
    let a = lie.nextElementSibling.querySelector("a") || lie.nextElementSibling;
    a.click();
}
})("div.adsearch-list > nav > ul > li.active");"#;

const SCRAPING_BAIDU: &str = r#"(function (selector){
window.__page_.scraping = true;
let lie = document.querySelector(selector);
if (lie && lie.nextElementSibling) {
    let a = lie.nextElementSibling.querySelector("a") || lie.nextElementSibling;
    a.click();
}
})('#page > div > strong');"#;

const VIDEO_PLAY: &str = r#";(function play() {
const { invoke, convertFileSrc } = window.__TAURI__.tauri;
invoke("video_uri").then(([scheme, path]) => {
    const div = document.createElement("div");
    const source = document.createElement("source");
    source.type = "video/mp4";
    source.src = convertFileSrc(path, scheme);
    console.log(`${scheme} ${path}`, source.src);
    const video = document.createElement("video"); // document.getElementById("video_source");
    video.autoplay = true;
    video.controls = true;
    video.name = "media";
    video.appendChild(source);
    div.append(video);
    document.body.append(div);
    video.load();
});
//console.log("log X");
//setTimeout(play, 9000);
})()"#;

const IMG_COLORFUL_ANT: &str = r#";(function () {
    let { convertFileSrc } = window.__TAURI__.tauri;
    let div = document.createElement("div");
    let img = document.createElement("img");
    img.src = convertFileSrc('colorful-ant.jpg', 'asset');
    div.append(img);
})()"#;

pub fn build_menu() -> Menu {
    #[allow(unused_mut)]
    let mut disable_item =
        CustomMenuItem::new("disable-menu", "Disable menu").accelerator("CmdOrControl+D");
    #[allow(unused_mut)]
    let mut test_item = CustomMenuItem::new("test", "Test").accelerator("CmdOrControl+T");
    #[cfg(target_os = "macos")]
    {
        disable_item = disable_item.native_image(tauri::NativeImage::MenuOnState);
        test_item = test_item.native_image(tauri::NativeImage::Add);
    }

    // create a submenu
    let my_sub_menu = Menu::new().add_item(disable_item);

    let my_app_menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_submenu(Submenu::new("Sub menu", my_sub_menu));

    let test_menu = Menu::new()
        .add_item(CustomMenuItem::new("STOP_", "Stop"))
        .add_item(CustomMenuItem::new("BAIDU_", "Auto baidu"))
        .add_item(CustomMenuItem::new("QCC_", "Auto qcc"))
        .add_native_item(MenuItem::Separator)
        .add_item(CustomMenuItem::new("map.baidu.com", "map.baidu.com"))
        .add_item(CustomMenuItem::new("www.qcc.com", "www.qcc.com"))
        .add_item(CustomMenuItem::new("baidu.com", "www.baidu.com"))
        .add_item(CustomMenuItem::new("www.jd.com", "www.jd.com"))
        .add_item(CustomMenuItem::new("juejin.cn", "www.juejin.cn"))
        .add_item(CustomMenuItem::new("tauri.localhost", "tauri.localhost"))
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Separator)
        .add_item(CustomMenuItem::new("greet", "Greet"))
        .add_item(CustomMenuItem::new("video-test", "VideoTest"))
        .add_item(CustomMenuItem::new("colorful-ant", "Colorful-Ant"))
        .add_item(test_item)
        .add_item(CustomMenuItem::new("egui-ex", "Egui"))
        .add_item(CustomMenuItem::new("selected/disabled", "NonOp"));

    // add all our childs to the menu (order is how they'll appear)
    Menu::new()
        .add_submenu(Submenu::new("App", my_app_menu))
        .add_submenu(Submenu::new("Select-X", test_menu))
}

pub fn menu_event_handler(event: WindowMenuEvent) {
    let window = event.window();
    match event.menu_item_id() {
        "STOP_" => {
            _ = dbg!(window.eval(SCRAPING_STOP));
        }
        "BAIDU_" => {
            _ = dbg!(window.eval(SCRAPING_BAIDU));
        }
        "QCC_" => {
            _ = dbg!(window.eval(SCRAPING_QCC));
        }

        host @ ("map.baidu.com" | "www.qcc.com" | "baidu.com" | "juejin.cn" | "www.jd.com"
        | "tauri.localhost") => {
            let js = format!(r#"document.location.href = 'https://{host}';"#);
            _ = dbg!(window.eval(js.as_ref()));
            // // open in browser (requires the `shell-open-api` feature)
            // api::shell::open(&window.shell_scope(), "https://baidu.com".into(), None, ).unwrap();
        }
        "egui-ex" => {
            let app = window.app_handle();
            create_egui_window(app.state::<tauri_egui::EguiPluginHandle>()).unwrap();
        }
        "greet" => {
            use crate::scripts::WasmBootstrap as Template;
            let host = "www.greet".into();
            let uuid = uuid::Uuid::new_v4().to_string();
            _ = dbg!(eval_template(window, Template { host, uuid }));
        }
        "test" => {
            let js = "console.log('test-eval')";
            _ = dbg!(window.eval(js));
        }
        "video-test" => {
            _ = dbg!(window.eval(VIDEO_PLAY));
        }
        "colorful-ant" => {
            _ = dbg!(window.eval(IMG_COLORFUL_ANT));
        }

        id => {
            println!("got menu event: {}", id);
        }
    }
}

fn eval_template<T: DefaultTemplate>(window: &tauri::Window, tpl: T) -> tauri::Result<()> {
    let js = tpl
        .render_default(&Default::default())
        .unwrap()
        .into_string();
    // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    // window.emit("jedi", Some(js)).expect("emit-jedi");
    window.eval(&js)
}

pub fn create_egui_window(egui_handle: State<'_, tauri_egui::EguiPluginHandle>) -> Result<(), ()> {
    //let (egui_app, rx) = Layout::new();
    let native_options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some([800.0, 600.0].into()),
        ..Default::default()
    };
    let _window = egui_handle
        .create_window(
            "egui-window".to_string(),
            Box::new(|_cc| Box::new(BasicApp::default())),
            //Box::new(|cc| Box::new(egui_demo_app::WrapApp::new(cc))),
            "Login".into(),
            native_options,
        )
        .unwrap();
    Ok(())
}

#[derive(Default)]
struct BasicApp {}

impl tauri_egui::eframe::App for BasicApp {
    fn update(&mut self, ctx: &tauri_egui::egui::Context, _frame: &mut eframe::Frame) {
        tauri_egui::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Basic App");
            let sign_in = ui.button("Sign in");
            if sign_in.clicked() {
                tracing::info!("Sign in clicked");
            }
        });
    }
}

pub fn create_tray(app: &tauri::App) -> tauri::Result<()> {
    // .system_tray(
    //     SystemTray::new().with_menu(
    //         SystemTrayMenu::new()
    //             .add_item(CustomMenuItem::new("exit", "Exit window"))
    //             .add_item(CustomMenuItem::new("new", "New window"))
    //             .add_item(CustomMenuItem::new("toggle", "Toggle"))
    //             .add_item(CustomMenuItem::new("baidu.com", "baidu.com"))
    //             .add_item(CustomMenuItem::new("qq.com", "qq.com"))
    //             .add_item(CustomMenuItem::new("jd.com", "jd.com"))
    //             .add_item(CustomMenuItem::new("_invoke.js", "_invoke.js"))
    //             .add_item(CustomMenuItem::new("xstep", "xstep#01")), //.add_item(CustomMenuItem::new("task00", "task00")),
    //     ),
    // )
    // .on_system_tray_event(on_system_tray_event)

    let mut tray_menu1 = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("toggle", "Toggle"))
        .add_item(CustomMenuItem::new("new", "New window"))
        .add_item(CustomMenuItem::new("icon_1", "Tray Icon 1"))
        .add_item(CustomMenuItem::new("icon_2", "Tray Icon 2"));

    #[cfg(target_os = "macos")]
    {
        tray_menu1 = tray_menu1.add_item(CustomMenuItem::new("set_title", "Set Title"));
    }

    tray_menu1 = tray_menu1
        .add_item(CustomMenuItem::new("switch_menu", "Switch Menu"))
        .add_item(CustomMenuItem::new("exit_app", "Quit"))
        .add_item(CustomMenuItem::new("destroy", "Destroy"));

    let tray_menu2 = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("toggle", "Toggle"))
        .add_item(CustomMenuItem::new("new", "New window"))
        .add_item(CustomMenuItem::new("exit_app", "Quit"))
        .add_item(CustomMenuItem::new("destroy", "Destroy"));
    let is_menu1 = std::sync::atomic::AtomicBool::new(true);

    let tray_id = "bat-tray"; //.to_string();
    let handle = app.handle();
    let on_event = move |event: SystemTrayEvent| {
        let tray_handle = handle.tray_handle_by_id(&tray_id).unwrap();
        match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = handle.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = tray_handle.get_item(&id);
                match id.as_str() {
                    "exit_app" => {
                        // exit the app
                        handle.exit(0);
                    }
                    "destroy" => {
                        tray_handle.destroy().unwrap();
                    }
                    "toggle" => {
                        let window = handle.get_window("main").unwrap();
                        let new_title = if window.is_visible().unwrap() {
                            window.hide().unwrap();
                            "Show"
                        } else {
                            window.show().unwrap();
                            "Hide"
                        };
                        item_handle.set_title(new_title).unwrap();
                    }
                    "new" => {
                        WindowBuilder::new(&handle, "new", WindowUrl::App("index.html".into()))
                            .title("Tauri")
                            .build()
                            .unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    };
    SystemTray::new()
        .with_id(tray_id.to_string())
        .with_menu(tray_menu1.clone())
        .on_event(on_event)
        .build(app)
        .map(|_| ())
}
