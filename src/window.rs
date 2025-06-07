use std::fs;

use gtk::{gio, glib};
use adw::subclass::prelude::*;

use crate::Application;

//------------------------------------------------------------------------------
// CONSTANTS
//------------------------------------------------------------------------------
const FNLOCK_PATH: &str = "/sys/devices/platform/lg-laptop/fn_lock";
const READER_PATH: &str = "/sys/devices/platform/lg-laptop/reader_mode";
const BATTERY_PATH: &str = "/sys/devices/platform/lg-laptop/battery_care_limit";
const FAN_PATH: &str = "/sys/devices/platform/lg-laptop/fan_mode";
const USB_PATH: &str = "/sys/devices/platform/lg-laptop/usb_charge";

//------------------------------------------------------------------------------
// MODULE: MainWindow
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //---------------------------------------
    // Private structure
    //---------------------------------------
    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/LG-GramSettings/ui/window.ui")]
    pub struct MainWindow {
        #[template_child]
        pub(super) battery_row: TemplateChild<adw::SpinRow>,
        #[template_child]
        pub(super) fnlock_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) reader_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) fan_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) usb_row: TemplateChild<adw::SwitchRow>,
     }

    //---------------------------------------
    // Subclass
    //---------------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for MainWindow {
        const NAME: &'static str = "MainWindow";
        type Type = super::MainWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MainWindow {
        //---------------------------------------
        // Constructor
        //---------------------------------------
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            obj.init_kernel_features();
        }
    }

    impl WidgetImpl for MainWindow {}
    impl WindowImpl for MainWindow {}
    impl ApplicationWindowImpl for MainWindow {}
    impl AdwApplicationWindowImpl for MainWindow {}
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: MainWindow
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MainWindow {
    //---------------------------------------
    // New function
    //---------------------------------------
    pub fn new(app: &Application) -> Self {
        glib::Object::builder()
            .property("application", app)
            .build()
    }

    //---------------------------------------
    // Init kernel features
    //---------------------------------------
    fn init_kernel_features(&self) {
        let imp = self.imp();

        let battery_limit = fs::read_to_string(BATTERY_PATH).ok()
            .and_then(|value| value.replace("\n", "").parse::<f64>().ok())
            .unwrap_or(100.0);
        
        imp.battery_row.set_value(battery_limit);

        let fn_lock = fs::read_to_string(FNLOCK_PATH).ok()
            .and_then(|value| value.replace("\n", "").parse::<u32>().ok())
            .map(|value| value != 0)
            .unwrap_or_default();

        imp.fnlock_row.set_active(fn_lock);

        let reader_mode = fs::read_to_string(READER_PATH).ok()
            .and_then(|value| value.replace("\n", "").parse::<u32>().ok())
            .map(|value| value != 0)
            .unwrap_or_default();

        imp.reader_row.set_active(reader_mode);

        let fan_mode = fs::read_to_string(FAN_PATH).ok()
            .and_then(|value| value.replace("\n", "").parse::<u32>().ok())
            .map(|value| value == 0)
            .unwrap_or_default();

        imp.fan_row.set_active(fan_mode);

        let usb_charge = fs::read_to_string(USB_PATH).ok()
            .and_then(|value| value.replace("\n", "").parse::<u32>().ok())
            .map(|value| value != 0)
            .unwrap_or_default();

        imp.usb_row.set_active(usb_charge);
    }

    //---------------------------------------
    // Bind gsettings
    //---------------------------------------
    // fn bind_gsettings(&self) {
    //     let imp = self.imp();

    //     let settings = gio::Settings::new(APP_ID);

    //     // Bind window settings
    //     settings.bind("window-width", self, "default-width").build();
    //     settings.bind("window-height", self, "default-height").build();
    //     settings.bind("window-maximized", self, "maximized").build();

    //     // Load initial search bar settings
    //     settings.bind("search-mode", &imp.search_bar.get(), "mode")
    //         .get()
    //         .get_no_changes()
    //         .build();

    //     settings.bind("search-prop", &imp.search_bar.get(), "prop")
    //         .get()
    //         .get_no_changes()
    //         .build();

    //     // Bind preferences
    //     let prefs_dialog = imp.prefs_dialog.get().unwrap();

    //     settings.bind("color-scheme", prefs_dialog, "color-scheme").build();
    //     settings.bind("sidebar-width", prefs_dialog, "sidebar-width").build();
    //     settings.bind("infopane-width", prefs_dialog, "infopane-width").build();
    //     settings.bind("aur-database-download", prefs_dialog, "aur-database-download").build();
    //     settings.bind("aur-database-age", prefs_dialog, "aur-database-age").build();
    //     settings.bind("auto-refresh", prefs_dialog, "auto-refresh").build();
    //     settings.bind("remember-sort", prefs_dialog, "remember-sort").build();
    //     settings.bind("search-mode", prefs_dialog, "search-mode").build();
    //     settings.bind("search-prop", prefs_dialog, "search-prop").build();
    //     settings.bind("search-delay", prefs_dialog, "search-delay").build();
    //     settings.bind("property-max-lines", prefs_dialog, "property-max-lines").build();
    //     settings.bind("property-line-spacing", prefs_dialog, "property-line-spacing").build();
    //     settings.bind("underline-links", prefs_dialog, "underline-links").build();
    //     settings.bind("pkgbuild-style-scheme", prefs_dialog, "pkgbuild-style-scheme").build();
    //     settings.bind("pkgbuild-use-system-font", prefs_dialog, "pkgbuild-use-system-font").build();
    //     settings.bind("pkgbuild-custom-font", prefs_dialog, "pkgbuild-custom-font").build();

    //     // Load/save package view sort properties
    //     if prefs_dialog.remember_sort() {
    //         settings.bind("sort-prop", &imp.package_view.get(), "sort-prop")
    //             .get()
    //             .get_no_changes()
    //             .build();

    //         settings.bind("sort-ascending", &imp.package_view.get(), "sort-ascending")
    //             .get()
    //             .get_no_changes()
    //             .build();
    //     }

    //     settings.bind("sort-prop", &imp.package_view.get(), "sort-prop")
    //         .set()
    //         .build();

    //     settings.bind("sort-ascending", &imp.package_view.get(), "sort-ascending")
    //         .set()
    //         .build();
    // }
}
