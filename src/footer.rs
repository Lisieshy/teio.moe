use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {

    view! { cx,
        <footer class="bg-white dark:bg-gray-900 mt-4">
            <div class="mx-auto w-full max-w-screen-xl p-4 py-6 lg:py-8">
                <div class="md:flex md:justify-between">
                    <div class="mb-6 md:mb-0">
                        <a href="https://teio.moe/" class="flex items-center mb-6">
                            <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">"Teio.moe"</span>
                        </a>
                        <ul class="text-gray-600 dark:text-gray-400 font-medium pr-4">
                            <li class="mb-4">
                                <p>"Gallery code originally made by "<a href="https://github.com/KawaiiShadowii" target="_blank" class="hover:underline">"KawaiiShadowii"</a>" for "<a href="https://tannhauser.moe/" target="_blank" class="hover:underline">"tannhauser.moe"</a></p>
                            </li>
                            <li class="mb-4">
                                <p>"If you have any feedback or suggestions, don't hesitate to contact me!"</p>
                            </li>
                            <li class="mb-4">
                                <p>"All images belong to their respective authors. If you want your image removed, please contact me."</p>
                            </li>
                        </ul>
                    </div>
                    <div class="grid grid-cols-2 gap-8 sm:gap-6 sm:grid-cols-3">
                        <div>
                            <h2 class="mb-6 text-sm font-semibold text-gray-900 uppercase dark:text-white">"Contact me"</h2>
                            <ul class="text-gray-600 dark:text-gray-400 font-medium">
                                <li class="mb-4">
                                    <a href="https://github.com/Lisieshy" target="_blank" class="hover:underline ">"Github"</a>
                                </li>
                                <li class="mb-4">
                                    <a href="https://catgirl.fr/@Lisieshy" rel="me" target="_blank"  class="hover:underline">"Fediverse"</a>
                                </li>
                                <li class="mb-4">
                                    <div class="flex items-center space-x-2">
                                        <svg class="w-5 h-5" fill="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 127.14 96.36"><g id="图层_2" data-name="图层 2"><g id="Discord_Logos" data-name="Discord Logos"><g id="Discord_Logo_-_Large_-_White" data-name="Discord Logo - Large - White"><path class="cls-1" d="M107.7,8.07A105.15,105.15,0,0,0,81.47,0a72.06,72.06,0,0,0-3.36,6.83A97.68,97.68,0,0,0,49,6.83,72.37,72.37,0,0,0,45.64,0,105.89,105.89,0,0,0,19.39,8.09C2.79,32.65-1.71,56.6.54,80.21h0A105.73,105.73,0,0,0,32.71,96.36,77.7,77.7,0,0,0,39.6,85.25a68.42,68.42,0,0,1-10.85-5.18c.91-.66,1.8-1.34,2.66-2a75.57,75.57,0,0,0,64.32,0c.87.71,1.76,1.39,2.66,2a68.68,68.68,0,0,1-10.87,5.19,77,77,0,0,0,6.89,11.1A105.25,105.25,0,0,0,126.6,80.22h0C129.24,52.84,122.09,29.11,107.7,8.07ZM42.45,65.69C36.18,65.69,31,60,31,53s5-12.74,11.43-12.74S54,46,53.89,53,48.84,65.69,42.45,65.69Zm42.24,0C78.41,65.69,73.25,60,73.25,53s5-12.74,11.44-12.74S96.23,46,96.12,53,91.08,65.69,84.69,65.69Z"/></g></g></g></svg>
                                        <p class="hover:underline">"@lisieshy"</p>
                                    </div>
                                </li>
                            </ul>
                        </div>
                        <div>
                            <h2 class="mb-6 text-sm font-semibold text-gray-900 uppercase dark:text-white">"Feedback"</h2>
                            <ul class="text-gray-600 dark:text-gray-400 font-medium">
                                <li class="mb-4">
                                    <a href="https://github.com/Lisieshy/teio.moe" target="_blank"  class="hover:underline">"Teio.moe repository"</a>
                                </li>
                                <li class="mb-4">
                                    <a href="https://github.com/Lisieshy/teio.moe/issues" target="_blank"  class="hover:underline">"Teio.moe feedback"</a>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
                <hr class="my-6 border-gray-200 sm:mx-auto dark:border-gray-700 lg:my-8" />
                <div class="sm:flex sm:items-center sm:justify-between">
                    <span class="text-sm text-gray-500 sm:text-center dark:text-gray-400">"Made with ❤️ & 🦀 by "<a href="https://lisieshy.dev/" target="_blank" class="hover:underline">"Lisieshy"</a></span>
                    <div class="flex mt-4 space-x-4 sm:justify-center sm:mt-0">
                        <a href="https://github.com/Lisieshy/" target="_blank"  class="text-gray-500 hover:text-gray-900 dark:hover:text-white">
                            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd" /></svg>
                            <span class="sr-only">"My GitHub account"</span>
                        </a>
                        <a href="https://catgirl.fr/@Lisieshy" target="_blank"  class="text-gray-500 hover:text-gray-900 dark:hover:text-white">
                            <svg viewBox="1.95 0.97 128 128" class="w-5 h-5" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.w3.org/2000/svg"><linearGradient id="a" gradientTransform="rotate(90)"><stop offset="5%" stop-color="#9ccfd8" style="--darkreader-inline-stopcolor:#265760"/><stop offset="95%" stop-color="#31748f" style="--darkreader-inline-stopcolor:#275d72"/></linearGradient><defs><linearGradient xlink:href="#a" id="f" gradientTransform="scale(1.27567 .7839)" x1="-43.77" y1="98.469" x2="-27.05" y2="137.466" gradientUnits="userSpaceOnUse"/><linearGradient xlink:href="#a" id="d" x1="0" y1="0" x2="1" y2="0" gradientUnits="userSpaceOnUse"/><linearGradient xlink:href="#a" id="e" gradientTransform="scale(1.27567 .7839)" x1="-43.77" y1="98.468" x2="-8.156" y2="98.468" gradientUnits="userSpaceOnUse"/><linearGradient xlink:href="#a" id="c" gradientTransform="scale(1.27567 .7839)" x1="1.571" y1="1.27" x2="133.179" y2="1.27" gradientUnits="userSpaceOnUse"/><linearGradient xlink:href="#a" id="b" gradientTransform="scale(1.27567 .7839)" x1="1.571" y1="1.27" x2="133.179" y2="1.27" gradientUnits="userSpaceOnUse"/></defs><g style="fill:url(#b)" transform="translate(.934 25.196) scale(.75646)"><g style="fill:url(#c)" fill="url(#a)" word-spacing="0" letter-spacing="0" font-family="'OTADESIGN Rounded'" font-weight="400"><g transform="translate(-55.341 -52.023) scale(.26953)" style="fill:url(#d)"/><g style="fill:url(#e)"><path style="fill:url(#f)" d="M-41.832 77.19c-3.868 0-7.177 1.358-9.93 4.074-2.716 2.752-4.074 6.063-4.074 9.931 0 3.869 1.358 7.04 4.074 9.793 2.753 2.716 6.064 4.073 9.932 4.073 3.831 0 7.122-1.357 9.875-4.073.855-.855 1.283-1.896 1.283-3.123 0-1.228-.428-2.271-1.283-3.127-.856-.855-1.897-1.281-3.123-1.281-1.229 0-2.27.426-3.125 1.281-1.004 1.042-2.213 1.563-3.627 1.563-3.035-.31-5.208-2.263-5.246-5.106.038-2.842 2.21-4.935 5.244-5.246 1.414 0 2.623.52 3.627 1.563.855.855 1.898 1.283 3.127 1.283 1.226 0 2.267-.428 3.123-1.283.855-.856 1.283-1.897 1.283-3.125 0-1.227-.428-2.268-1.283-3.123-2.753-2.716-6.046-4.075-9.877-4.075zm20.902 6.91c-2.88 0-5.353 1.02-7.422 3.06-.642.643-.964 1.426-.964 2.348 0 .923.322 1.706.964 2.35.644.642 1.427.962 2.348.962.924 0 1.707-.32 2.35-.963.754-.783 1.662-1.173 2.724-1.173 1.09 0 2.026.376 2.809 1.13a3.909 3.909 0 0 1 1.135 2.811c0 1.062-.393 1.97-1.176 2.725-.392.419-.868.7-1.426.84-.141.027-.252.012-.336-.044-.056-.084-.028-.168.084-.251l.84-.881c.643-.643.965-1.411.965-2.305 0-.922-.28-1.663-.838-2.223-.559-.559-1.343-.84-2.35-.84-.698 0-1.397.35-2.095 1.05l-4.866 4.822c-.643.643-.964 1.426-.964 2.347 0 .923.321 1.705.964 2.348 1.957 1.93 4.375 2.894 7.254 2.894 2.908 0 5.396-1.034 7.465-3.103 2.041-2.041 3.06-4.5 3.06-7.379 0-2.907-1.019-5.396-3.06-7.465-2.069-2.04-4.557-3.06-7.465-3.06z" transform="translate(208.34 -284.25) scale(3.6954)" clip-rule="evenodd" fill-rule="evenodd"/></g></g></g></svg>
                            <span class="sr-only">"My Fediverse account"</span>
                        </a>
                    </div>
                </div>
            </div>
        </footer>
    }
}