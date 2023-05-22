use leptos::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Images {
    image: String,
    link: String,
    artist: String,
}

#[derive(Error, Clone, Debug)]
pub enum FetchError {
    #[error("Error while loading data.")]
    Request,
    #[error("Error while deserializing data from request.")]
    Json,
}

async fn fetch_images_data() -> Result<Vec<Images>, FetchError> {
    let res = reqwasm::http::Request::get(&format!(
        "/images.json",
    ))
    .send()
    .await
    .map_err(|_| FetchError::Request)?
    .json::<Vec<Images>>()
    .await
    .map_err(|_| FetchError::Json)?
    .into_iter()
    .collect::<Vec<_>>();
    // log!("res: {:?}", res);
    Ok(res)
}

#[component]
pub fn Gallery(cx: Scope) -> impl IntoView {

    let images = create_local_resource(
        cx,
        || (),
        |_| async move { fetch_images_data().await }
    );


    let images_view = move || {
        images.read(cx).map(|data| {
            data.map(|data| {
                data.iter()
                    .map(|item| view! { cx,
                        <div class="h-min w-full">
                            // <p>{item.artist.clone()}</p>
                            <img class="object-cover rounded-lg" src=item.image.clone() />
                        </div>
                    })
                    .collect_view(cx)
            })
        })
    };

    view! { cx,
        <h2 class="text-3xl text-center">"Images"</h2>
        <div class="sm:columns-1 md:columns-2 lg:columns-3 xl:columns-4 2xl:columns-5 gap-4 space-y-4 p-8">
        <ErrorBoundary
            fallback=move |cx, errors| view! { cx,
                <div>
                    <h1>"Something went wrong"</h1>
                    <ul>
                        {move || errors.get()
                            .into_iter()
                            .map(|(_, e)| view! { cx, <li>{e.to_string()}</li> })
                            .collect_view(cx)
                        }
                    </ul>
                </div>
            }
        >
            <Transition fallback=move || {
                view! { cx, <div>"Loading..."</div> }
            }>
                {images_view}
            </Transition>
        </ErrorBoundary>
            // <div class="h-min w-full">
            //     <img class="object-cover rounded-lg" src="94445214_p0.jpg" />
            // </div>
            // <div class="h-min w-full">
            //     <img class="object-cover rounded-lg" src="94445214_p0.jpg" />
            // </div>
            // <div class="h-min w-full">
            //     <img class="object-cover rounded-lg" src="94445214_p0.jpg" />
            // </div>
            // <div class="h-min w-full">
            //     <img class="object-cover rounded-lg" src="94445214_p0.jpg" />
            // </div>
            // <div class="h-min w-full">
            //     <img class="object-cover rounded-lg" src="94445214_p0.jpg" />
            // </div>
        </div>
    }
}