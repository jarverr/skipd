use leptos::{prelude::*, reactive::spawn_local};
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
    let on_click_arrow = move |_| {
        spawn_local(async {
            controller_command(Command::RightArrow)
                .await
                .expect("towork")
        })
    };
    let on_click_space =
        move |_| spawn_local(async { controller_command(Command::Space).await.expect("towork") });

    view! {
        <section>
        <h1>"SkipD"</h1>
        <button on:click=on_click_arrow>"rightarrow"</button>
        <button on:click=on_click_space>"space"</button>
        </section>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Command {
    RightArrow,
    Space,
}

#[server]
async fn controller_command(command: Command) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};

    let key = match command {
        Command::RightArrow => Key::RightArrow,
        Command::Space => Key::Space,
    };

    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(key, Click).expect("Enige keypress");

    Ok(())
}
