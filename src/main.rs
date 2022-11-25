use std::env::current_dir;
use tauri::RunEvent;

fn main() {
    dbg!(current_dir().unwrap()); //___//

    netscape::build()
        .expect("error while build app")
        .run(|_app, event| {
            if let RunEvent::WindowEvent { label, event, .. } = event {
                println!("{} {:?}", label, event);
            }
        });
}
