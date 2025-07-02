use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

// use thaw::*;
// use thaw::ssr::SSRMountStyleProvider;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        //<SSRMountStyleProvider>
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
        //</SSRMountStyleProvider>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/thaw-demo.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

// #[server(GetServerData, "/api")]
// async fn get_server_data() -> Result<String, ServerFnError> {
//     Ok("data from server".to_string())
// }

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // let (data, set_data) = signal(String::new());
    // Effect::new(move |_| spawn_local(async move {
    //     let data = get_server_data().await.unwrap();
    //     set_data.set(data);
    // }));

    view! {
        <div>
        // <p>{move || data.get()}</p>
        </div>
    }
}
