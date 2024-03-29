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
        <div class="flex flex-col items-center pb-4">
            <p class="text-lg">
                "Tokai Teio has been headpatted"
            </p>
            <p class="text-blue-500 text-2xl font-bold">
                {move ||
                    // Tried to find a better way to format numbers with commas, but I couldn't find one.
                    multiplayer_value.get()
                        .unwrap_or_default()
                        .to_string()
                        .as_bytes()
                        .rchunks(3)
                        .rev()
                        .map(std::str::from_utf8)
                        .collect::<Result<Vec<&str>, _>>()
                        .unwrap()
                        .join(",")
                }
            </p>
            <p class="text-lg pb-4">
                "times"
            </p>
            <button
                class="bg-[#3c6cdb] hover:bg-[#1a2c9f] text-white font-bold py-2 px-4 rounded-lg"
                on:click=move |_| inc.dispatch(())
            >
                "Headpat!"
            </button>
        </div>
    }
}