use crate::app::components::card_grid::CardThumbnail;
use hemoglobin::cards::rich_text::RichElement;
use hemoglobin::cards::{rich_text::RichString, Card};
use reqwest::Client;
use yew::suspense::use_future_with;
use yew::{function_component, html, Callback, Html, HtmlResult, MouseEvent, Properties};
use yew_hooks::use_clipboard;
use yew_router::components::Link;

use crate::app::HOST;
use crate::app::{get_ascii_titlecase, get_filegarden_link, modify_title, Route};

#[derive(Properties, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct CardDetailsProps {
    pub card_id: String,
    pub img_index: usize,
}

#[derive(Eq, PartialEq)]
enum CardDetailsErr {
    NotACard,
    BadResponse,
}

#[function_component(CardDetails)]
pub fn card_details(CardDetailsProps { card_id, img_index }: &CardDetailsProps) -> HtmlResult {
    let card = use_future_with(card_id.to_owned(), |card_id| async move {
        let client = Client::new();
        let url = format!("{HOST}/api/card?id={card_id}");
        if let Ok(response) = client.get(&url).send().await {
            (response.json::<Card>().await).map_or(Err(CardDetailsErr::NotACard), Ok)
        } else {
            Err(CardDetailsErr::BadResponse)
        }
    })?;

    let clipboard = use_clipboard();

    match *card {
        Err(CardDetailsErr::NotACard) => Ok(html! {
            <div>
                <p>{"Error: Server sent something that is not a card"}</p>
            </div>
        }),
        Err(CardDetailsErr::BadResponse) => Ok(html! {
            <div>
                <p>{"Error: Server couldn't be reached"}</p>
            </div>
        }),
        Ok(ref card) => {
            let name = &card.name;
            let description: Html = render_rich_string(&card.description);
            let r#type = &card.r#type;
            let img = card.get_image_path(*img_index);
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
            let alts = card.images.iter().enumerate().map(|(idx, image)| {
                html! {
                    <CardThumbnail id={card.id.clone()} image={card.get_image_path(idx)} art={idx} authors={image.authors.clone()}/>
                }
            });

            modify_title(name);

            let image_id_clone = img.clone();
            let copy_id =
                Callback::from(move |_: MouseEvent| clipboard.write_text(image_id_clone.clone()));

            Ok(html! {
                <div id="details-view">
                    <div id="details">
                        <img id="details-preview" src={get_filegarden_link(&img)} />
                        <div id="text-description">
                            <h1 id="details-title">{name.clone()}</h1>
                            <hr />
                            <p id="cost-line">{get_ascii_titlecase(r#type)} if !r#type.contains("blood flask") {{" :: "} {cost} {" Blood"}}</p>
                            <hr />
                            {description}
                            if !flavor_text.is_empty() {
                                <hr />
                                { for flavor_text }
                            }
                            if !r#type.contains("command") {
                                <hr />
                                <p id="stats-line">{health}{"/"}{defense}{"/"}{power}</p>
                            }
                            <button onclick={copy_id}>{"Copy Marrow ID"}</button>
                        </div>
                    </div>
                    if card.images.len() > 1 {
                        <h2 class="center-text">{"Alternate Artwork"}</h2>
                        <div id="card-alts" class="card-grid">
                            {for alts}
                        </div>
                    }
                </div>
            })
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
                    RichElement::CardId { display, identity: _ } => html! {{display}},
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
