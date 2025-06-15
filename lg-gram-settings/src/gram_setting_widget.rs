use std::cell::{Cell, RefCell, OnceCell};
use std::sync::OnceLock;

use gtk::glib;
use adw::subclass::prelude::*;
use adw::prelude::*;
use glib::clone;
use glib::subclass::Signal;

use crate::lg_gram::gram;

//------------------------------------------------------------------------------
// MODULE: GramSettingWidget
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //---------------------------------------
    // Private structure
    //---------------------------------------
    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::GramSettingWidget)]
    #[template(resource = "/com/github/LG-GramSettings/ui/gram_setting_widget.ui")]
    pub struct GramSettingWidget {
        #[template_child]
        pub(super) icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub(super) toggle_group: TemplateChild<adw::ToggleGroup>,
        #[template_child]
        pub(super) off_toggle: TemplateChild<adw::Toggle>,
        #[template_child]
        pub(super) on_toggle: TemplateChild<adw::Toggle>,
        #[template_child]
        pub(super) persistent_button: TemplateChild<gtk::ToggleButton>,

        #[property(get, set, nullable)]
        icon_name: RefCell<Option<String>>,
        #[property(get, set, nullable, default = Some("0"), construct)]
        off_value: RefCell<Option<String>>,
        #[property(get, set, nullable, default = Some("1"), construct)]
        on_value: RefCell<Option<String>>,

        pub(super) id: OnceCell<String>,

        pub(super) is_feature_reverting: Cell<bool>,
        pub(super) is_persistent_reverting: Cell<bool>,
    }

    //---------------------------------------
    // Subclass
    //---------------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for GramSettingWidget {
        const NAME: &'static str = "GramSettingWidget";
        type Type = super::GramSettingWidget;
        type ParentType = adw::ActionRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            // Gram set feature action async
            klass.install_action_async("gram.set-feature-async",
                Some(&String::static_variant_type()),
                async |widget, _, parameter| {
                    if let Some(id) = parameter.and_then(|param| param.get::<Vec<String>>())
                        .and_then(|params| params.first().cloned())
                    {
                        let imp = widget.imp();

                        if imp.is_feature_reverting.get() {
                            imp.is_feature_reverting.set(false);
                            return
                        }

                        let group = imp.toggle_group.get();

                        if let Ok(value) = group.toggle(group.active())
                            .and_then(|toggle| toggle.label())
                            .ok_or_else(|| String::from("Error: no valid selection"))
                        {
                            let result = gram::set_feature_async(&id, &value).await;

                            if let Err(error) = result {
                                imp.is_feature_reverting.set(true);
                                widget.invert_toggle_group();

                                widget.emit_error_signal(&error);
                            } else if group.active() == 0 {
                                imp.persistent_button.set_active(false);
                            }
                        }

                        imp.persistent_button.set_sensitive(group.active() != 0);
                    }
                }
            );

            // Gram enable service action async
            klass.install_action_async("gram.enable-service-async",
                Some(&String::static_variant_type()),
                async |widget, _, parameter| {
                    if let Some(id) = parameter.and_then(|param| param.get::<Vec<String>>())
                        .and_then(|params| params.first().cloned())
                    {
                        let imp = widget.imp();

                        if imp.is_persistent_reverting.get() {
                            imp.is_persistent_reverting.set(false);
                            return
                        }

                        let button = imp.persistent_button.get();

                        let value = u32::from(button.is_active());

                        if let Err(error) = gram::enable_service_async(&id, value).await {
                            imp.is_persistent_reverting.set(true);
                            button.set_active(!button.is_active());

                            widget.emit_error_signal(&error);
                        }
                    }
                }
            );
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for GramSettingWidget {
        //---------------------------------------
        // Signals
        //---------------------------------------
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("error")
                        .param_types([String::static_type()])
                        .build()
                ]
            })
        }

        //---------------------------------------
        // Constructor
        //---------------------------------------
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().setup_widgets();
        }
    }

    impl WidgetImpl for GramSettingWidget {}
    impl ListBoxRowImpl for GramSettingWidget {}
    impl PreferencesRowImpl for GramSettingWidget {}
    impl ActionRowImpl for GramSettingWidget {}
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: GramSettingWidget
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct GramSettingWidget(ObjectSubclass<imp::GramSettingWidget>)
        @extends gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow, adw::ActionRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable,
                    gtk::ConstraintTarget;
}

impl GramSettingWidget {
    //---------------------------------------
    // Setup widgets
    //---------------------------------------
    fn setup_widgets(&self) {
        let imp = self.imp();

        self.bind_property("icon-name", &imp.icon.get(), "icon-name")
            .sync_create()
            .bidirectional()
            .build();

        self.bind_property("off-value", &imp.off_toggle.get(), "label")
            .sync_create()
            .bidirectional()
            .build();

        self.bind_property("on-value", &imp.on_toggle.get(), "label")
            .sync_create()
            .bidirectional()
            .build();
    }

    //---------------------------------------
    // Helper functions
    //---------------------------------------
    fn invert_toggle_group(&self) {
        let toggle_group = &self.imp().toggle_group;

        toggle_group.set_active(1 - toggle_group.active());
    }

    fn emit_error_signal(&self, error: &str) {
        self.emit_by_name::<()>("error", &[&error]);
    }

    //---------------------------------------
    // Setup signals
    //---------------------------------------
    fn setup_signals(&self) {
        let imp = self.imp();

        // Activated signal
        self.connect_activated(|widget| {
            widget.invert_toggle_group();
        });

        // Toggle group active property notify signal
        imp.toggle_group.connect_active_notify(clone!(
            #[weak(rename_to = widget)] self,
            move |_| {
                let id = widget.imp().id.get();

                if id.is_none() {
                    widget.emit_error_signal("Error: setting ID not initialized");
                    return
                }

                widget.activate_action("gram.set-feature-async", Some(&id.to_variant()))
                    .unwrap();
            }
        ));

        // Persistent button toggled signal
        imp.persistent_button.connect_toggled(clone!(
            #[weak(rename_to = widget)] self,
            move |_| {
                let id = widget.imp().id.get();

                if id.is_none() {
                    widget.emit_error_signal("Error: setting ID not initialized");
                    return
                }

                widget.activate_action("gram.enable-service-async", Some(&id.to_variant()))
                    .unwrap();
            }
        ));
    }

    //---------------------------------------
    // Init ID function
    //---------------------------------------
    pub fn init_id(&self, id: &str) {
        let imp = self.imp();

        let active_index = gram::feature(id)
            .and_then(|value| {
                let toggles = imp.toggle_group.toggles();

                toggles.iter::<glib::Object>()
                    .filter_map(|obj| {
                        obj.ok()
                            .and_downcast::<adw::Toggle>()
                            .and_then(|toggle| toggle.label())
                    })
                    .position(|s| s == value)
                    .ok_or_else(|| String::from("unknown value"))
            });

        match &active_index {
            Ok(index) => {
                imp.toggle_group.set_active(*index as u32);

                match gram::is_service_enabled(id) {
                    Ok(state) => {
                        imp.persistent_button.set_sensitive(*index != 0);
                        imp.persistent_button.set_active(state);

                        imp.id.set(id.to_owned()).unwrap();

                        self.setup_signals();
                    },
                    Err(error) => {
                        imp.persistent_button.set_sensitive(false);

                        self.emit_error_signal(&format!("Failed to load {id} service status: {error}"));
                    }
                }
            },
            Err(error) => {
                imp.toggle_group.set_sensitive(false);
                imp.persistent_button.set_sensitive(false);

                self.emit_error_signal(&format!("Failed to read {id} value: {error}"));
            }
        }
    }
}
