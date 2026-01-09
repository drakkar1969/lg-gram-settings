use gtk::{gio, glib, pango};
use adw::subclass::prelude::*;
use adw::prelude::*;

use crate::Application;
use crate::gram_widget::GramWidget;
use crate::lg_gram::gram;

//------------------------------------------------------------------------------
// CONSTANTS
//------------------------------------------------------------------------------
const BATTERY_LIMIT: &str = "battery_care_limit";
const FN_LOCK: &str = "fn_lock";
const USB_CHARGE: &str = "usb_charge";
const FAN_MODE: &str = "fan_mode";

//------------------------------------------------------------------------------
// ENUM: BatteryCareLimit
//------------------------------------------------------------------------------
#[derive(Default, Debug, Clone, Copy, glib::Enum)]
#[repr(u32)]
#[enum_type(name = "BatteryCareLimit")]
pub enum BatteryCareLimit {
    #[default]
    #[enum_value(name = "No Limit")]
    NoLimit = 100,
    #[enum_value(name = "Limit to 80%")]
    Limit80 = 80
}

//------------------------------------------------------------------------------
// ENUM: OnOff
//------------------------------------------------------------------------------
#[derive(Default, Debug, Clone, Copy, glib::Enum)]
#[repr(u32)]
#[enum_type(name = "OnOff")]
pub enum OnOff {
    #[default]
    Disabled = 0,
    Enabled = 1
}

//------------------------------------------------------------------------------
// ENUM: FanMode
//------------------------------------------------------------------------------
#[derive(Default, Debug, Clone, Copy, glib::Enum)]
#[repr(u32)]
#[enum_type(name = "FanMode")]
pub enum FanMode {
    #[default]
    Optimized = 0,
    Silent = 1,
    Performance = 2
}

//------------------------------------------------------------------------------
// MODULE: MainWindow
//------------------------------------------------------------------------------
mod imp {
    use gtk::glib::VariantTy;

    use super::*;

    //---------------------------------------
    // Private structure
    //---------------------------------------
    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/LGGramSettings/ui/window.ui")]
    pub struct MainWindow {
        #[template_child]
        pub(super) toast_overlay: TemplateChild<adw::ToastOverlay>,

        #[template_child]
        pub(super) battery_limit_widget: TemplateChild<GramWidget>,
        #[template_child]
        pub(super) fn_lock_widget: TemplateChild<GramWidget>,
        #[template_child]
        pub(super) usb_charge_widget: TemplateChild<GramWidget>,
        #[template_child]
        pub(super) fan_mode_widget: TemplateChild<GramWidget>,
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

            // Show error toast action
            klass.install_action("win.show-error-toast", Some(VariantTy::STRING),
                |window, _, param| {
                    let error = param.and_then(|param| param.get::<String>())
                        .unwrap_or_else(|| String::from("ERROR: unknown error"));

                    let label = gtk::Label::builder()
                        .label(error.trim())
                        .ellipsize(pango::EllipsizeMode::End)
                        .css_classes(["heading", "warning"])
                        .build();

                    let toast = adw::Toast::builder()
                        .priority(adw::ToastPriority::High)
                        .custom_title(&label)
                        .build();

                    window.imp().toast_overlay.add_toast(toast);
                }
            );

            // Show system information action async
            klass.install_action_async("win.show-system-info", None, async |window, _, _| {
                match gram::system_information_async().await {
                    Ok(info) => {
                        let builder = gtk::Builder::from_resource(
                            "/com/github/LGGramSettings/ui/builder/info_dialog.ui"
                        );

                        let info_dialog: adw::Dialog = builder.object("info_dialog").unwrap();
                        let group: adw::PreferencesGroup = builder.object("group").unwrap();

                        let mut iter = info.split('\n');

                        while let Some((label, value)) = iter.next().zip(iter.next()) {
                            group.add(&adw::ActionRow::builder()
                                .title(label)
                                .subtitle(value)
                                .subtitle_selectable(true)
                                .css_classes(["property"])
                                .build()
                            );
                        }

                        info_dialog.present(Some(&window));
                    },
                    Err(error) => {
                        gtk::prelude::WidgetExt::activate_action(&window, "win.show-error-toast", Some(&error.to_variant())).unwrap();
                    }
                }
            });

            // Open settings folder action
            klass.install_action("win.open-settings-folder", None, |_, _, _| {
                let uri = "file:///sys/devices/platform/lg-laptop";

                if let Some(desktop) = gio::AppInfo::default_for_type("inode/directory", true) {
                    let _res = desktop.launch_uris(&[uri], None::<&gio::AppLaunchContext>);
                }
            });
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

            self.obj().init_kernel_features();
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

        imp.battery_limit_widget.init(BATTERY_LIMIT, BatteryCareLimit::static_type());
        imp.fn_lock_widget.init(FN_LOCK, OnOff::static_type());
        imp.usb_charge_widget.init(USB_CHARGE, OnOff::static_type());
        imp.fan_mode_widget.init(FAN_MODE, FanMode::static_type());
    }
}
