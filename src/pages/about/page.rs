use leptos::{html::Div, prelude::*};
use web_sys::wasm_bindgen::JsCast;

use crate::components::page::Page;

#[component]
pub fn AboutMe() -> impl IntoView {
    let parent_scroll = expect_context::<Signal<f64>>();
    let el = NodeRef::<Div>::new();
    let left_style = RwSignal::new(String::new());
    let right_style = RwSignal::new(String::new());

    Effect::new(move |_| {
        let Some(el_node) = el.get() else { return };
        let Ok(el) = el_node.dyn_into::<web_sys::Element>() else {
            return;
        };

        let scroll_y = parent_scroll.get();
        let el_offset_top = el.get_bounding_client_rect().top() + scroll_y;
        let el_height = el.scroll_height() as f64;
        let viewport_height = window()
            .inner_height()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0);

        let start_threshold = 0.3;
        let end_threshold = 0.7;

        // How far the top of the element is from the top of the viewport
        let distance_from_top = el_offset_top - scroll_y;

        // How much of the element is scrolled into view (0.0 to 1.0)
        let visible_ratio = ((viewport_height - distance_from_top) / el_height).clamp(0.0, 1.0);

        // Normalize to 0–1 animation progress based on scroll thresholds
        let anim_progress =
            ((visible_ratio - start_threshold) / (end_threshold - start_threshold)).clamp(0.0, 1.0);

        let translate_left = (1.0 - anim_progress) * -100.0;
        let translate_right = (1.0 - anim_progress) * 100.0;
        let opacity = anim_progress;

        left_style.set(format!(
            "transform: translateX({}%) scale(1); opacity: {}; transition: all 0.4s ease-out;",
            translate_left, opacity
        ));
        right_style.set(format!(
            "transform: translateX({}%) scale(1); opacity: {}; transition: all 0.4s ease-out;",
            translate_right, opacity
        ));
    });

    view! {
        <Page>
            <div
                node_ref=el
                class="w-full h-full flex items-center justify-center relative bg-purple-600/10"
            >
                <div class="max-w-6xl w-full px-8 grid md:grid-cols-2 gap-12 items-center z-10">
                    <div style=move || left_style.get()>
                        <h2 class="text-5xl font-bold mb-8 bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
                            "About Me"
                        </h2>

                        <p class="text-lg text-gray-300 leading-relaxed mb-6">
                            "I’m a full-stack developer who cares a lot about making things that are both functional and thoughtfully designed. I’ve been at this for about 5 years now — building clean interfaces, shipping real products, and fine-tuning every pixel and line of logic along the way."
                        </p>

                        <p class="text-lg text-gray-300 leading-relaxed">
                            "I got into this by just being curious. I’ve always wanted to understand how these beautiful apps I was seeing were made, and that’s driven me to push the bounds of what I knew how to make, blending design and engineering to get there. For me, it’s not just about these apps solving problems — it’s about building something that feel good to use."
                        </p>

                    </div>

                    <div class="flex justify-center items-center" style=move || right_style.get()>
                        <div class="relative w-80 h-80 rounded-full p-5 ">
                            <div class="absolute inset-0 rounded-full bg-gradient-to-br from-purple-500 to-pink-500 opacity-20 z-0">

                            </div>

                            <img
                                src="img/CaseyVaughnUT-148.jpg"
                                alt="Casey popping champagne"
                                class="w-full h-full object-cover rounded-full relative z-10"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </Page>
    }
}
