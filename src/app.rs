use leptos::{html::Div, prelude::*};
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use leptos_use::{use_scroll, UseScrollReturn};
use strum::IntoEnumIterator;

use crate::{
    components::{page::Page, timeline::Timeline},
    pages::{about::page::AboutMe, intro::page::Intro, timeline::page::Timeline},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| "Route not found...">
                <Route path=path!("") view=Home />
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <main
            class="w-screen h-screen"
        >
            <ScrollProvider>
                <Timeline/>

                <Intro/>

                <AboutMe/>

                <Page>
                    <div class="w-full h-full bg-orange-300"></div>
                </Page>

                <Page>
                    <div class="w-full h-full bg-green-300"></div>
                </Page>

                <Page>
                    <div class="w-full h-full bg-cyan-300"></div>
                </Page>

                <Page>
                    <div class="w-full h-full bg-slate-600"></div>
                </Page>

                <Page>
                    <div class="w-full h-full bg-zinc-600"></div>
                </Page>
            </ScrollProvider>
        </main>
    }
}

#[component]
pub fn ScrollProvider(children: Children) -> impl IntoView {
    let active_timestep = RwSignal::new(Timeline::default());
    provide_context(active_timestep.read_only());

    let el = NodeRef::<Div>::new();

    let timeline_variants = Timeline::iter().collect::<Vec<_>>();
    let num_sections = timeline_variants.len();

    #[allow(unused)]
    let UseScrollReturn {
        x,
        y,
        set_x,
        set_y,
        is_scrolling,
        arrived_state,
        directions,
        ..
    } = use_scroll(el);

    provide_context(y);

    Effect::new(move |_| {
        let scroll_top = y.get();
        let viewport_height = window()
            .inner_height()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0);

        let page_index = (((scroll_top + (viewport_height / 3.0)) / viewport_height) as usize)
            .min(num_sections - 1);
        active_timestep.set(timeline_variants[page_index]);
    });

    view! {
        <div
            node_ref=el
            class="w-full h-full overflow-scroll"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ThirdWall() -> impl IntoView {
    view! {
        <span >
            <p>"This entire app was built in"</p>
            <a>"Leptos"</a>
        </span>
    }
}
