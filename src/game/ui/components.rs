pub mod banner;
pub mod button;
pub mod icon_label_button;

pub enum ComponentEvents {
    ButtonEvents(button::ButtonEvents),
}
