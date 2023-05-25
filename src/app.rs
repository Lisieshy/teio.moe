use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::counters::*;
use crate::gallery::*;

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

        <script src="https://unpkg.com/masonry-layout@4/dist/masonry.pkgd.min.js"></script>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> } />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <main>
            <div class="my-0 mx-auto max-w-3xl text-center">
                <h2 class="p-6 text-4xl">"Welcome to teio.moe"</h2>
                <p class="px-10 text-center">"Tokai Teio is blazingly fast, just like this website"</p>
                <Counter />
                <hr class="py-6 dashed-test" />
            </div>
            <Gallery />
        </main>
    }
}
