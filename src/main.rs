use dotenv::dotenv;
use gtk::{
    gio,
    glib::{self, clone, MainContext, Priority},
    prelude::*,
    Application, ApplicationWindow, Label, ListItem, ListView, NoSelection, PolicyType,
    ScrolledWindow, SignalListItemFactory, StringList, StringObject,
};
use libpurpur::{PurpurAPI, UIAction, protocols::matrix::MatrixProtocol};

use crate::libpurpur::protocols::{
    irc::IRCProtocol, BuiltinProtocols, Protocol,
};

pub mod libpurpur;
pub mod ui;

fn main() -> glib::ExitCode {
    dotenv().ok();

    // Create a new application
    let app = Application::builder()
        .application_id("dev.blusk.purpur")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(move |app| {
        let (uitx, uirx) = MainContext::channel::<UIAction>(Priority::default());
        let api = PurpurAPI { action_sender: uitx };
        let mut protocol = BuiltinProtocols::from(MatrixProtocol {});
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
                        UIAction::NewMessage(s) => {
                            println!("{:?}", s);
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
