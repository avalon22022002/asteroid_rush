pub mod home_page;

use home_page::{HomePage};

pub enum Pages{
    HomePage(HomePage)
}

pub enum PageEvents{
    HomePageEvent(home_page::HomePageEvent)
}