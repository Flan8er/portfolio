use leptos::prelude::*;

#[component]
pub fn Page(children: Children) -> impl IntoView {
    view! {
        <div class="h-screen w-screen min-w-[600px] min-h-[600px]">
            {children()}
        </div>
    }
}
