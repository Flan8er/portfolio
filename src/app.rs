use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::{
    components::page::Page,
    pages::{about::page::AboutMe, timeline::page::Timeline},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| "Route not found...">
                <Route path=path!("") view=Home/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <main class="text-center overflow-y-auto">
            <Timeline/>
            <AboutMe/>

            <Page>
                <div class="w-full h-full bg-blue-300"></div>
            </Page>

            <Page>
                <div class="w-full h-full bg-orange-300"></div>
            </Page>

            <Page>
                <div class="w-full h-full bg-green-300"></div>
            </Page>
            <h1>"Hello, World!"</h1>
            <ThirdWall/>
        </main>
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
