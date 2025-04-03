use crate::util::macros;

pub mod get_item;
pub mod get_keys;
pub mod remove_item;
pub mod set_item;

macros::routes! {
    load get_keys,
    load get_item,
    load set_item,
    load remove_item
}
