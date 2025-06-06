use leptos::prelude::*;

#[component]
pub fn Navigator(index: RwSignal<u8>, length: usize) -> impl IntoView {
    view! {
        <div class="flex gap-4 px-6 py-4 rounded-full backdrop-blur-md bg-white/5 border border-white/10 shadow-md">
            <For
                each=move || 0..length
                key=|&i| i
                children=move |i| {
                    let is_active = move || index.get() == i as u8;

                    view! {
                        <button
                            class="h-3 rounded-full bg-white/70 transition-bounce"
                            style=move || format!(
                                "width: {}px;",
                                if is_active() { 40 } else { 12 }
                            )
                            on:click=move |_| index.set(i as u8)
                        />
                    }
                }
            />
        </div>
    }
}
