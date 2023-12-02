use std::thread;

use gtk::{
    glib::{self, clone, MainContext, Priority},
    prelude::*,
    Application, ApplicationWindow,
    StringList
};
use libpurpur::{protocols::irc::IRCProtocol, Purpur, Update};

mod message_list;
mod channel_list;

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
        thread::spawn(move || {
            tokio::spawn(async move {
                while let Ok(message) = receiver.recv().await {
                    glib_update_tx.send(message).unwrap();
                }
            });
        });
        self.purpur.add_protocol(Box::new(IRCProtocol::new()));

        let scrolled_window = self.build_message_list();

        let message_list_model = &self.models.message_list;
        let channel_list_model = &self.models.channel_list;
        glib_update_rx.attach(
                None,
                clone!(
                    @strong message_list_model,
                    @strong channel_list_model,
                    @weak scrolled_window => @default-return glib::ControlFlow::Break,
                    move |data| {
                        match data {
                            Update::NewMessage(s) => {
                                message_list_model.append(&s);
                                scrolled_window.vadjustment().set_value(scrolled_window.vadjustment().upper());
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
