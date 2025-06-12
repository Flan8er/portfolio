use leptos::prelude::*;
use web_sys::MouseEvent;

use crate::components::page::Page;

#[component]
pub fn Appendix() -> impl IntoView {
    view! {
        <Page>
            <div
                class="w-full h-full flex flex-col items-center justify-evenly relative"
            >
                <div class="flex">
                    <div class="min-w-[100px] w-[100px]"/>
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

                <div class="w-full flex flex-col md:flex-row gap-4 justify-center items-center mb-12">
                    <ProjectFrame title="Waveform Animation" link="waveform" image="waveform.png"/>
                    <ProjectFrame title="Reentry Simulation" link="reentry" image="reentry_simulation.png"/>
                    <ProjectFrame title="Galaga" link="galaga" image="galaga_gameplay.png"/>
                </div>
            </div>
        </Page>
    }
}

#[component]
pub fn ProjectFrame(title: &'static str, link: &'static str, image: &'static str) -> impl IntoView {
    let show_link = RwSignal::new(false);

    view! {
        <div class="flex flex-col rounded-xl p-2 bg-accent w-[250px] h-[250px] items-center justify-between gap-2">
            <div
                class="w-full h-full overflow-hidden relative"
                on:mouseenter=move |_: MouseEvent| show_link.set(true)
                on:mouseleave=move |_: MouseEvent| show_link.set(false)
            >
                <Show
                    when=move || show_link.get()
                    fallback=|| view! {<></>}
                >
                    <div class="absolute inset-0 flex items-center justify-center backdrop-blur-[16px] backdrop-saturate-[180%] rounded-[16px]">
                        <a
                            href=format!("/{}", link)
                            target="_blank"
                            rel="noopener noreferrer"
                            class="px-4 py-2 bg-foreground rounded-[10px] text-accent"
                        >
                            "View"
                        </a>
                    </div>
                </Show>

                <img
                    src=format!("img/{}", image)
                    alt=""
                    class="w-full h-full object-cover rounded-[16px]"
                />
            </div>

            <h3>{title}</h3>
        </div>
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
