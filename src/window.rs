use gtk::{gio, glib};
use adw::subclass::prelude::*;

use crate::Application;

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
        // #[template_child]
        // pub(super) sidebar_breakpoint: TemplateChild<adw::Breakpoint>,
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
        // fn constructed(&self) {
        //     self.parent_constructed();

        //     let obj = self.obj();

        //     obj.setup_dialogs();
        //     obj.setup_signals();

        //     obj.bind_gsettings();

        //     obj.setup_widgets();

        //     obj.setup_alpm(true);

        //     obj.setup_inotify();
        // }
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
