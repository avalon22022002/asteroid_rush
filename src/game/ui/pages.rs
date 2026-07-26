use crate::game::{
    interaction::{Interactive, SelfEventHandler},
    rendering::{Drawable, StateUpdatable},
    ui::pages::{home_page::HomePageEvent, level_selection_page::LevelSelectionPage},
};
use home_page::HomePage;

pub mod home_page;
pub mod level_selection_page;

pub enum Pages {
    HomePage(HomePage),
    LevelSelectionPage(LevelSelectionPage),
}

pub enum PageEvents {
    HomePageEvent(home_page::HomePageEvent),
    LevelSelectionPageEvent(level_selection_page::LevelSelectionPageEvent),
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

impl Drawable for PageManager {
    fn draw(&self) {
        match &self.current_page {
            Pages::HomePage(home_page) => {
                home_page.draw();
            }
            Pages::LevelSelectionPage(level_selection_page) => {
                level_selection_page.draw();
            }
        }
    }
}

impl StateUpdatable<()> for PageManager {
    fn update_state(&mut self, data: ()) {
        match &mut self.current_page {
            Pages::HomePage(home_page) => {
                home_page.update_state(data);
            }
            Pages::LevelSelectionPage(level_selection_page) => {
                level_selection_page.update_state(data);
            }
        }
    }
}

impl Interactive for PageManager {
    type Event = Option<PageEvents>;
    fn poll_event(&self) -> Self::Event {
        match &self.current_page {
            Pages::HomePage(home_page) => home_page
                .poll_event()
                .map(|event| PageEvents::HomePageEvent(event)),
            Pages::LevelSelectionPage(level_selection_page) => level_selection_page
                .poll_event()
                .map(|event| PageEvents::LevelSelectionPageEvent(event)),
        }
    }
}
impl SelfEventHandler for PageManager {
    fn handle_self_event(&mut self, _event: Self::Event) {
        match &mut self.current_page {
            Pages::HomePage(home_page) => {
                let home_page_event = home_page.poll_event();
                home_page.handle_self_event(home_page_event);
                if let Some(HomePageEvent::NewGame) = home_page_event {
                    self.current_page = Pages::LevelSelectionPage(LevelSelectionPage::new())
                }
            }
            Pages::LevelSelectionPage(_level_selection_page) => {}
        }
    }
}
