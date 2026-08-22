mod home_page;
mod level_selection_page;
mod ship_selection_page;
mod gameplay_page;

use crate::game::{
    game_config::GameConfig, interaction::{Interactive, SelfEventHandler}, rendering::{Drawable, StateUpdatable}, ui::page_manager::{
        gameplay_page::GameplayPage, 
        home_page::{HomePage,HomePageEvent}, 
        level_selection_page::{LevelSelectionPage, LevelSelectionPageEvent}, 
        ship_selection_page::{ShipSelectionPage, ShipSelectionPageEvent}
    },
};

const LOG_PREFIX: &str = "[pages]";

pub enum Pages {
    HomePage(HomePage),
    ShipSelectionPage(ShipSelectionPage),
    LevelSelectionPage(LevelSelectionPage),
    GameplayPage(GameplayPage)
}

pub enum PageEvents {
    HomePageEvent(home_page::HomePageEvent),
    ShipSelectionPageEvent(ship_selection_page::ShipSelectionPageEvent),
    LevelSelectionPageEvent(level_selection_page::LevelSelectionPageEvent),
    GamplayPageEvent(gameplay_page::GamplayPageEvent)
}

pub struct PageManager {
    current_page: Pages,
}

impl Default for PageManager {
    fn default() -> Self {
        Self::new()
    }
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
            Pages::ShipSelectionPage(ship_selection_page) => {
                ship_selection_page.draw();
            }
            Pages::LevelSelectionPage(level_selection_page) => {
                level_selection_page.draw();
            }
            Pages::GameplayPage(gameplay_page) =>{
                gameplay_page.draw();
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
            Pages::ShipSelectionPage(ship_selection_page) => {
                ship_selection_page.update_state(data);
            }
            Pages::LevelSelectionPage(level_selection_page) => {
                level_selection_page.update_state(data);
            }
            Pages::GameplayPage(gameplay_page)=>{
                gameplay_page.update_state(data);
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
                .map(PageEvents::HomePageEvent),
            Pages::ShipSelectionPage(ship_selection_page) => ship_selection_page
                .poll_event()
                .map(PageEvents::ShipSelectionPageEvent),
            Pages::LevelSelectionPage(level_selection_page) => level_selection_page
                .poll_event()
                .map(PageEvents::LevelSelectionPageEvent),
            Pages::GameplayPage(gameplay_page)=> gameplay_page
                .poll_event()
                .map(PageEvents::GamplayPageEvent),
        }
    }
}
impl SelfEventHandler for PageManager {
    fn handle_self_event(&mut self, _event: Self::Event) {
        match &mut self.current_page {
            Pages::HomePage(home_page) => {
                let home_page_event = home_page.poll_event();
                home_page.handle_self_event(home_page_event);
                match home_page_event {
                    Some(HomePageEvent::NewGame) => {
                        println!("{LOG_PREFIX}[HomePage] New Game clicked");
                        self.current_page = Pages::ShipSelectionPage(ShipSelectionPage::new())
                    }
                    Some(HomePageEvent::Exit) => {
                        println!("{LOG_PREFIX}[HomePage] Exit clicked. Exiting...");
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            Pages::ShipSelectionPage(ship_selection_page) => {
                let ship_selection_page_event = ship_selection_page.poll_event();
                ship_selection_page.handle_self_event(ship_selection_page_event);
                match ship_selection_page_event {
                    Some(ShipSelectionPageEvent::ShipConfirmed(kind)) => {
                        println!("{LOG_PREFIX}[ShipSelectionPage] {kind:?} confirmed");
                        self.current_page = Pages::LevelSelectionPage(LevelSelectionPage::new())
                    }
                    Some(ShipSelectionPageEvent::BackButtonPressed) => {
                        println!("{LOG_PREFIX}[ShipSelectionPage] Back clicked");
                        self.current_page = Pages::HomePage(HomePage::new())
                    }
                    None => {}
                }
            }
            Pages::LevelSelectionPage(level_selection_page) => {
                let level_selection_page_event = level_selection_page.poll_event();
                level_selection_page.handle_self_event(level_selection_page_event);

                match level_selection_page_event {
                    Some(LevelSelectionPageEvent::LevelConfirmed(level)) => {
                        println!("{LOG_PREFIX}[LevelSelectionPage] Level {level:#?} confirmed");
                        self.current_page= Pages::GameplayPage(GameplayPage::new(GameConfig::default()));
                    }
                    Some(LevelSelectionPageEvent::BackButtonPressed) => {
                        println!("{LOG_PREFIX}[LevelSelectionPage] Back clicked");
                        self.current_page = Pages::ShipSelectionPage(ShipSelectionPage::new())
                    }
                    None => {}
                }
            }
            Pages::GameplayPage(gameplay_page) =>{
                let gameplay_page_event = gameplay_page.poll_event();
                gameplay_page.handle_self_event(gameplay_page_event);

                match gameplay_page_event {
                    _ =>{}
                }
            }
        }
    }
}
