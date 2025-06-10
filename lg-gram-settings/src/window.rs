use std::cell::Cell;

use gtk::{gio, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;
use glib::clone;

use crate::Application;
use crate::lg_gram::gram;
use crate::battery_limit_object::BatteryLimitObject;

//------------------------------------------------------------------------------
// CONSTANTS
//------------------------------------------------------------------------------
const BATTERY_LIMIT: &str = "battery_care_limit";
const FN_LOCK: &str = "fn_lock";
const READER_MODE: &str = "reader_mode";
const USB_CHARGE: &str = "usb_charge";

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
        pub(super) toast_overlay: TemplateChild<adw::ToastOverlay>,

        #[template_child]
        pub(super) fn_lock_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) fn_persistent_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) battery_limit_row: TemplateChild<adw::ComboRow>,
        #[template_child]
        pub(super) battery_limit_model: TemplateChild<gio::ListStore>,
        #[template_child]
        pub(super) battery_persistent_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) usb_charge_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) usb_persistent_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) reader_mode_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) reader_persistent_row: TemplateChild<adw::SwitchRow>,

        pub(super) is_fn_lock_reverting: Cell<bool>,
        pub(super) is_battery_limit_reverting: Cell<bool>,
        pub(super) is_usb_charge_reverting: Cell<bool>,
        pub(super) is_reader_mode_reverting: Cell<bool>,
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
            BatteryLimitObject::ensure_type();

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
    // Show toast helper function
    //---------------------------------------
    fn show_toast(&self, error: &str) {
        let label = gtk::Label::builder()
            .label(error.trim())
            .css_classes(["heading", "warning"])
            .build();

        let toast = adw::Toast::builder()
            .priority(adw::ToastPriority::High)
            .custom_title(&label)
            .build();

        self.imp().toast_overlay.add_toast(toast);
    }

    //---------------------------------------
    // Init kernel features
    //---------------------------------------
    fn init_kernel_features(&self) {
        let imp = self.imp();

        // Fn lock
        match gram::feature(FN_LOCK) {
            Ok(lock) => {
                imp.fn_lock_row.set_active(lock != 0);
            },
            Err(error) => {
                imp.fn_lock_row.set_sensitive(false);
                imp.fn_persistent_row.set_sensitive(false);

                self.show_toast(&format!("Failed to load Fn lock status: {error}"));
            }
        }

        // Battery limit
        match gram::feature(BATTERY_LIMIT) {
            Ok(limit) => {
                let index = imp.battery_limit_model.iter::<BatteryLimitObject>()
                    .flatten()
                    .position(|item| item.value() == limit)
                    .unwrap_or_default();

                imp.battery_limit_row.set_selected(index as u32);
            },
            Err(error) => {
                imp.battery_limit_row.set_sensitive(false);
                imp.battery_persistent_row.set_sensitive(false);

                self.show_toast(&format!("Failed to load battery care limit: {error}"));
            }
        }

        // USB charge
        match gram::feature(USB_CHARGE) {
            Ok(charge) => {
                imp.usb_charge_row.set_active(charge != 0);
            },
            Err(error) => {
                imp.usb_charge_row.set_sensitive(false);
                imp.usb_persistent_row.set_sensitive(false);

                self.show_toast(&format!("Failed to load USB charge mode: {error}"));
            }
        }

        // Reader mode
        match gram::feature(READER_MODE) {
            Ok(mode) => {
                imp.reader_mode_row.set_active(mode != 0);
            },
            Err(error) => {
                imp.reader_mode_row.set_sensitive(false);
                imp.reader_persistent_row.set_sensitive(false);

                self.show_toast(&format!("Failed to load reader mode: {error}"));
            }
        }
    }

    //---------------------------------------
    // Setup signals
    //---------------------------------------
    fn setup_signals(&self) {
        let imp = self.imp();

        // Fn lock
        imp.fn_lock_row.connect_active_notify(clone!(
            #[weak(rename_to = window)] self,
            move |row| {
                let imp = window.imp();

                if imp.is_fn_lock_reverting.get() {
                    imp.is_fn_lock_reverting.set(false);
                    return
                }

                let value = u32::from(row.is_active());

                if let Err(error) = gram::set_feature(FN_LOCK, value) {
                    imp.is_fn_lock_reverting.set(true);
                    row.set_active(!row.is_active());

                    window.show_toast(&error);
                }
            }
        ));

        // Battery limit
        imp.battery_limit_row.connect_selected_notify(clone!(
            #[weak(rename_to = window)] self,
            move |row| {
                let imp = window.imp();

                if imp.is_battery_limit_reverting.get() {
                    imp.is_battery_limit_reverting.set(false);
                    return
                }

                let value = row.selected_item()
                    .and_downcast_ref::<BatteryLimitObject>()
                    .expect("Failed to downcast to 'BatteryLimitObject'")
                    .value();

                if let Err(error) = gram::set_feature(BATTERY_LIMIT, value) {
                    imp.is_battery_limit_reverting.set(true);
                    row.set_selected(1 - row.selected());

                    window.show_toast(&error);
                }
            }
        ));

        // USB charge
        imp.usb_charge_row.connect_active_notify(clone!(
            #[weak(rename_to = window)] self,
            move |row| {
                let imp = window.imp();

                if imp.is_usb_charge_reverting.get() {
                    imp.is_usb_charge_reverting.set(false);
                    return
                }

                let value = u32::from(row.is_active());

                if let Err(error) = gram::set_feature(USB_CHARGE, value) {
                    imp.is_usb_charge_reverting.set(true);
                    row.set_active(!row.is_active());

                    window.show_toast(&error);
                }
            }
        ));

        // Reader mode
        imp.reader_mode_row.connect_active_notify(clone!(
            #[weak(rename_to = window)] self,
            move |row| {
                let imp = window.imp();

                if imp.is_reader_mode_reverting.get() {
                    imp.is_reader_mode_reverting.set(false);
                    return
                }

                let value = u32::from(row.is_active());

                if let Err(error) = gram::set_feature(READER_MODE, value) {
                    imp.is_reader_mode_reverting.set(true);
                    row.set_active(!row.is_active());

                    window.show_toast(&error);
                }
            }
        ));
    }
}
