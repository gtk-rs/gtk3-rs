use gtk::prelude::*;
use gtk::{HeaderBar, Label, Stack, StackSwitcher};

pub struct TitleBar {
    header: HeaderBar,
    menu: StackSwitcher,
}

impl TitleBar {
    pub fn new() -> Self {
        let menu = StackSwitcher::new();
        let header = HeaderBar::builder()
            .custom_title(&menu)
            .show_close_button(true)
            .build();
        header.add(&Label::new(Some("Custom title")));
        Self { header, menu }
    }

    pub fn set_stack(&self, stack: &Stack) {
        self.menu.set_stack(Some(stack));
    }

    pub fn header(&self) -> &HeaderBar {
        &self.header
    }
}
