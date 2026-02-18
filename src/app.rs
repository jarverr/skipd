use std::str::FromStr;

#[cfg(feature = "ssr")]
use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use leptos::{ev::MouseEvent, prelude::*, reactive::spawn_local};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use serde::{Deserialize, Serialize};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/skipd.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main class="container">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let on_click = move |ev: MouseEvent| {
        let value = Command::from_str(&event_target_value(&ev)).expect("Command to come out");

        spawn_local(async { controller_command(value).await.expect("towork") })
    };

    view! {
        <section>
        <h1>"SkipD"</h1>
        <button on:click=on_click value="RightArrow" >"rightarrow"</button>
        <button on:click=on_click value="Space" >"space"</button>
        </section>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Command {
    RightArrow,
    Space,
}

#[cfg(feature = "ssr")]
impl From<Command> for Key {
    fn from(command: Command) -> Self {
        match command {
            Command::RightArrow => Key::RightArrow,
            Command::Space => Key::Space,
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "RightArrow" => Ok(Command::RightArrow),
            "Space" => Ok(Command::Space),
            _ => Err(()),
        }
    }
}

#[server]
async fn controller_command(command: Command) -> Result<(), ServerFnError> {
    let key = Key::from(command);
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(key, Click).expect("Enige keypress");

    Ok(())
}
