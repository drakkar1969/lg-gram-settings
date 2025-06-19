use gtk::{gio, glib};
use adw::prelude::*;
use adw::subclass::prelude::*;

use crate::window::MainWindow;

//------------------------------------------------------------------------------
// MODULE: Application
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //---------------------------------------
    // Private structure
    //---------------------------------------
    #[derive(Default)]
    pub struct Application;

    //---------------------------------------
    // Subclass
    //---------------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Application";
        type Type = super::Application;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for Application {
        //---------------------------------------
        // Constructor
        //---------------------------------------
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().setup_actions();
        }
    }

    impl ApplicationImpl for Application {
        //---------------------------------------
        // Activate handler
        //---------------------------------------
        fn activate(&self) {
            let application = self.obj();

            // Show main window
            let window = application.active_window().map_or_else(|| {
                MainWindow::new(&application).upcast()
            }, |window| window);

            window.present();
        }
    }

    impl GtkApplicationImpl for Application {}
    impl AdwApplicationImpl for Application {}
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: Application
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Application {
    //---------------------------------------
    // New function
    //---------------------------------------
    pub fn new(application_id: &str, flags: gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    //---------------------------------------
    // Setup actions
    //---------------------------------------
    fn setup_actions(&self) {
        // Quit action
        let quit_action = gio::ActionEntry::builder("quit-app")
            .activate(move |app: &Self, _, _| app.quit())
            .build();

        // Show about dialog action
        let about_action = gio::ActionEntry::builder("show-about")
            .activate(move |app: &Self, _, _| {
                let window = app.active_window()
                    .expect("Failed to retrieve active window");

                let about_dialog = adw::AboutDialog::builder()
                    .application_name("LG Gram Settings")
                    .application_icon("lg-gram-settings")
                    .developer_name("draKKar1969")
                    .version(env!("CARGO_PKG_VERSION"))
                    .website("https://github.com/drakkar1969/lg-gram")
                    .copyright("© 2025 draKKar1969")
                    .license_type(gtk::License::Gpl30)
                    .build();

                about_dialog.add_link("LG Gram Laptop Extra Features", "https://www.kernel.org/doc/html/latest/admin-guide/laptops/lg-laptop.html");

                about_dialog.present(Some(&window));
            })
            .build();

        // Add actions to app
        self.add_action_entries([quit_action, about_action]);

        // Add app keyboard shortcuts
        self.set_accels_for_action("app.quit-app", &["<ctrl>Q"]);
    }
}
