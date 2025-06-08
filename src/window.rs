use std::cell::Cell;

use gtk::{gio, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;
use glib::clone;

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
        pub(super) battery_limit_row: TemplateChild<adw::ComboRow>,
        #[template_child]
        pub(super) usb_charge_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) reader_mode_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) fn_lock_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) fan_mode_row: TemplateChild<adw::SwitchRow>,

        pub(super) is_battery_limit_reverting: Cell<bool>,
        pub(super) is_fn_lock_reverting: Cell<bool>,
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
            obj.setup_signals();
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

        // Battery limit
        match kernel_features::battery_limit() {
            Ok(limit) => { imp.battery_limit_row.set_selected(limit); },
            Err(_) => {}
        }

        // USB charge
        match kernel_features::usb_charge() {
            Ok(charge) => { imp.usb_charge_row.set_active(charge); },
            Err(_) => {}
        }

        // Reader mode
        match kernel_features::reader_mode() {
            Ok(mode) => { imp.reader_mode_row.set_active(mode); },
            Err(_) => {}
        }

        // Fn lock
        match kernel_features::fn_lock() {
            Ok(lock) => { imp.fn_lock_row.set_active(lock); },
            Err(_) => {}
        }

        // Fan mode
        match kernel_features::fan_mode() {
            Ok(mode) => { imp.fan_mode_row.set_active(mode); },
            Err(_) => {}
        }
    }

    //---------------------------------------
    // Setup signals
    //---------------------------------------
    fn setup_signals(&self) {
        let imp = self.imp();

        // Battery limit
        imp.battery_limit_row.connect_selected_notify(clone!(
            #[weak] imp,
            move |row| {
                if imp.is_battery_limit_reverting.get() {
                    imp.is_battery_limit_reverting.set(false);
                    return
                }

                let value = if row.selected() == 1 { 80 } else { 100 };

                match kernel_features::set_battery_limit(value) {
                    Ok(status) if !status.success() => {
                        imp.is_battery_limit_reverting.set(true);
                        row.set_selected(1 - row.selected());
                    },
                    Err(_) => {
                        imp.is_battery_limit_reverting.set(true);
                        row.set_selected(1 - row.selected());
                    },
                    _ => {}
                }
            }
        ));

        // Fn lock
        imp.fn_lock_row.connect_active_notify(clone!(
            #[weak] imp,
            move |row| {
                if imp.is_fn_lock_reverting.get() {
                    imp.is_fn_lock_reverting.set(false);
                    return
                }

                let value = if row.is_active() { 1 } else { 0 };

                match kernel_features::set_fn_lock(value) {
                    Ok(status) if !status.success() => {
                        imp.is_fn_lock_reverting.set(true);
                        row.set_active(!row.is_active());
                    },
                    Err(_) => {
                        imp.is_fn_lock_reverting.set(true);
                        row.set_active(!row.is_active());
                    },
                    _ => {}
                }
            }
        ));
    }
}
