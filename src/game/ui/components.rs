pub mod banner;
pub mod button;

pub enum ComponentEvents {
    ButtonEvents(button::ButtonEvents),
}
