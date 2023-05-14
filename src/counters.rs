use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::sync::atomic::{AtomicI32, Ordering};
        use broadcaster::BroadcastChannel;
        use std::fs;

        // static COUNT: AtomicI32 = AtomicI32::new(fs::read_to_string("count.txt").unwrap_or("0".to_string()).parse::<i32>().unwrap_or(0));

        lazy_static::lazy_static! {
            pub static ref COUNT: AtomicI32 = AtomicI32::new(fs::read_to_string("count.txt").ok().and_then(|x| x.parse::<i32>().ok()).unwrap_or(0));
            pub static ref COUNT_CHANNEL: BroadcastChannel<i32> = BroadcastChannel::new();
        }

        pub fn register_server_functions() {
            _ = GetServerCount::register();
            _ = AdjustServerCount::register();
            _ = ClearServerCount::register();
        }

    }
}

// "/api" is an optional prefix that allows you to locate server functions wherever you'd like on the server
#[server(GetServerCount, "/api")]
pub async fn get_server_count() -> Result<i32, ServerFnError> {
    Ok(COUNT.load(Ordering::Relaxed))
}

#[server(AdjustServerCount, "/api")]
pub async fn adjust_server_count(
    delta: i32,
) -> Result<i32, ServerFnError> {
    let new = COUNT.load(Ordering::Relaxed) + delta;
    COUNT.store(new, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&new).await;
    fs::write("count.txt", new.to_string()).expect("Unable to write file");
    // println!("message = {:?}", msg);
    Ok(new)
}

#[server(ClearServerCount, "/api")]
pub async fn clear_server_count() -> Result<i32, ServerFnError> {
    COUNT.store(0, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&0).await;
    Ok(0)
}
#[component]
pub fn Counters(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! { cx,
        <Stylesheet id="leptos" href="/pkg/teiomoe.css"/>

        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        <Title text="Tokai Teio | teio.moe"/>

        <MultiuserCounter/>
    }
}

// This is a kind of "multi-user" counter
// It relies on a stream of server-sent events (SSE) for the counter's value
// Whenever another user updates the value, it will update here
// This is the primitive pattern for live chat, collaborative editing, etc.
#[component]
pub fn MultiuserCounter(cx: Scope) -> impl IntoView {
    // let dec =
    //     create_action(cx, |_| adjust_server_count(-1, "dec dec goose".into()));
    let inc =
        create_action(cx, |_| adjust_server_count(1));
    // let clear = create_action(cx, |_| clear_server_count());

    #[cfg(not(feature = "ssr"))]
    let multiplayer_value = {
        use futures::StreamExt;

        let mut source =
            gloo_net::eventsource::futures::EventSource::new("/api/events")
                .expect("couldn't connect to SSE stream");
        let s = create_signal_from_stream(
            cx,
            source
                .subscribe("message")
                .unwrap()
                .map(|value| match value {
                    Ok(value) => value
                        .1
                        .data()
                        .as_string()
                        .expect("expected string value"),
                    Err(_) => "0".to_string(),
                }),
        );

        on_cleanup(cx, move || source.close());
        s
    };

    #[cfg(feature = "ssr")]
    let (multiplayer_value, _) = create_signal(cx, None::<i32>);

    view! { cx,
        <main class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Teio.moe"</h2>
            <div>
                // <button on:click=move |_| clear.dispatch(())>"Clear"</button>
                // <button on:click=move |_| dec.dispatch(())>"-1"</button>
                <span class="px-10 pb-10 text-center">
                    "Total headpats: " {move || multiplayer_value.get().unwrap_or_default()}
                </span>
                <br/>
                <button
                    class="bg-amber-600 hover:bg-violet-700 px-5 py-3 text-white rounded-lg"
                    on:click=move |_| inc.dispatch(())
                >
                    "headpat"
                </button>
            </div>
        </main>
    }
}