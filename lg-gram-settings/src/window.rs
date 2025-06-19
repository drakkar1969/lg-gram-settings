use gtk::{gio, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;
use glib::{clone, closure_local};

use crate::Application;
use crate::gram_widget::GramWidget;
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
        pub(super) reader_mode_widget: TemplateChild<GramWidget>,
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

            // Show system information action async
            klass.install_action_async("win.show-system-info-async", None, async |window, _, _| {
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
                        window.show_toast(&error);
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

        imp.battery_limit_widget.connect_closure("error", false, closure_local!(
            #[weak(rename_to = window)] self,
            move |_: GramWidget, error: &str| {
                window.show_toast(error);
            }
        ));

        imp.fn_lock_widget.connect_closure("error", false, closure_local!(
            #[weak(rename_to = window)] self,
            move |_: GramWidget, error: &str| {
                window.show_toast(error);
            }
        ));

        imp.usb_charge_widget.connect_closure("error", false, closure_local!(
            #[weak(rename_to = window)] self,
            move |_: GramWidget, error: &str| {
                window.show_toast(error);
            }
        ));

        imp.reader_mode_widget.connect_closure("error", false, closure_local!(
            #[weak(rename_to = window)] self,
            move |_: GramWidget, error: &str| {
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

                imp.battery_limit_widget.init_id(BATTERY_LIMIT);

                imp.fn_lock_widget.init_id(FN_LOCK);

                imp.usb_charge_widget.init_id(USB_CHARGE);

                imp.reader_mode_widget.init_id(READER_MODE);
            }
        ));
    }
}
