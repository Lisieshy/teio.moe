use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::sync::atomic::{AtomicU64, Ordering};
        use broadcaster::BroadcastChannel;
        use std::fs;

        lazy_static::lazy_static! {
            pub static ref COUNT: AtomicU64 = AtomicU64::new(fs::read_to_string("count.txt").ok().and_then(|x| x.parse::<u64>().ok()).unwrap_or(0));
            pub static ref COUNT_CHANNEL: BroadcastChannel<u64> = BroadcastChannel::new();
        }

        pub fn register_server_functions() {
            _ = GetServerCount::register();
            _ = AdjustServerCount::register();
        }
    }
}

#[server(GetServerCount, "/api")]
pub async fn get_server_count() -> Result<u64, ServerFnError> {
    Ok(COUNT.load(Ordering::Relaxed))
}

#[server(AdjustServerCount, "/api")]
pub async fn adjust_server_count(
    delta: u64,
) -> Result<u64, ServerFnError> {
    let new = COUNT.load(Ordering::Relaxed) + delta;
    COUNT.store(new, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&new).await;
    fs::write("count.txt", new.to_string()).expect("Unable to write file");
    Ok(new)
}

#[component]
pub fn Counter(cx: Scope) -> impl IntoView {
    let inc = create_action(cx, |_| adjust_server_count(1));

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
    let (multiplayer_value, _) = create_signal(cx, None::<u64>);


    view! { cx,
        <div class="pb-16 pt-8">
            <span>
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
        // </main>
    }
}