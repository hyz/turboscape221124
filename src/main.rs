use std::env::current_dir;
use tauri::RunEvent;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber_init().unwrap();
    tracing::debug!("{:?}", std::env::current_dir().unwrap());
    tracing::info!("{:?}", chrono::offset::Local::now().format("%y%m%d %F"));

    netscape::build()
        .expect("netscape::build")
        .run(|_app, event| {
            if let RunEvent::WindowEvent { label, event, .. } = event {
                println!("{} {:?}", label, event);
            }
        });
    Ok(())
}

fn tracing_subscriber_init() -> Result<(), Box<dyn std::error::Error>> {
    use tracing_subscriber::{fmt::format::*, prelude::*, EnvFilter};
    // use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
    // use tracing_subscriber::fmt::format::FmtSpan;

    // let default_directive = "myapp=debug".parse()?;
    let env_filter = EnvFilter::builder()
        .with_default_directive(tracing::metadata::LevelFilter::ERROR.into())
        .from_env()?;
    //.add_directive("axum_serve_tauri=trace".parse()?);
    //.add_directive("my_crate::module=trace".parse()?)
    //.add_directive("my_crate::my_other_module::something=info".parse()?);

    tracing_subscriber::fmt() // ::registry()
        .with_env_filter(env_filter)
        // .with(tracing_subscriber::fmt::layer())
        .with_span_events(FmtSpan::FULL)
        .init();
    Ok(())
}
