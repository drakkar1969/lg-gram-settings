use std::cell::{Cell, RefCell};
use std::marker::PhantomData;

use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::prelude::ObjectExt;

//------------------------------------------------------------------------------
// MODULE: BatteryLimitObject
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //-----------------------------------
    // Private structure
    //-----------------------------------
    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::BatteryLimitObject)]
    pub struct BatteryLimitObject {
        #[property(get, set)]
        value: Cell<u32>,
        #[property(get = Self::label)]
        label: PhantomData<String>,
        #[property(get, set)]
        description: RefCell<String>,
    }

    //-----------------------------------
    // Subclass
    //-----------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for BatteryLimitObject {
        const NAME: &'static str = "BatteryLimitObject";
        type Type = super::BatteryLimitObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for BatteryLimitObject {}

    impl BatteryLimitObject {
        fn label(&self) -> String {
            format!("{}%", self.value.get())
        }
    }
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: BatteryLimitObject
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct BatteryLimitObject(ObjectSubclass<imp::BatteryLimitObject>);
}
