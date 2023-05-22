use cfg_if::cfg_if;
mod counters;
mod app;
mod gallery;

// boilerplate to run in different modes
cfg_if! {
    // server-only stuff
    if #[cfg(feature = "ssr")] {
        use leptos::*;
        use actix_files::{Files};
        use actix_web::*;
        use crate::counters::*;
        use crate::app::*;
        use leptos_actix::{generate_route_list, LeptosRoutes};

        #[get("/style.css")]
        async fn css() -> impl Responder {
            actix_files::NamedFile::open_async("./src/styles/output.css").await
        }

        #[get("/pixiv_proxy/{path:.*}")]
        async fn proxy(path: web::Path<String>) -> impl Responder {
            let url = format!("https://i.pximg.net/{}", path);

            let client = reqwest::Client::new();

            let res = client
                .get(&url)
                .header(
                    "Referer".to_string(),
                    "https://www.pixiv.net/".to_string(),
                )
                .send()
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap();

            HttpResponse::Ok()
                .body(res)
        }

        #[get("/api/events")]
        async fn counter_events() -> impl Responder {
            use futures::StreamExt;

            let stream =
                futures::stream::once(async { crate::counters::get_server_count().await.unwrap_or(0) })
                    .chain(COUNT_CHANNEL.clone())
                    .map(|value| {
                        Ok(web::Bytes::from(format!(
                            "event: message\ndata: {value}\n\n"
                        ))) as Result<web::Bytes>
                    });
            HttpResponse::Ok()
                .insert_header(("Content-Type", "text/event-stream"))
                .streaming(stream)
        }

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {

            crate::counters::register_server_functions();

            // Setting this to None means we'll be using cargo-leptos and its env vars.
            // when not using cargo-leptos None must be replaced with Some("Cargo.toml")
            let conf = get_configuration(None).await.unwrap();

            let addr = conf.leptos_options.site_addr;
            let routes = generate_route_list(|cx| view! { cx, <App/> });

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = &routes;


                App::new()
                    .service(css)
                    .service(proxy)
                    .service(counter_events)
                    .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                    .leptos_routes(leptos_options.to_owned(), routes.to_owned(), |cx| view! { cx, <App/> })
                    .service(Files::new("/", site_root))
                    // .wrap(middleware::Compress::default())
            })
            .bind(&addr)?
            .run()
            .await
        }
        }

    // client-only main for Trunk
    else {
        pub fn main() {
            // isomorphic counters cannot work in a Client-Side-Rendered only
            // app as a server is required to maintain state
        }
    }
}