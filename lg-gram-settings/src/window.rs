use gtk::{gio, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;
use glib::{clone, closure_local};

use crate::Application;
use crate::gram_setting_widget::GramSettingWidget;
use crate::battery_limit_object::BatteryLimitObject;
use crate::lg_gram::gram;

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
        pub(super) fn_lock_toggle: TemplateChild<GramSettingWidget>,
        #[template_child]
        pub(super) battery_limit_toggle: TemplateChild<GramSettingWidget>,
        #[template_child]
        pub(super) usb_charge_toggle: TemplateChild<GramSettingWidget>,
        #[template_child]
        pub(super) reader_mode_toggle: TemplateChild<GramSettingWidget>,
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

            // Show system information action
            klass.install_action("win.show-system-information", None, |window, _, _| {
                glib::spawn_future_local(clone!(
                    #[weak] window,
                    async move {
                        match gram::system_information() {
                            Ok(info) => {
                                let builder = gtk::Builder::from_resource(
                                    "/com/github/LG-GramSettings/ui/builder/info_dialog.ui"
                                );

                                let info_dialog: adw::Dialog = builder.object("info_dialog").unwrap();
                                let group: adw::PreferencesGroup = builder.object("group").unwrap();

                                let mut iter = info.split('\n');
                                
                                while let (Some(label), Some(value)) = (iter.next(), iter.next()) {
                                    if !label.is_empty() {
                                        group.add(&adw::ActionRow::builder()
                                            .title(label)
                                            .subtitle(value)
                                            .subtitle_selectable(true)
                                            .css_classes(["property"])
                                            .build()
                                        );
                                    }
                                }

                                info_dialog
                                    .present(Some(window.upcast_ref::<adw::ApplicationWindow>()));
                            },
                            Err(error) => {
                                window.show_toast(&error);
                            }
                        }
                    }
                ));
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

            let obj = self.obj();

            obj.setup_signals();
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
    // Setup signals
    //---------------------------------------
    fn setup_signals(&self) {
        let imp = self.imp();

        imp.fn_lock_toggle.connect_closure("error", false, closure_local!(
            #[weak(rename_to = window)] self,
            move |_: GramSettingWidget, error: &str| {
                window.show_toast(error);
            }
        ));

        imp.battery_limit_toggle.connect_closure("error", false, closure_local!(
            #[weak(rename_to = window)] self,
            move |_: GramSettingWidget, error: &str| {
                window.show_toast(error);
            }
        ));

        imp.usb_charge_toggle.connect_closure("error", false, closure_local!(
            #[weak(rename_to = window)] self,
            move |_: GramSettingWidget, error: &str| {
                window.show_toast(error);
            }
        ));

        imp.reader_mode_toggle.connect_closure("error", false, closure_local!(
            #[weak(rename_to = window)] self,
            move |_: GramSettingWidget, error: &str| {
                window.show_toast(error);
            }
        ));
    }

    //---------------------------------------
    // Init kernel features
    //---------------------------------------
    fn init_kernel_features(&self) {
        glib::spawn_future_local(clone!(
            #[weak(rename_to = window)] self,
            async move {
                let imp = window.imp();

                imp.fn_lock_toggle.init_id(FN_LOCK);

                imp.battery_limit_toggle.init_id(BATTERY_LIMIT);

                imp.usb_charge_toggle.init_id(USB_CHARGE);

                imp.reader_mode_toggle.init_id(READER_MODE);
            }
        ));
    }
}
