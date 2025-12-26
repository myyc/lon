use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::window::LonWindow;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct LonApplication;

    #[glib::object_subclass]
    impl ObjectSubclass for LonApplication {
        const NAME: &'static str = "LonApplication";
        type Type = super::LonApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for LonApplication {}

    impl ApplicationImpl for LonApplication {
        fn activate(&self) {
            let app = self.obj();
            let window = LonWindow::new(app.upcast_ref());
            window.present();
        }

        fn startup(&self) {
            self.parent_startup();

            let app = self.obj();

            // Set up actions
            let quit_action = gio::ActionEntry::builder("quit")
                .activate(|app: &super::LonApplication, _, _| {
                    app.quit();
                })
                .build();

            let about_action = gio::ActionEntry::builder("about")
                .activate(|app: &super::LonApplication, _, _| {
                    let window = app.active_window();
                    let about = adw::AboutDialog::builder()
                        .application_name("lon")
                        .application_icon("dev.myyc.lon")
                        .developer_name("myyc")
                        .version("0.1.0")
                        .license_type(gtk::License::Gpl30)
                        .comments("A Pantone color browser")
                        .build();
                    about.present(window.as_ref());
                })
                .build();

            app.add_action_entries([quit_action, about_action]);
            app.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl GtkApplicationImpl for LonApplication {}
    impl AdwApplicationImpl for LonApplication {}
}

glib::wrapper! {
    pub struct LonApplication(ObjectSubclass<imp::LonApplication>)
        @extends adw::Application, gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl LonApplication {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", "dev.myyc.lon")
            .property("resource-base-path", "/dev/myyc/lon")
            .build()
    }
}

impl Default for LonApplication {
    fn default() -> Self {
        Self::new()
    }
}
