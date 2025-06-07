use leptos::prelude::*;

use crate::components::page::Page;

#[component]
pub fn Appendix() -> impl IntoView {
    view! {
        <Page>
            <div
                class="w-full h-full flex items-center justify-center relative"
            >
                <div class="max-w-6xl w-full px-8 grid md:grid-cols-2 gap-12 items-center z-10">
                    <div>
                        <p class="text-lg text-gray-300 leading-relaxed mb-6">
                            "I just want to say a thank you if you've made it this far!
                            Software develpment is a real passion of mine and it's great to be able to share it
                            with other like-minded people."

                            <ThirdWall/>
                        </p>
                    </div>

                    <div class="flex justify-center items-center glass rounded-md">
                        <img
                            src="img/this.png"
                            alt=""
                            class="w-full h-full object-cover z-10
                                   transform transition-transform duration-700 ease-out
                                   scale-100 hover:scale-105"
                        />
                    </div>
                </div>
            </div>
        </Page>
    }
}

#[component]
pub fn ThirdWall() -> impl IntoView {
    view! {
        <p class="text-lg text-gray-300 leading-relaxed mb-6">
            <br/>

            "This entire app was built in Leptos — if you like, feel free to checkout the source code "

            <a
                href="https://github.com/Flan8er/portfolio"
                target="_blank"
                rel="noopener noreferrer"
                class="text-blue-500"
            >
                "here"
            </a>

            "."
        </p>
    }
}
