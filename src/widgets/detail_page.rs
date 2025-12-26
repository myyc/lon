use crate::color::PantoneColor;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gdk, glib, graphene, gsk};
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct ColorDetailPanel {
        pub color: RefCell<Option<PantoneColor>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ColorDetailPanel {
        const NAME: &'static str = "LonColorDetailPanel";
        type Type = super::ColorDetailPanel;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for ColorDetailPanel {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().set_orientation(gtk::Orientation::Vertical);
            self.obj().set_spacing(8);
            self.obj().set_margin_start(16);
            self.obj().set_margin_end(16);
            self.obj().set_margin_top(16);
            self.obj().set_margin_bottom(16);
        }
    }

    impl WidgetImpl for ColorDetailPanel {}
    impl BoxImpl for ColorDetailPanel {}
}

glib::wrapper! {
    pub struct ColorDetailPanel(ObjectSubclass<imp::ColorDetailPanel>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Orientable;
}

impl ColorDetailPanel {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_color(&self, color: &PantoneColor) {
        self.imp().color.replace(Some(color.clone()));

        // Clear existing children
        while let Some(child) = self.first_child() {
            self.remove(&child);
        }

        // Color preview (tap to copy)
        let preview = ColorPreview::new(color);
        preview.set_height_request(200);
        preview.set_hexpand(true);
        preview.set_cursor_from_name(Some("pointer"));

        let gesture = gtk::GestureClick::new();
        let hex_value = color.hex.clone();
        gesture.connect_released(move |gesture, _, _, _| {
            if let Some(display) = gdk::Display::default() {
                display.clipboard().set_text(&hex_value);
                if let Some(widget) = gesture.widget() {
                    let mut parent = widget.parent();
                    while let Some(p) = parent {
                        if let Ok(overlay) = p.clone().downcast::<adw::ToastOverlay>() {
                            let toast = adw::Toast::new("Copied");
                            toast.set_timeout(1);
                            overlay.add_toast(toast);
                            break;
                        }
                        parent = p.parent();
                    }
                }
            }
        });
        preview.add_controller(gesture);

        self.append(&preview);

        // Color name
        let name_label = gtk::Label::new(Some(&color.name));
        name_label.add_css_class("title-2");
        name_label.set_halign(gtk::Align::Center);
        name_label.set_margin_top(8);
        self.append(&name_label);

        // HEX value (tap to copy)
        let hex_label = gtk::Label::new(Some(&color.hex));
        hex_label.add_css_class("dim-label");
        hex_label.set_halign(gtk::Align::Center);
        hex_label.set_cursor_from_name(Some("pointer"));

        let gesture = gtk::GestureClick::new();
        let hex_value = color.hex.clone();
        gesture.connect_released(move |gesture, _, _, _| {
            if let Some(display) = gdk::Display::default() {
                display.clipboard().set_text(&hex_value);
                // Show toast
                if let Some(widget) = gesture.widget() {
                    let mut parent = widget.parent();
                    while let Some(p) = parent {
                        if let Ok(overlay) = p.clone().downcast::<adw::ToastOverlay>() {
                            let toast = adw::Toast::new("Copied");
                            toast.set_timeout(1);
                            overlay.add_toast(toast);
                            break;
                        }
                        parent = p.parent();
                    }
                }
            }
        });
        hex_label.add_controller(gesture);

        self.append(&hex_label);
    }
}

impl Default for ColorDetailPanel {
    fn default() -> Self {
        Self::new()
    }
}

// Simple color preview widget
mod color_preview {
    use super::*;

    mod imp {
        use super::*;

        #[derive(Default)]
        pub struct ColorPreview {
            pub color: RefCell<Option<PantoneColor>>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for ColorPreview {
            const NAME: &'static str = "LonColorPreview";
            type Type = super::ColorPreview;
            type ParentType = gtk::Widget;
        }

        impl ObjectImpl for ColorPreview {}

        impl WidgetImpl for ColorPreview {
            fn snapshot(&self, snapshot: &gtk::Snapshot) {
                let widget = self.obj();
                let width = widget.width() as f32;
                let height = widget.height() as f32;

                if let Some(color) = self.color.borrow().as_ref() {
                    let gdk_color = gdk::RGBA::new(
                        color.rgb.r as f32 / 255.0,
                        color.rgb.g as f32 / 255.0,
                        color.rgb.b as f32 / 255.0,
                        1.0,
                    );

                    let rect = graphene::Rect::new(0.0, 0.0, width, height);
                    let rounded = gsk::RoundedRect::from_rect(rect, 12.0);

                    snapshot.push_rounded_clip(&rounded);
                    snapshot.append_color(&gdk_color, &rect);
                    snapshot.pop();
                }
            }
        }
    }

    glib::wrapper! {
        pub struct ColorPreview(ObjectSubclass<imp::ColorPreview>)
            @extends gtk::Widget;
    }

    impl ColorPreview {
        pub fn new(color: &PantoneColor) -> Self {
            let obj: Self = glib::Object::new();
            obj.imp().color.replace(Some(color.clone()));
            obj
        }
    }
}

use color_preview::ColorPreview;
