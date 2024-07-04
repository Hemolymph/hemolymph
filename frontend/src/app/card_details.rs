use hemoglobin::cards::rich_text::RichElement;
use hemoglobin::cards::{rich_text::RichString, Card};
use rand::seq::SliceRandom;
use reqwest::Client;
use yew::suspense::use_future;
use yew::{function_component, hook, html, use_effect, use_state_eq, Html, HtmlResult, Properties};
use yew_router::components::Link;

use crate::app::{get_ascii_titlecase, get_filegarden_link, modify_title, run_future, Route};
use crate::app::{HOST, PORT};

#[derive(Properties, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct CardDetailsProps {
    pub card_id: String,
}

#[derive(Eq, PartialEq)]
enum CardDetailsErr {
    NotACard,
    BadResponse,
}

#[function_component(CardDetails)]
pub fn card_details(CardDetailsProps { card_id }: &CardDetailsProps) -> Html {
    let card_id = card_id.to_owned();
    let card = use_state_eq(|| None);
    let card1 = card.clone();
    use_effect(move || {
        let card = card1.clone();
        run_future(async move {
            let client = Client::new();
            let url = format!("http://{HOST}:{PORT}/api/card?id={}", card_id.clone());
            if let Ok(response) = client.get(&url).send().await {
                card.set(Some(
                    (response.json::<Card>().await).map_or(Err(CardDetailsErr::NotACard), Ok),
                ));
            } else {
                card.set(Some(Err(CardDetailsErr::BadResponse)));
            }
        });
    });

    match *card {
        None => html! {<p>{"Loading"}</p>},
        Some(Err(CardDetailsErr::NotACard)) => html! {
            <div>
                <p>{"Error: Server sent something that is not a card"}</p>
            </div>
        },
        Some(Err(CardDetailsErr::BadResponse)) => html! {
            <div>
                <p>{"Error: Server couldn't be reached"}</p>
            </div>
        },
        Some(Ok(ref card)) => {
            let name = &card.name;

            let description: Html = render_rich_string(&card.description);

            let r#type = &card.r#type;

            let img = &card.img;

            let img = img.choose(&mut rand::thread_rng()).unwrap_or(name);

            let cost = &card.cost;
            let health = &card.health;
            let defense = &card.defense;
            let power = &card.power;
            let flavor_text: Vec<Html> = card
                .flavor_text
                .lines()
                .filter(|x| !x.is_empty())
                .map(|x| html! {<p class="flavor-line">{x}</p>})
                .collect();

            modify_title(name);

            html! {
                <div id="details-view">
                    <div id="details">
                        <img id="details-preview" src={get_filegarden_link(img)} />
                        <div id="text-description">
                            <h1 id="details-title">{name.clone()}</h1>
                            <hr />
                            <p id="cost-line">{get_ascii_titlecase(r#type)} if !r#type.contains("blood flask") {{" :: "} {cost} {" Blood"}}</p>
                            <hr />
                            {description}
                            if !flavor_text.is_empty() {
                                <hr />
                                {for flavor_text}
                            }
                            if !r#type.contains("command") {
                                <hr />
                                <p id="stats-line">{health}{"/"}{defense}{"/"}{power}</p>
                            }
                        </div>
                    </div>
                </div>
            }
        }
    }
}

fn render_rich_string(string: &RichString) -> Html {
    let mut paragraphs = vec![];
    for element in string {
        match element {
            RichElement::String(string) => {
                if paragraphs.is_empty() {
                    paragraphs.push(vec![]);
                }

                let lines = &mut string.lines();
                if let Some(x) = lines.next().filter(|x| !x.is_empty()) {
                    paragraphs
                        .last_mut()
                        .unwrap()
                        .push(RichElement::String(x.to_string()));
                }

                for line in lines.filter(|x| !x.is_empty()) {
                    paragraphs.push(vec![RichElement::String(line.to_string())]);
                }
            }
            el @ (RichElement::CardId {
                display: _,
                identity: _,
            }
            | RichElement::SpecificCard { display: _, id: _ }
            | RichElement::CardSearch {
                display: _,
                search: _,
            }) => match paragraphs.last_mut() {
                Some(last) => last.push(el.clone()),
                None => paragraphs.push(vec![el.clone()]),
            },
            el @ RichElement::Saga(_) => paragraphs.push(vec![el.clone()]),
            RichElement::LineBreak => paragraphs.push(vec![]),
        }
    }

    paragraphs
        .iter()
        .map(|x| {
            let x: Html = x
                .iter()
                .map(|x| match x {
                    RichElement::String(string) => html! {{string}},
                    RichElement::CardId { display, identity } => html! {{display}},
                    RichElement::SpecificCard { display, id } => {
                        html! {<Link<Route> to={Route::Card{id: id.clone()}}>{display}</Link<Route>>}
                    }
                    RichElement::CardSearch { display, search } => html! {<Link<Route> to={Route::Search{query: search.clone()}}>{display}</Link<Route>>},
                    RichElement::Saga(list) => {
                        let list: Vec<Html> = list
                            .iter()
                            .map(|x| html! {<li>{render_rich_string(x)}</li>})
                            .collect();
                        html! {<ol>{list}</ol>}
                    }
                    RichElement::LineBreak => html! {<br />},
                })
                .collect();
            html! { <p>{x}</p> }
        })
        .collect()
}
