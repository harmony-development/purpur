use std::thread;

use gtk::{
    glib::{self, clone, MainContext, Priority},
    prelude::*,
    Application, ApplicationWindow, Label, ListItem, ListView, NoSelection, ScrolledWindow,
    SignalListItemFactory, StringList, StringObject,
};
use libpurpur::{protocols::irc::IRCProtocol, Purpur, Update};

#[derive(Clone)]
pub struct Models {
    message_list: StringList,
    channel_list: StringList,
}

pub struct App {
    pub purpur: Purpur,
    pub gtk_app: Application,
    pub models: Models,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            purpur: Purpur::new(),
            gtk_app: Application::builder()
                .application_id("dev.blusk.purpur")
                .build(),
            models: Models {
                message_list: StringList::default(),
                channel_list: StringList::default(),
            },
        };

        // Connect to "activate" signal of `app`
        app.gtk_app.connect_activate(|x| app.build_ui(x));

        app
    }

    pub fn build_ui(&mut self, application: &Application) {
        let models = self.models.clone();
        let (glib_update_tx, glib_update_rx) = MainContext::channel::<Update>(Priority::DEFAULT);
        let receiver = self.purpur.update_receiver.clone();
        thread::spawn(move || {
            tokio::spawn(async move {
                while let Ok(message) = receiver.recv().await {
                    glib_update_tx.send(message).unwrap();
                }
            });
        });
        self.purpur.add_protocol(Box::new(IRCProtocol::new()));

        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let label = Label::builder().halign(gtk::Align::Start).build();
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
                .expect("The item has to be an `StringObject`.");

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
        let message_list_model = models.message_list.clone();
        let selection_model = NoSelection::new(Some(message_list_model.clone()));
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

        glib_update_rx.attach(
                None,
                clone!(@strong message_list_model, @weak scrolled_window => @default-return glib::ControlFlow::Break,
                    move |data| {
                        match data {
                            Update::NewMessage(s) => {
                                // println!("{:?}", s);
                                message_list_model.append(&s);
                                scrolled_window.vadjustment().set_value(scrolled_window.vadjustment().upper());
                            },
                            Update::NewChannel(c) => {

                            },
                            _ => {},
                        };
                        glib::ControlFlow::Continue
                    }
                ),
            );

        // Create a window
        let window = ApplicationWindow::builder()
            .application(application)
            .title("Purpur")
            .child(&scrolled_window)
            .build();

        // Present window
        window.present();
    }
}
