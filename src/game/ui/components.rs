pub mod button;
pub mod title;

pub enum ComponentEvents{
    ButtonEvents(button::Events),
}