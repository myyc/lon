use crate::widgets::ColorObject;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct InfiniteListModel {
        pub base: RefCell<Option<gio::ListStore>>,
        pub multiplier: std::cell::Cell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for InfiniteListModel {
        const NAME: &'static str = "LonInfiniteListModel";
        type Type = super::InfiniteListModel;
        type Interfaces = (gio::ListModel,);
    }

    impl ObjectImpl for InfiniteListModel {}

    impl ListModelImpl for InfiniteListModel {
        fn item_type(&self) -> glib::Type {
            ColorObject::static_type()
        }

        fn n_items(&self) -> u32 {
            self.base
                .borrow()
                .as_ref()
                .map(|b| b.n_items().saturating_mul(self.multiplier.get()))
                .unwrap_or(0)
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            self.base.borrow().as_ref().and_then(|base| {
                let real_count = base.n_items();
                if real_count == 0 {
                    return None;
                }
                let real_pos = position % real_count;
                base.item(real_pos)
            })
        }
    }
}

glib::wrapper! {
    pub struct InfiniteListModel(ObjectSubclass<imp::InfiniteListModel>)
        @implements gio::ListModel;
}

impl InfiniteListModel {
    pub fn new(base: gio::ListStore, multiplier: u32) -> Self {
        let obj: Self = glib::Object::new();
        obj.imp().base.replace(Some(base));
        obj.imp().multiplier.set(multiplier);
        obj
    }

    pub fn real_count(&self) -> u32 {
        self.imp()
            .base
            .borrow()
            .as_ref()
            .map(|b| b.n_items())
            .unwrap_or(0)
    }

    pub fn middle_position(&self) -> u32 {
        let real_count = self.real_count();
        let multiplier = self.imp().multiplier.get();
        // Start at middle of the virtual list
        (multiplier / 2) * real_count
    }
}
