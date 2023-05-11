use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[server(IncrementCounter, "/api/increment")]
pub async fn increment_counter() -> Result<String, ServerFnError> {
    Ok("Incremented counter".to_string())
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/teiomoe.css"/>

        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        // sets the document title
        <Title text="Tokai Teio | teio.moe"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <main class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to teio.moe"</h2>
            <p class="px-10 pb-10 text-center">"Tokai Tei is blazingly fast, just like this website"</p>
            <button
                class="bg-amber-600 hover:bg-violet-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                "Something's here | "
                {move || if count() == 0 {
                    "Click me!".to_string()
                } else {
                    count().to_string()
                }}
                " | Some more text"
            </button>
            <button
                class="bg-amber-600 hover:bg-violet-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| {
                    spawn_local(async {
                        let _ = increment_counter().await;
                    });
                }>
                "Increment counter"
            </button>
        </main>
    }
}
