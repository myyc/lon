use crate::color::PantoneColor;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;
use gtk::{gdk, graphene, gsk};
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct ColorSwatch {
        pub color: RefCell<Option<PantoneColor>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ColorSwatch {
        const NAME: &'static str = "LonColorSwatch";
        type Type = super::ColorSwatch;
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for ColorSwatch {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.set_size_request(72, 72);
            obj.set_overflow(gtk::Overflow::Hidden);
            obj.set_cursor_from_name(Some("pointer"));
        }
    }

    impl WidgetImpl for ColorSwatch {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let widget = self.obj();
            let width = widget.width() as f32;
            let height = widget.height() as f32;

            if width <= 0.0 || height <= 0.0 {
                return;
            }

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

        fn measure(&self, orientation: gtk::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            match orientation {
                gtk::Orientation::Horizontal => (72, 72, -1, -1),
                gtk::Orientation::Vertical => (72, 72, -1, -1),
                _ => (72, 72, -1, -1),
            }
        }
    }
}

glib::wrapper! {
    pub struct ColorSwatch(ObjectSubclass<imp::ColorSwatch>)
        @extends gtk::Widget;
}

impl Default for ColorSwatch {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorSwatch {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_color(&self, color: &PantoneColor) {
        self.imp().color.replace(Some(color.clone()));
        self.queue_draw();
    }
}
