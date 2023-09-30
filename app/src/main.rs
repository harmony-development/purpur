use dotenv::dotenv;
use gtk::{
    gio,
    glib::{self, clone, MainContext, Priority},
    prelude::*,
    Application, ApplicationWindow, Label, ListItem, ListView, NoSelection, PolicyType,
    ScrolledWindow, SignalListItemFactory, StringList, StringObject,
};
use libpurpur::{protocols::matrix::MatrixProtocol, PurpurAPI, Update};
use tracing::metadata::LevelFilter;
use tracing_subscriber::{fmt, prelude::*};

use crate::libpurpur::protocols::{irc::IRCProtocol, BuiltinProtocols, Protocol};

pub mod libpurpur;
pub mod ui;

fn main() -> glib::ExitCode {
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

    // Create a new application
    let app = Application::builder()
        .application_id("dev.blusk.purpur")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(move |app| {
        let (uitx, uirx) = MainContext::channel::<Update>(Priority::default());
        let api = PurpurAPI { update_sender: uitx };
        let mut protocol = BuiltinProtocols::from(MatrixProtocol::new());
        gio::spawn_blocking(move || protocol.connect(api));

        let model = StringList::default();

        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let label = Label::builder()
                .halign(gtk::Align::Start)
                .build();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&label));
        });
        factory.connect_bind(move |_, list_item| {
            // Get `IntegerObject` from `ListItem`
            let string_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<StringObject>()
                .expect("The item has to be an `IntegerObject`.");

            // Get `Label` from `ListItem`
            let label = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<Label>()
                .expect("The child has to be a `Label`.");

            // Set "label" to "number"
            label.set_label(&string_object.string().to_string());
        });
        let selection_model = NoSelection::new(Some(model.clone()));
        let list_box = ListView::new(Some(selection_model), Some(factory));
        let scrolled_window = ScrolledWindow::builder()
            .min_content_width(360)
            .hexpand(false)
            .margin_top(8)
            .margin_bottom(8)
            .margin_end(8)
            .margin_start(8)
            .child(&list_box)
            .build();

        uirx.attach(
            None,
            clone!(@strong model, @weak scrolled_window => @default-return glib::ControlFlow::Break,
                move |data| {
                    match data {
                        Update::NewMessage(s) => {
                            // println!("{:?}", s);
                            model.append(&s);
                            scrolled_window.vadjustment().set_value(scrolled_window.vadjustment().upper());
                        },
                        _ => {},
                    };
                    glib::ControlFlow::Continue
                }
            ),
        );

        // Create a window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Purpur")
            .child(&scrolled_window)
            .build();

        // Present window
        window.present();
    });

    // Run the application
    app.run()
}
