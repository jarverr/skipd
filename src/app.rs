use std::str::FromStr;

#[cfg(feature = "ssr")]
use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use leptos::{ev::MouseEvent, logging, prelude::*, reactive::spawn_local};
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
        let raw = event_target_value(&ev);

        let command = match Command::from_str(&raw) {
            Ok(cmd) => cmd,
            Err(err) => {
                logging::error!("Invalid command from  UI: {err}");
                return;
            }
        };

        spawn_local(async {
            if let Err(err) = controller(command).await {
                logging::error!("Controller failed {err}")
            }
        })
    };

    view! {
        <section>
        <h1>"SkipD"</h1>
        <div class="controls">
            <button on:click=on_click value="LeftArrow">
                <img class="icon" src="/backward-svgrepo-com.svg" alt="Back"/>
            </button>
            <button on:click=on_click value="Space">
                <img class="icon" src="/play-svgrepo-com.svg" alt="Play"/>
            </button>
            <button on:click=on_click value="RightArrow">
                <img class="icon icon-rotated" src="/backward-svgrepo-com.svg" alt="Next"/>
            </button>
        </div>
        <div class="volume">
            <button on:click=on_click value="VolumeDown">
                <img class="icon" src="/volume-min-svgrepo-com.svg" alt="Volume down"/>
            </button>
            <button on:click=on_click value="VolumeMute">
                <img class="icon" src="/volume-xmark-svgrepo-com.svg" alt="Mute"/>
            </button>
            <button on:click=on_click value="VolumeUp">
                <img class="icon" src="/volume-max-svgrepo-com.svg" alt="Volume up"/>
            </button>
        </div>
        </section>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Command {
    LeftArrow,
    RightArrow,
    Space,
    VolumeMute,
    VolumeUp,
    VolumeDown,
}

#[cfg(feature = "ssr")]
impl From<Command> for Key {
    fn from(command: Command) -> Self {
        match command {
            Command::LeftArrow => Key::LeftArrow,
            Command::RightArrow => Key::RightArrow,
            Command::Space => Key::Space,
            Command::VolumeDown => Key::VolumeDown,
            Command::VolumeUp => Key::VolumeUp,
            Command::VolumeMute => Key::VolumeMute,
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "LeftArrow" => Ok(Command::LeftArrow),
            "RightArrow" => Ok(Command::RightArrow),
            "Space" => Ok(Command::Space),
            "VolumeMute" => Ok(Command::VolumeMute),
            "VolumeUp" => Ok(Command::VolumeUp),
            "VolumeDown" => Ok(Command::VolumeDown),
            _ => Err(format!("Unknown command: {value}")),
        }
    }
}

#[server]
async fn controller(command: Command) -> Result<(), ServerFnError> {
    let key = Key::from(command);
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(key, Click).expect("Enige keypress");

    Ok(())
}
