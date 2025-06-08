use gtk::{gio, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;

use crate::Application;
use crate::kernel_features::kernel_features;

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
        match kernel_features::battery_limit() {
            Ok(limit) => { imp.battery_row.set_selected(limit); },
            Err(_) => {}
        }

        // Fn lock
        match kernel_features::fn_lock() {
            Ok(lock) => { imp.fnlock_row.set_active(lock); },
            Err(_) => {}
        }

        // Reader
        match kernel_features::reader_mode() {
            Ok(mode) => { imp.reader_row.set_active(mode); },
            Err(_) => {}
        }

        // Fan
        match kernel_features::fan_mode() {
            Ok(mode) => { imp.fan_row.set_active(mode); },
            Err(_) => {}
        }

        // USB charge
        match kernel_features::usb_charge() {
            Ok(charge) => { imp.usb_row.set_active(charge); },
            Err(_) => {}
        }
    }
}
