use dotenv::dotenv;
use gtk::{
    glib,
    prelude::{ApplicationExt, ApplicationExtManual},
    Application,
};
use tracing::metadata::LevelFilter;
use tracing_subscriber::{fmt, prelude::*};
use ui::App;

pub mod ui;

#[tokio::main]
async fn main() -> glib::ExitCode {
    dotenv().ok();

    let fmt_layer = fmt::layer();
    let filter_layer = tracing_subscriber::filter::Targets::new()
        .with_default(LevelFilter::INFO)
        .with_target("matrix_sdk_ui", LevelFilter::ERROR)
        .with_target("matrix_sdk_crypto", LevelFilter::ERROR)
        .with_target("purpur", LevelFilter::DEBUG);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    let gtk_app = Application::builder()
        .application_id("dev.blusk.purpur")
        .build();

    gtk_app.connect_activate(move |x| {
        let mut app = App::new();
        app.build_ui(x)
    });

    gtk_app.run()
}
