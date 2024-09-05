use crate::app::Route;
use crate::app::HOST;
use crate::app::{get_filegarden_link, modify_title, QueryResult};
use reqwest::Client;
use yew::html;
use yew::AttrValue;
use yew::Callback;
use yew::MouseEvent;
use yew::{function_component, suspense::use_future_with, HtmlResult, Properties};
use yew_hooks::use_clipboard;
use yew_router::components::Link;

#[derive(Properties, PartialEq)]
pub struct CardListProps {
    pub search: AttrValue,
    pub force_text_fn: Callback<AttrValue>,
}

#[function_component(SearchResults)]
pub fn search_results(
    CardListProps {
        search,
        force_text_fn,
    }: &CardListProps,
) -> HtmlResult {
    if search.trim().is_empty() {
        modify_title("");
    } else {
        modify_title("Searching");
    }
    force_text_fn.emit(search.clone());
    let result = use_future_with(search.clone(), |search| async move {
        let client = Client::new();
        let url = format!("{HOST}/api/search?query={}", search.clone());
        match client.get(&url).send().await {
            Ok(response) => match response.json::<QueryResult>().await {
                Ok(queryres) => queryres,
                Err(err) => QueryResult::Error {
                    message: format!("Obtained a malformed response: \n{err:#?}"),
                },
            },
            Err(err) => QueryResult::Error {
                message: format!("Couldn't get a response from the server. {err}"),
            },
        }
    })?;
    let clipboard = use_clipboard();
    match *result {
        QueryResult::CardList {
            ref query_text,
            ref content,
        } => {
            let cards = content
                .iter()
                .map(|card| {
                    let clipboard = clipboard.clone();
                    let image_id = card.get_image_path(0);
                    let image_id_clone = image_id.clone();
                    let copy_id = Callback::from(move |_: MouseEvent| clipboard.write_text(image_id_clone.clone()));
                    html! {
                        <div class="card_result">
                            <Link<Route> to={Route::Card{id: card.id.clone()}}><img class="card-result" src={get_filegarden_link(&image_id)} /></Link<Route>>
                            <button onclick={copy_id}>{"Copy Marrow ID"}</button>
                        </div>
                    }
                });

            Ok(html! {
                <>
                    <p id="query_readable">{"Showing "}{cards.len()}{" "}{query_text}</p>
                    <div id="results" class="card-grid">
                        {for cards}
                    </div>
                </>
            })
        }
        QueryResult::Error { ref message } => Ok(html! {
            <div id="search-error">
                <p><b>{"ERROR:"}</b>{message}</p>
            </div>
        }),
    }
}
