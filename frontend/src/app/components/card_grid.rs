use yew::{function_component, html, Html, Properties};
use yew_router::components::Link;

use crate::app::{get_filegarden_link, Route};

#[derive(Properties, PartialEq, Eq)]
pub struct CardThumbnailProps {
    pub id: String,
    pub image: String,
    pub art: usize,
}

#[function_component(CardThumbnail)]
pub fn card_thumbnail(CardThumbnailProps { id, image, art }: &CardThumbnailProps) -> Html {
    if *art == 0 {
        html! {
            <Link<Route> to={Route::Card{id: id.clone()}}><img class="card-result" src={get_filegarden_link(image)} /></Link<Route>>
        }
    } else {
        html! {
            <Link<Route> to={Route::CardArt{ id: id.clone(), index: *art }}><img class="card-result" src={get_filegarden_link(image)} /></Link<Route>>
        }
    }
}
