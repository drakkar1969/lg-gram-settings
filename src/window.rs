use std::fs;

use gtk::{gio, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;

use crate::Application;

//------------------------------------------------------------------------------
// CONSTANTS
//------------------------------------------------------------------------------
const BATTERY_PATH: &str = "/sys/devices/platform/lg-laptop/battery_care_limit";
const FNLOCK_PATH: &str = "/sys/devices/platform/lg-laptop/fn_lock";
const READER_PATH: &str = "/sys/devices/platform/lg-laptop/reader_mode";
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
        pub(super) battery_row: TemplateChild<adw::ComboRow>,
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

        // Battery
        let battery_limit = fs::read_to_string(BATTERY_PATH).ok()
            .and_then(|value| value.trim().parse::<u32>().ok())
            .map(|value| if value == 100 { 0 } else { 1 })
            .unwrap_or_default();
        
        imp.battery_row.set_selected(battery_limit);

        // Fn lock
        let fn_lock = fs::read_to_string(FNLOCK_PATH).ok()
            .and_then(|value| value.trim().parse::<u32>().ok())
            .map(|value| value != 0)
            .unwrap_or_default();

        imp.fnlock_row.set_active(fn_lock);

        // Reader
        let reader_mode = fs::read_to_string(READER_PATH).ok()
            .and_then(|value| value.trim().parse::<u32>().ok())
            .map(|value| value != 0)
            .unwrap_or_default();

        imp.reader_row.set_active(reader_mode);

        // Fan (note 0 = silent fan enabled)
        let fan_mode = fs::read_to_string(FAN_PATH).ok()
            .and_then(|value| value.trim().parse::<u32>().ok())
            .map(|value| value == 0)
            .unwrap_or_default();

        imp.fan_row.set_active(fan_mode);

        // USB charge
        let usb_charge = fs::read_to_string(USB_PATH).ok()
            .and_then(|value| value.trim().parse::<u32>().ok())
            .map(|value| value != 0)
            .unwrap_or_default();

        imp.usb_row.set_active(usb_charge);
    }
}
