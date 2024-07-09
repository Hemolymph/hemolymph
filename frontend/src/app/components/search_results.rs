use crate::app::Route;
use crate::app::HOST;
use crate::app::{get_filegarden_link, modify_title, QueryResult};
use reqwest::Client;
use yew::html;
use yew::{function_component, suspense::use_future_with, HtmlResult, Properties};
use yew_router::components::Link;

#[derive(Properties, PartialEq, Eq)]
pub struct CardListProps {
    pub search: String,
}

#[function_component(SearchResults)]
pub fn search_results(CardListProps { search }: &CardListProps) -> HtmlResult {
    if search.trim().is_empty() {
        modify_title("");
    } else {
        modify_title("Searching");
    }
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
    match *result {
        QueryResult::CardList {
            ref query_text,
            ref content,
        } => {
            let a = content
                .iter()
                .map(|card| {
                    html! {
                        <Link<Route> to={Route::Card{id: card.id.clone()}}><img class="card-result" src={get_filegarden_link(&card.get_image_path(0))} /></Link<Route>>
                    }
                });

            Ok(html! {
                <>
                    <p id="query_readable">{"Showing "}{a.len()}{" "}{query_text}</p>
                    <div id="results" class="card-grid">
                        {for a}
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
