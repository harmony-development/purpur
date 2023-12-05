use std::thread;

use gtk::{
    glib::{self, clone, MainContext, Priority},
    prelude::*,
    Application, ApplicationWindow, Box as GtkBox, StringList,
};
use libpurpur::{protocols::irc::IRCProtocol, Purpur, Update};

mod channel_list;
mod message_list;

#[derive(Clone)]
pub struct Models {
    message_list: StringList,
    channel_list: StringList,
}

pub struct App {
    pub purpur: Purpur,
    pub models: Models,
}

impl App {
    pub fn new() -> App {
        App {
            purpur: Purpur::new(),
            models: Models {
                message_list: StringList::default(),
                channel_list: StringList::default(),
            },
        }
    }

    pub fn build_ui(&mut self, application: &Application) {
        let (glib_update_tx, glib_update_rx) = MainContext::channel::<Update>(Priority::DEFAULT);
        let receiver = self.purpur.update_receiver.clone();
        tokio::spawn(async move {
            while let Ok(message) = receiver.recv().await {
                glib_update_tx.send(message).unwrap();
            }
        });
        self.purpur.add_protocol(Box::new(IRCProtocol::new()));

        let message_list = self.build_message_list();
        let channel_list = self.build_channel_list();

        let message_list_model = &self.models.message_list;
        let channel_list_model = &self.models.channel_list;
        glib_update_rx.attach(
                None,
                clone!(
                    @strong message_list_model,
                    @strong channel_list_model,
                    @weak message_list => @default-return glib::ControlFlow::Break,
                    move |data| {
                        match data {
                            Update::NewMessage(s) => {
                                message_list_model.append(&s);
                                message_list.vadjustment().set_value(message_list.vadjustment().upper());
                            },
                            Update::NewChannel(c) => {
                                channel_list_model.append(&c.name);
                            },
                            _ => {},
                        };
                        glib::ControlFlow::Continue
                    }
                ),
            );

        let main_view = GtkBox::new(gtk::Orientation::Horizontal, 2);
        main_view.append(&channel_list);
        main_view.append(&message_list);

        // Create a window
        let window = ApplicationWindow::builder()
            .application(application)
            .title("Purpur")
            .child(&main_view)
            .build();

        // Present window
        window.present();
    }
}
