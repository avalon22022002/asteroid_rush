pub mod components;
pub mod page_manager;

pub enum UiEvents {
    ComponentEnums(components::ComponentEvents),
    PageEvents(page_manager::PageEvents),
}
