#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod components;

use components::card_details::CardDetails;
use components::search_results::SearchResults;
use std::collections::HashMap;
use yew_hooks::UseClipboardHandle;

use gloo_timers::callback::Timeout;
use hemoglobin::cards::Card;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::history::AnyHistory;
use yew_router::history::History;
use yew_router::history::MemoryHistory;
use yew_router::prelude::*;
use yew_router::Router;

// static QUERY: Mutex<String> = Mutex::new(String::new());
#[cfg(not(debug_assertions))]
static HOST: &'static str = "https://hemolymph.net";

#[cfg(debug_assertions)]
pub static HOST: &str = "http://127.0.0.1:8080";

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/:query")]
    Search { query: String },
    #[at("/card/:id/:index")]
    CardArt { id: String, index: usize },
    #[at("/card/:id")]
    Card { id: String },
    #[at("/howto")]
    Instructions,
}

#[derive(Deserialize, PartialEq)]
#[serde(tag = "type")]
enum QueryResult {
    CardList {
        query_text: String,
        content: Vec<Card>,
    },
    Error {
        message: String,
    },
}

#[cfg(target_arch = "wasm32")]
fn modify_title(title: &str) {
    let title = title.trim();
    let window = web_sys::window().expect("No window exists");
    let document = window.document().expect("No document on window");
    if title.is_empty() {
        document.set_title("Hemolymph");
    } else {
        document.set_title(&format!("{title} - Hemolymph"));
    }
}
#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_variables)]
const fn modify_title(title: &str) {}

fn get_ascii_titlecase(s: &str) -> String {
    let mut b = s.to_string();
    if let Some(r) = b.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    b
}

#[derive(Properties, PartialEq, Eq)]
struct SearchBarProps {
    force_text: AttrValue,
}

#[function_component(SearchBar)]
fn search_bar(properties: &SearchBarProps) -> Html {
    let nav = use_navigator().unwrap();
    let debounce_task = use_mut_ref::<Option<Timeout>, _>(|| None);
    let oninput = {
        Callback::from(move |e: InputEvent| {
            let nav = nav.clone();
            if let Some(task) = debounce_task.borrow_mut().take() {
                task.cancel();
            }
            let task = Timeout::new(500, move || {
                let input = e
                    .target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value();
                nav.replace(&Route::Search { query: input });
            });

            debounce_task.borrow_mut().replace(task);
        })
    };

    html! {
         <nav id="search">
            <Link<Route> to={Route::Search { query: String::new() }}><img id="logo" src="https://file.garden/ZJSEzoaUL3bz8vYK/hemolymphlogo.png" /></Link<Route>>
            <input id="search-bar" type="text" value={properties.force_text.clone()} placeholder="Type your search here. Search for () to see all cards." autofocus=true {oninput} />
            <Link<Route> to={Route::Instructions}><span>{"How To Use"}</span></Link<Route>>
        </nav>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <AnyApp/>
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <AnyApp/>
        </Router>
    }
}

#[function_component(AnyApp)]
pub fn any_app() -> Html {
    let force_text_str = use_state_eq(|| AttrValue::from(""));
    let force_text_str_other = force_text_str.clone();
    let force_text_fn: Callback<AttrValue> = Callback::from(move |name: AttrValue| {
        force_text_str_other.set(name);
    });
    let switch_real = move |route: Route| switch(&force_text_fn, route);
    html! {
        <>
            <SearchBar force_text={(*force_text_str).clone()} />
            <Switch<Route> render={switch_real} />
        </>
    }
}

fn switch(force_text_fn: &Callback<AttrValue>, route: Route) -> Html {
    let fallback = html! {<div><p class="suspense">{"Loading..."}</p></div>};
    match route {
        Route::Search { query } => {
            html! {<Suspense fallback={fallback}> <SearchResults search={query} force_text_fn={force_text_fn} /></Suspense>}
        }
        Route::Card { id } => {
            html! {<Suspense fallback={fallback}> <CardDetails card_id={id} img_index=0/> </Suspense>}
        }
        Route::CardArt { id, index } => {
            html! {<Suspense fallback={fallback}> <CardDetails card_id={id} img_index={index}/> </Suspense>}
        }
        Route::Instructions => {
            modify_title("How To");
            instructions()
        }
    }
}

fn instructions() -> Html {
    html! {
        <section id="instructions">
            <h2>{"How to use Hemolymph"}</h2>
            <p>{"Hemolymph is the arthropod equivalent of blood. It is also Bloodless' official card database and search engine."}</p>
            <div id="instructions-grid">
                <section class="instruction fuzzy_instr">
                    <h3>{"Fuzzy Search"}</h3>
                    <p>{"By default, your searches look for matches in Name, Kins, Keywords and Description, prioritizing them in that order."}</p>
                </section>
                <section class="instruction name_instr">
                    <h3>{"Name"}</h3>
                    <p>{"If you want to search by name only, you can write "}<span class="code">{"name:"}</span>{" or "}<span class="code">{"n:"}</span>{" before the name."}</p>
                    <p class="code">{"n:mantis"}</p>
                    <p>{"Surround the name in quotation marks if it contains spaces."}</p>
                    <p class="code">{"n:\"lost man\""}</p>
                </section>
                <section class="instruction kins_instr">
                    <h3>{"Kins, Types and Keywords"}</h3>
                    <p>{"You can use "}<span class="code">{"k:"}</span>{" for kins and "}<span class="code">{"kw:"}</span>{" for keywords. If you want to match more than one kin, they have to be separate. To search by type, use "} <span class="code">{"t:"}</span>{"."}</p>
                    <p class="code">{"k:ant kw:\"flying defense\" t:creature"}</p>
                </section>
                <section class="instruction stats_instr">
                    <h3>{"Stats"}</h3>
                    <p>{"You can use "}<span class="code">{"h: d: p:"}</span>{" and "}<span class="code">{"c:"}</span>{" for health, defense, power and strength, respectively. You can also match comparisons."}</p>
                    <p class="code">{"c=2 p>1 d<2 h!=1"}</p>
                </section>
                <section class="instruction devours">
                    <h3>{"Devours"}</h3>
                    <p>{"To look for cards that devour other cards, you use "}<span class="code">{"devours:"}</span>{" or "}<span class="code">{"dev:"}</span>{", which require a search query inside them, wrapped in parentheses."}</p>
                    <p class="code">{"devours:(cost=1)"}</p>
                </section>
                <section class="instruction devouredby">
                    <h3>{"Devoured By"}</h3>
                    <p>{"To look for cards that are devoured by other cards, you use "}<span class="code">{"devouredby:"}</span>{" or "}<span class="code">{"dby:"}</span>{", which require a search query inside them, wrapped in parentheses."}</p>
                    <p class="code">{"dby:(n:\"vampire mantis\")"}</p>
                </section>
                <section class="instruction fn_instr">
                    <h3>{"Functions"}</h3>
                    <p>{"To search based on things cards can be used for, use "}<span class="code">{"fn:"}</span>{". The spefifics of functions will be documented later, but right now you can, for example, search for "}<span class="code">{"fn:\"search deck\""}</span>{"."}</p>
                </section>
                <section class="instruction negation">
                    <h3>{"Negation"}</h3>
                    <p>{"You can invert a query's result by putting a dash before it. The following example matches all cards without \"mantis\" in their name."}</p>
                    <p class="code">{"-n:mantis"}</p>
                </section>
                <section class="instruction flavortext">
                    <h3>{"Flavor Text"}</h3>
                    <p>{"You can search by flavortext. The fuzzy match ignores flavor text."}</p>
                    <p class="code">{"flavortext:\"dr. vats\""}</p>
                </section>
                <section class="instruction orxor">
                    <h3>{"OR and XOR"}</h3>
                    <p>{"By default, your queries are AND-ed together, such that a card must match all restrictions. You can OR two restrictions together with OR and XOR"}</p>
                    <p class="code">{"p:0 OR t:command"}</p>
                </section>
                <section class="instruction group">
                    <h3>{"Instruction Groups"}</h3>
                    <p>{"You can group queries together by wrapping them in parentheses, and then perform operations on the whole query. For example, negating or OR-ing various matches"}</p>
                    <p class="code">{"(t:command k:sorcery) OR (t:creature)"}</p>
                </section>
                <section class="instruction sorting">
                    <h3>{"Sorting"}</h3>
                    <p>{"By default, your searches are sorted by their fuzzy match. If there was no fuzzy search, they are sorted by their name in alphabetical order. You can change the sorting with "}<span class="code">{"so:"}</span>{" to sort ascendingly and "}<span class="code">{"sod:"}</span>{" to sort descendingly. The following example sorts by flavor text in ascending alphabetical order."}</p>
                    <p class="code">{"so:ft"}</p>
                </section>
            </div>
        </section>
    }
}

fn get_filegarden_link(name: &str) -> String {
    format!(
        "https://file.garden/ZJSEzoaUL3bz8vYK/bloodlesscards/{}.png",
        name.replace(' ', "").replace('ä', "a")
    )
}

#[hook]
fn use_clipboard() -> Option<UseClipboardHandle> {
    let clipboard;
    #[cfg(target_arch = "wasm32")]
    {
        clipboard = Some(yew_hooks::use_clipboard());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        clipboard = None;
    }

    clipboard
}
