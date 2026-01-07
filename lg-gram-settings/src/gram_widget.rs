use std::cell::{RefCell, OnceCell};

use gtk::glib;
use adw::subclass::prelude::*;
use adw::prelude::*;

use crate::lg_gram::gram;

//------------------------------------------------------------------------------
// MODULE: GramWidget
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //---------------------------------------
    // Private structure
    //---------------------------------------
    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::GramWidget)]
    #[template(resource = "/com/github/LGGramSettings/ui/gram_widget.ui")]
    pub struct GramWidget {
        #[template_child]
        pub(super) icon: TemplateChild<gtk::Image>,

        #[property(get, set, nullable)]
        icon_name: RefCell<Option<String>>,
        #[property(get, set, nullable, default = Some("0"), construct)]
        off_value: RefCell<Option<String>>,
        #[property(get, set, nullable, default = Some("1"), construct)]
        on_value: RefCell<Option<String>>,

        pub(super) id: OnceCell<String>,
    }

    //---------------------------------------
    // Subclass
    //---------------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for GramWidget {
        const NAME: &'static str = "GramWidget";
        type Type = super::GramWidget;
        type ParentType = adw::ComboRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            // Gram set feature action
            klass.install_action_async("gram.set-feature", Some(glib::VariantTy::STRING),
                async |widget, _, param| {
                    let Some(id) = widget.imp().id.get() else {
                        widget.throw_error("ERROR: setting ID not initialized");
                        return
                    };

                    let Some(value) = param.and_then(|param| param.get::<String>()) else {
                        widget.throw_error("ERROR: failed to get variant value");
                        return
                    };

                    if let Err(error) = gram::set_feature_async(&id, &value).await {
                        widget.throw_error(&error);
                    }
                }
            );
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for GramWidget {
        //---------------------------------------
        // Constructor
        //---------------------------------------
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            obj.setup_widgets();
            obj.setup_signals();
        }
    }

    impl WidgetImpl for GramWidget {}
    impl ListBoxRowImpl for GramWidget {}
    impl PreferencesRowImpl for GramWidget {}
    impl ActionRowImpl for GramWidget {}
    impl ComboRowImpl for GramWidget {}
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: GramWidget
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct GramWidget(ObjectSubclass<imp::GramWidget>)
        @extends gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow, adw::ActionRow, adw::ComboRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable,
                    gtk::ConstraintTarget;
}

impl GramWidget {
    //---------------------------------------
    // Setup widgets
    //---------------------------------------
    fn setup_widgets(&self) {
        let imp = self.imp();

        self.bind_property("icon-name", &imp.icon.get(), "icon-name")
            .sync_create()
            .bidirectional()
            .build();
    }

    //---------------------------------------
    // Setup signals
    //---------------------------------------
    fn setup_signals(&self) {
        // Selected item property notify signal
        self.connect_selected_item_notify(|widget| {
            let id = widget.imp().id.get();

            if id.is_some() && let Some(item) = widget.selected_item()
                .and_downcast::<adw::EnumListItem>() {
                    let variant = item.value().to_string().to_variant();

                    widget.activate_action("gram.set-feature", Some(&variant)).unwrap();
                }

        });
    }

    //---------------------------------------
    // Helper function
    //---------------------------------------
    fn throw_error(&self, error: &str) {
        self.activate_action("win.show-error-toast", Some(&error.to_variant())).unwrap();
    }

    //---------------------------------------
    // Init function
    //---------------------------------------
    pub fn init(&self, id: &str, enum_type: glib::Type) {
        let model = adw::EnumListModel::new(enum_type);

        self.set_model(Some(&model));

        let active_index = gram::feature(id)
            .and_then(|value| {
                model.iter::<adw::EnumListItem>().flatten()
                    .position(|item| Ok(item.value()) == value.parse::<i32>())
                    .ok_or_else(|| String::from("unknown value"))
            });

        match active_index {
            Ok(index) => {
                self.set_selected(index as u32);

                self.imp().id.set(id.to_owned()).unwrap();

                self.set_sensitive(true);
            },
            Err(error) => {
                self.throw_error(&format!("Failed to read {id}: {error}"));
            }
        }
    }
}
