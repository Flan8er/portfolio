//
use leptos::prelude::*;

use crate::components::{navigator::Navigator, page::Page};

#[component]
pub fn TwentyTwentyThree() -> impl IntoView {
    let photos: [String; 7] = [
        String::from("Reveal1.png"),
        String::from("Reveal2.png"),
        String::from("Reveal3.png"),
        String::from("Reveal4.png"),
        String::from("Reveal5.png"),
        String::from("Reveal6.png"),
        String::from("Reveal7.png"),
    ];

    let active_photo: RwSignal<u8> = RwSignal::new(0);

    view! {
        <Page>
            <div
                class="w-full h-full flex items-center justify-center relative bg-gradient-to-b from-[rgba(201,42,148,0.125)] to-[rgba(183,46,176,0.15)]"
            >
                <div class="max-w-6xl w-full px-8 grid md:grid-cols-2 gap-12 items-center z-10">
                    <div>
                        <h2 class="text-5xl font-bold mb-8 leading-tight bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
                            "The Big \"Reveal\""
                        </h2>

                        <p class="text-lg text-gray-300 leading-relaxed mb-6">
                            "After 5 iterations in just as many languages and frameworks, I became eager to work on something other than a workout tracking app. "
                            "At this point, I was working a full-time job as a Mechanical Engineer sticking my nose into any software opportunities that would come up, "
                            "but I felt I could do more to lay a path to becoming a full-time software developer. So, after hours I would come home to build my own software company. "
                            "Looking back I had such naive ambition - stay tuned as that's a reoccurring theme - but after several months I had created something I could hang my hat on. "
                        </p>

                        <p class="text-lg text-gray-300 leading-relaxed">
                            "My goal was to create a "
                            <a
                                href="https://setapp.com"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-blue-500"
                            >"Setapp"</a>
                            "-esque company that focussed on making no-code builders for popular NPM packages. The thinking was that not everybody wants to learn "
                            "how to develop the latest-and-greatest package, but doing just that was something that fueled me. I had my sights on "
                            <a
                                href="https://d3js.org"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-blue-500"
                            >"D3"</a>
                            ", "
                            <a
                                href="https://nodered.org"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-blue-500"
                            >"Node-RED"</a>
                            ", and "
                            <a
                                href="https://threejs.org"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-blue-500"
                            >"three.js"</a>
                            ", but decided to first build a no-code editor for "
                            <a
                                href="https://revealjs.com"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-blue-500"
                            >"Reveal.js"</a>
                            ". Taking stylistic influence from Apple's approach towards UI/UX design - built on Rust using the Tauri framework with a React frontend: Reveal."
                        </p>

                    </div>

                    <div class="w-[600px] h-[500px] relative flex flex-col items-center justify-between rounded-md overflow-hidden">
                        <div class="absolute inset-0">
                            <img
                                src={
                                    let photos = photos.clone();
                                    move || format!("img/{}", photos[active_photo.get() as usize])
                                }
                                alt=""
                                class="w-full h-full object-contain transition-all duration-300"
                            />
                        </div>

                        <div class="relative z-10 mt-auto">
                            <Navigator index=active_photo length=photos.len() />
                        </div>
                    </div>

                </div>
            </div>
        </Page>
    }
}
