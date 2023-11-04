mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Now that this `pub use` has re-exported the hosting module
// from the root module, external code can now use the path
// `re_exporting_module::hosting::add_to_waitlist()`
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
