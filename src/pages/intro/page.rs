use leptos::prelude::*;
use leptos_use::use_interval_fn;

use crate::{
    components::page::Page,
    pages::intro::{arrow::AnimatedChevron, socials::SocialIcons},
};

#[component]
pub fn Intro() -> impl IntoView {
    // Background animation
    let bg_angle = RwSignal::new(45.0); // degrees
    use_interval_fn(
        move || {
            bg_angle.update(|angle| {
                *angle += 0.5;
                if *angle >= 360.0 {
                    *angle -= 360.0;
                }
            });
        },
        40,
    );

    view! {
        <Page>
            <div
                class="w-full h-full flex flex-col items-center justify-center relative"
                style=move || {
                    format!(
                        "background: linear-gradient({}deg, rgba(147, 51, 234, 0.25), rgba(59, 130, 246, 0.25));",
                        bg_angle.get()
                    )
                }
            >
                <h1 class="text-6xl md:text-8xl font-bold">"Casey Vaughn"</h1>

                <p class="text-xl md:text-2xl text-gray-300 mt-8">"Full Stack Developer & UI/UX Designer"</p>

                <SocialIcons/>

                <AnimatedChevron/>
            </div>
        </Page>
    }
}
