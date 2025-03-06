use crate::routes;

pub mod get_item;
pub mod get_keys;
pub mod set_item;

routes! {
    load get_keys,
    load get_item,
    load set_item
}
