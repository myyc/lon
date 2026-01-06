use crate::color::{ColorDatabase, ColorLibrary, PantoneColor};
use crate::widgets::{ColorDetailPanel, ColorObject, ColorSwatch, InfiniteListModel};
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct LonWindow {
        pub bottom_sheet: RefCell<Option<adw::BottomSheet>>,
        pub detail_panel: RefCell<Option<ColorDetailPanel>>,
        pub section_toast: RefCell<Option<adw::Toast>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LonWindow {
        const NAME: &'static str = "LonWindow";
        type Type = super::LonWindow;
        type ParentType = adw::ApplicationWindow;
    }

    impl ObjectImpl for LonWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().setup_ui();
        }
    }

    impl WidgetImpl for LonWindow {}
    impl WindowImpl for LonWindow {}
    impl ApplicationWindowImpl for LonWindow {}
    impl AdwApplicationWindowImpl for LonWindow {}
}

glib::wrapper! {
    pub struct LonWindow(ObjectSubclass<imp::LonWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl LonWindow {
    pub fn new(app: &adw::Application) -> Self {
        let window: Self = glib::Object::builder()
            .property("application", app)
            .property("default-width", 400)
            .property("default-height", 600)
            .property("title", "lon")
            .build();
        window.set_size_request(280, 400);
        window.set_resizable(true);
        window
    }

    fn setup_ui(&self) {
        let imp = self.imp();

        // Custom CSS
        let css = gtk::CssProvider::new();
        css.load_from_string(
            "@define-color window_bg_color #0a0a0a;
             @define-color view_bg_color #0a0a0a;
             @define-color headerbar_bg_color #0f0f0f;
             @define-color popover_bg_color #0f0f0f;
             @define-color card_bg_color #0f0f0f;
             @define-color dialog_bg_color #0a0a0a;
             @define-color bottom_sheet_bg #0f0f0f;
             .no-scrollbar scrollbar { opacity: 0; }
             gridview { padding: 6px; }
             gridview > child:hover {
                 background: @accent_bg_color;
                 border-radius: 14px;
             }
             .close-btn { background: alpha(@window_bg_color, 0.8); border-radius: 50%; }
             carouselindicatordots { background: transparent; }
"
        );
        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().unwrap(),
            &css,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Pre-load color data
        let db = ColorDatabase::new();

        // Carousel for swiping between libraries
        let carousel = adw::Carousel::new();
        carousel.set_allow_long_swipes(true);
        carousel.set_vexpand(true);

        let tcx_grid = self.create_grid_view(&db, ColorLibrary::FashionHomeTcx);
        let solid_grid = self.create_grid_view(&db, ColorLibrary::SolidCoated);

        carousel.append(&tcx_grid);
        carousel.append(&solid_grid);

        // Connect carousel page change to show toast
        carousel.connect_page_changed(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |carousel, index| {
                let imp = window.imp();

                // Dismiss previous toast
                if let Some(old_toast) = imp.section_toast.borrow().as_ref() {
                    old_toast.dismiss();
                }

                let name = if index == 0 { "TCX" } else { "Solid Coated" };
                let mut parent = carousel.parent();
                while let Some(p) = parent {
                    if let Ok(overlay) = p.clone().downcast::<adw::ToastOverlay>() {
                        let toast = adw::Toast::new(name);
                        toast.set_timeout(1);
                        overlay.add_toast(toast.clone());
                        imp.section_toast.replace(Some(toast));
                        break;
                    }
                    parent = p.parent();
                }
            }
        ));

        // Overlay for carousel and indicator dots
        let carousel_overlay = gtk::Overlay::new();
        carousel_overlay.set_child(Some(&carousel));

        // Carousel indicator dots (overlaid at bottom)
        let indicators = adw::CarouselIndicatorDots::new();
        indicators.set_carousel(Some(&carousel));
        indicators.set_halign(gtk::Align::Center);
        indicators.set_valign(gtk::Align::End);
        indicators.set_margin_bottom(12);
        carousel_overlay.add_overlay(&indicators);

        // Create bottom sheet
        let bottom_sheet = adw::BottomSheet::new();
        bottom_sheet.set_content(Some(&carousel_overlay));
        bottom_sheet.set_show_drag_handle(false);

        // Create detail panel for the sheet
        let detail_panel = ColorDetailPanel::new();
        bottom_sheet.set_sheet(Some(&detail_panel));

        imp.bottom_sheet.replace(Some(bottom_sheet.clone()));
        imp.detail_panel.replace(Some(detail_panel));

        // Overlay for close button
        let overlay = gtk::Overlay::new();
        overlay.set_child(Some(&bottom_sheet));

        // Top-right button box
        let btn_box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
        btn_box.set_halign(gtk::Align::End);
        btn_box.set_valign(gtk::Align::Start);
        btn_box.set_margin_top(8);
        btn_box.set_margin_end(8);

        // Close button
        let close_btn = gtk::Button::from_icon_name("window-close-symbolic");
        close_btn.add_css_class("close-btn");
        close_btn.add_css_class("circular");
        close_btn.connect_clicked(glib::clone!(
            #[weak]
            bottom_sheet,
            move |btn| {
                if bottom_sheet.is_open() {
                    bottom_sheet.set_open(false);
                } else if let Some(window) = btn.root().and_then(|r| r.downcast::<gtk::Window>().ok()) {
                    window.close();
                }
            }
        ));
        btn_box.append(&close_btn);

        overlay.add_overlay(&btn_box);

        let toast_overlay = adw::ToastOverlay::new();
        toast_overlay.set_child(Some(&overlay));
        self.set_content(Some(&toast_overlay));
    }

    fn create_grid_view(&self, db: &ColorDatabase, library: ColorLibrary) -> gtk::ScrolledWindow {
        let scrolled = gtk::ScrolledWindow::new();
        scrolled.add_css_class("no-scrollbar");
        scrolled.set_margin_bottom(1);
        scrolled.set_hexpand(true);
        scrolled.set_vexpand(true);

        // Create and populate base model
        let base_model = gio::ListStore::new::<ColorObject>();
        let mut colors: Vec<_> = db.get_library(library).to_vec();
        colors.sort_by(|a, b| {
            a.hsl.h.partial_cmp(&b.hsl.h).unwrap_or(std::cmp::Ordering::Equal)
        });
        for color in colors {
            base_model.append(&ColorObject::new(color));
        }

        // Wrap in infinite model
        let infinite_model = InfiniteListModel::new(base_model, 1000);
        let middle_pos = infinite_model.middle_position();

        // Create GridView
        let factory = gtk::SignalListItemFactory::new();

        factory.connect_setup(|_, list_item| {
            let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
            let swatch = ColorSwatch::new();
            list_item.set_child(Some(&swatch));
        });

        factory.connect_bind(|_, list_item| {
            let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
            let color_obj = list_item
                .item()
                .and_downcast::<ColorObject>()
                .expect("Item must be ColorObject");
            let swatch = list_item
                .child()
                .and_downcast::<ColorSwatch>()
                .expect("Child must be ColorSwatch");
            swatch.set_color(&color_obj.color());
            swatch.set_tooltip_text(Some(&color_obj.color().name));
        });

        let selection = gtk::SingleSelection::new(Some(infinite_model));
        let grid_view = gtk::GridView::new(Some(selection.clone()), Some(factory));
        grid_view.set_min_columns(3);
        grid_view.set_max_columns(6);
        grid_view.set_enable_rubberband(false);
        grid_view.set_single_click_activate(true);

        grid_view.connect_activate(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |grid, position| {
                let model = grid.model().unwrap();
                if let Some(item) = model.item(position) {
                    let color_obj = item.downcast::<ColorObject>().unwrap();
                    window.show_color_detail(&color_obj.color());
                }
            }
        ));

        // Scroll to middle after GridView is mapped
        grid_view.connect_map(move |grid| {
            let grid = grid.clone();
            let selection = selection.clone();
            glib::timeout_add_local_once(std::time::Duration::from_millis(100), move || {
                selection.set_selected(middle_pos);
                grid.scroll_to(middle_pos, gtk::ListScrollFlags::FOCUS, None);
                grid.queue_draw();
            });
        });

        scrolled.set_child(Some(&grid_view));
        scrolled
    }

    fn show_color_detail(&self, color: &PantoneColor) {
        let imp = self.imp();
        if let Some(panel) = imp.detail_panel.borrow().as_ref() {
            panel.set_color(color);
        }
        if let Some(sheet) = imp.bottom_sheet.borrow().as_ref() {
            sheet.set_open(true);
        }
    }
}
