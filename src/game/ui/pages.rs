use home_page::HomePage;
use crate::game::{rendering::{Drawable, StateUpdatable}, interaction::{Interactive, SelfEventHandler}};

pub mod home_page;


pub enum Pages {
    HomePage(HomePage),
}

pub enum PageEvents {
    HomePageEvent(home_page::HomePageEvent),
}

pub struct PageManager {
    current_page: Pages,
}

impl PageManager {
    pub fn new() -> Self {
        Self {
            current_page: Pages::HomePage(HomePage::new()),
        }
    }
}

impl Drawable for PageManager{
    fn draw(&self) {
        match &self.current_page {
            Pages::HomePage(home_page)=>{
                home_page.draw();
            }
        }
    }
}

impl StateUpdatable<()> for PageManager {
    fn update_state(&mut self, data: ()) {
        match &mut self.current_page {
            Pages::HomePage(home_page)=>{
                home_page.update_state(data);
            }
        }
    }
}

impl Interactive for PageManager {
    type Event = Option<PageEvents>;
    fn poll_event(&self) -> Self::Event {
        match &self.current_page {
            Pages::HomePage(home_page) =>{
                home_page.poll_event().map(|event| PageEvents::HomePageEvent(event))
            }
        }
    }
}
impl SelfEventHandler for PageManager {
    fn handle_self_event(&mut self, _event: Self::Event) {
        match &mut self.current_page {
            Pages::HomePage(home_page) => {
                let home_page_event = home_page.poll_event();
                home_page.handle_self_event(home_page_event);
            }
        }
    }
}