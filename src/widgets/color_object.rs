use crate::color::PantoneColor;
use adw::subclass::prelude::*;
use gtk::glib;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct ColorObject {
        pub color: RefCell<Option<PantoneColor>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ColorObject {
        const NAME: &'static str = "LonColorObject";
        type Type = super::ColorObject;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for ColorObject {}
}

glib::wrapper! {
    pub struct ColorObject(ObjectSubclass<imp::ColorObject>);
}

impl ColorObject {
    pub fn new(color: PantoneColor) -> Self {
        let obj: Self = glib::Object::new();
        obj.imp().color.replace(Some(color));
        obj
    }

    pub fn color(&self) -> PantoneColor {
        self.imp().color.borrow().clone().unwrap()
    }
}
