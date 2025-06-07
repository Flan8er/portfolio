use leptos::prelude::*;
use leptos_use::use_interval_fn;

use crate::components::page::Page;

// from-[rgba(165,49,205,0.175)] to-[rgba(147,51,234,0.2)]
#[component]
pub fn TwentyTwentyFive() -> impl IntoView {
    // let photos: [String; 7] = [
    //     String::from("Meteorite1.png"),
    //     String::from("Meteorite2.png"),
    //     String::from("Meteorite3.png"),
    //     String::from("Meteorite4.png"),
    //     String::from("Meteorite5.png"),
    //     String::from("Meteorite6.png"),
    //     String::from("Meteorite7.png"),
    // ];
    let photos = [
        "Meteorite1.png",
        "Meteorite2.png",
        "Meteorite3.png",
        "Meteorite4.png",
        "Meteorite5.png",
        "Meteorite6.png",
        "Meteorite7.png",
    ]
    .map(String::from);

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
        12000,
    );

    view! {
        <Page>
            <div
                class="w-full h-full flex items-center justify-center relative"
                // style=move || {
                //     let index = active_photo.get();
                //     let photo = &photos[index as usize];
                //     format!("background-image: url(\"/img/{}\"); background-size: cover; background-position: center; background-repeat: no-repeat;", photo)
                // }
            >
                <div
                    class="absolute inset-0 bg-center bg-no-repeat bg-cover opacity-[15%]"
                    style=move || {
                        let index = active_photo.get() as usize;
                        format!("background-image: url(\"img/{}\");", photos[index])
                    }
                />
                <div class="max-w-6xl w-full h-full px-8 flex items-center justify-center z-10">
                    <div>
                        <h2 class="text-5xl font-bold mb-8 leading-tight bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
                            "Rust all the way down"
                        </h2>

                        <p class="text-lg text-gray-300 leading-relaxed mb-6">
                            "The next contract I receieved was from a space startup whos goal was to 3D print solid rocket motors and and reentry capsules. "
                            "Up until this point I haven't touched much on my previous fulltime job at the KCNSC (Kansas City National Security Campus) "
                            "- I worked in a group developing high precision 5-Axis "
                            "DIW 3D printers and the motion control/slicer software for them. We developed a program to perform coordinated motion on complex contoured parts. "
                            "This new contract was to build a similar software, but with the experience of having done it prior: build it better."
                        </p>

                        <p class="text-lg text-gray-300 leading-relaxed">
                            "Acting as a SME, I lead their internal team to develop a real-time, monitoring and control software for a 6-Axis robot arm MAD 3D printer. "
                            "This software was the culmination of so many previous experiences: Reveal being built in Tauri was a desperation attempt to get away from Electron - this taught me Rust, "
                            "indi game development through my knowledge in Rust introduced me to Bevy - and ECS driven game engine built in Rust, and my work at the KCNSC for building motion control software. "
                            "This new software used all of them. Bevy was used as the backend since entity driven logic translates cleanly to real-world interaction. "
                            "For the frontend, yet another Rust library was used - Leptos."
                        </p>
                    </div>

                    // <div class="w-[600px] h-[500px] relative flex flex-col items-center justify-between rounded-md overflow-hidden">
                    // </div>
                </div>
            </div>
        </Page>
    }
}
