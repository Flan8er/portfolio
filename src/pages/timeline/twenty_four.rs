use leptos::{html::Div, prelude::*};
use leptos_use::use_interval_fn;

use crate::components::page::Page;

#[component]
pub fn TwentyTwentyFour() -> impl IntoView {
    let photos: [String; 5] = [
        String::from("Labelize1.png"),
        String::from("Labelize2.png"),
        String::from("Labelize3.png"),
        String::from("Limitfab.png"),
        String::from("RRT.png"),
    ];

    let active_photo: RwSignal<u8> = RwSignal::new(0);

    use_interval_fn(
        {
            let photo_count = photos.len() as u8;
            move || {
                active_photo.update(|c| {
                    *c = (*c + 1) % photo_count;
                });
            }
        },
        2000,
    );

    view! {
        <Page>
            <div
                class="w-full h-full flex items-center justify-center relative bg-gradient-to-b from-[rgba(183,46,176,0.15)] to-[rgba(165,49,205,0.175)]"
            >
            // <div
            //     class="w-full h-full flex items-center justify-center relative"
            // >
                <div class="max-w-6xl w-full px-8 grid md:grid-cols-2 gap-12 items-center z-10">
                    <div>
                        <h2 class="text-5xl font-bold mb-8 leading-tight bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
                            "Off to the Races"
                        </h2>

                        <p class="text-lg text-gray-300 leading-relaxed mb-6">
                            "After growing confidence through successfully developing such an intricate app, I found myself having the right opportunity to "
                            "take a true venture into becoming a freelance software developer - this was the start of the goal I worked so hard to achieve. "
                            "During the first stage of building clientele I was still working my full-time mechanical engineering job. Not too long after, though, "
                            "with a few contracts under my belt, I was able to run like never before!"
                        </p>

                        <p class="text-lg text-gray-300 leading-relaxed">
                            "The contracts were relatively tame, but in numbers it was definetely more work than I initially thought. "
                            "A few indi-game proof of concepts, web apps and internal company software. I was finally able to operate wide open, "
                            "explore every avenue and scratch every itch. I even got back to my roots as a Mechanical Engineer on one..."
                        </p>

                        <p class="text-lg text-gray-300 leading-relaxed">
                            "(Code)dependent, LimitFab Then, the big one."
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
                    </div>
                </div>
            </div>
        </Page>
    }
}
