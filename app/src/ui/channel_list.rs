use gtk::{
    prelude::*, Label, ListItem, ListView, NoSelection, ScrolledWindow, SignalListItemFactory,
    StringObject, Widget,
};

use super::App;

impl App {
    /// TODO: actually implement proper channel list hierarchy
    pub fn build_channel_list(&mut self) -> ScrolledWindow {
        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let label = Label::builder().halign(gtk::Align::Start).build();
            let list_item = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem");
            list_item.set_child(Some(&label));
            list_item
                .property_expression("item")
                .chain_property::<StringObject>("label")
                .bind(&label, "label", Widget::NONE);
        });
        let selection_model = NoSelection::new(Some(self.models.message_list.clone()));
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

        return scrolled_window;
    }
}
