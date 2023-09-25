use gtk::prelude::*;
use kanal::Receiver;
use libpurpur::{PurpurAPI, UIAction};
use relm4::{prelude::*, Worker};

use crate::libpurpur::protocols::{discord::Discord, BuiltinProtocols, Protocol};

pub mod libpurpur;

struct App {}

struct AsyncHandler;

impl Worker for AsyncHandler {
    type Init = ();

    type Input = ();

    type Output = UIAction;

    fn init(_init: Self::Init, _sender: ComponentSender<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: Self::Input, sender: ComponentSender<Self>) {}
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = UIAction;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Simple app"),
            set_default_size: (300, 100),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,
            }
        }
    }

    // Initialize the component.
    fn init(
        counter: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (tx, rx) = kanal::unbounded();
        let api = PurpurAPI { action_sender: tx };

        std::thread::spawn(move || {
            let mut protocol = BuiltinProtocols::from(Discord {});
            protocol.connect(api);
        });

        std::thread::spawn(move || {
            let a = rx.recv().unwrap();
            sender.input(a);
        });

        let model = App {};

        // Insert the code generation of the view! macro here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        println!("hello {:?}", msg);
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.simple");
    app.run::<App>(());
}
