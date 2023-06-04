use leptos::*;

#[component]
pub fn Gallery(cx: Scope) -> impl IntoView {

    view! { cx,
        <h2 class="text-3xl pb-6 text-center">"Tokai Teio Picture Gallery"</h2>

        <div class="grid-msnry"></div>

        <script>
            {r#"
                var elem = document.querySelector('.grid-msnry');
                var msnry;

                function load_images() {
                    var req = new XMLHttpRequest();
                    req.onreadystatechange = function() {
                        if (this.readyState == 4 && this.status == 200) {
                            var data = JSON.parse(this.responseText);

                            // randomly sorts data array to show images in random order
                            data.sort(() => Math.random() - 0.5);

                            var contentWidth = window.innerWidth * 0.8
                            var placeholderWidth = 0
    
                            if (window.innerWidth <= 500) {
                                placeholderWidth = contentWidth - 10;
                            } else if (window.innerWidth <= 854) {
                                placeholderWidth = contentWidth * 0.5 - 10;
                            } else if (window.innerWidth <= 1280) {
                                placeholderWidth = contentWidth / 3 - 10;
                            } else if (window.innerWidth <= 1920) {
                                placeholderWidth = contentWidth * 0.25 - 10;
                            } else if (window.innerWidth <= 2240) {
                                placeholderWidth = contentWidth * 0.2 - 10;
                            } else {
                                placeholderWidth = contentWidth / 6 - 10;
                            }

                            for (const image of data) {
                                var img_container = document.createElement('div');
                                img_container.classList.add('grid-item');

                                img_container.classList.add('overflow-hidden');
                                img_container.classList.add('rounded-lg');
                
                                var img_link = document.createElement('a');
                                img_link.href = image.image_link;
                                img_link.target = '_blank';

                                var img = document.createElement('img');
                                img.classList.add('object-cover');
                                img.src = image.image_source;
                                img.loading = 'lazy';
                                img.width = placeholderWidth;
                                img.height = placeholderWidth;
                                img.classList.add('custom-img');
                                img.addEventListener("load", function() {
                                    msnry.layout();
                                });

                                img_container.appendChild(img_link);
                                img_link.appendChild(img);

                                elem.appendChild(img_container);
                            }

                            msnry = new Masonry( elem, {
                                itemSelector: '.grid-item',
                                percentPosition: true
                            });
                        }
                    };
                    req.open("GET", "/images.json");
                    req.send();
                }

                window.onload = load_images();
            "#}
        </script>

    }
}