use yew::{function_component, html, Callback, Html, InputEvent, MouseEvent, Properties};
use yew_hooks::use_clipboard;
use yew_router::components::Link;

use crate::app::{get_filegarden_link, Route};

#[derive(Properties, PartialEq, Eq)]
pub struct CardThumbnailProps {
    pub id: String,
    pub image: String,
    pub art: usize,
    pub authors: Vec<String>,
}

#[function_component(CardThumbnail)]
pub fn card_thumbnail(
    CardThumbnailProps {
        id,
        image,
        art,
        authors,
    }: &CardThumbnailProps,
) -> Html {
    let authors = match authors.len() {
        0 => "Unspecified author".to_string(),
        1 => authors
            .first()
            .expect("Authors field was empty even though it verifiably was full")
            .to_string(),
        _ => format!(
            "{} et al",
            authors
                .first()
                .expect("Authors field was empty even though it verifiably was full")
        ),
    };

    let clipboard = use_clipboard();

    let image_clone = image.to_owned();

    let copy_id = Callback::from(move |_: MouseEvent| clipboard.write_text(image_clone.clone()));
    if *art == 0 {
        html! {
            <div class="card-alt-view">
                <span class="art-author">{authors}</span>
                <Link<Route> to={Route::Card{id: id.clone()}}><img class="card-result" src={get_filegarden_link(image)} /></Link<Route>>
                <button onclick={copy_id}>{"Copy Marrow ID"}</button>
            </div>
        }
    } else {
        html! {
            <div class="card-alt-view">
                <span class="art-author">{authors}</span>
                <Link<Route> to={Route::CardArt{ id: id.clone(), index: *art }}><img class="card-result" src={get_filegarden_link(image)} /></Link<Route>>
                <button onclick={copy_id}>{"Copy Marrow ID"}</button>
            </div>
        }
    }
}
