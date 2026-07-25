pub mod components;
pub mod pages;

pub enum UiEvents {
    ComponentEnums(components::ComponentEvents),
    PageEvents(pages::PageEvents),
}
