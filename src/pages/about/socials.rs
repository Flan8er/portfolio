use leptos::prelude::{set_timeout, *};
use leptos_icons::Icon;
use web_sys::window;

const EMAIL: &str = "casey.vaughn9@aol.com";

#[component]
pub fn SocialIcons() -> impl IntoView {
    let copy_success = RwSignal::new(false);

    let copy_email = move |_| {
        let _ = window()
            .unwrap()
            .navigator()
            .clipboard()
            // .unwrap()
            .write_text(EMAIL);
        copy_success.set(true);

        // Optional: reset success message after a delay
        set_timeout(
            move || copy_success.set(false),
            std::time::Duration::from_secs(2),
        );
    };

    view! {
        <span class="socials flex gap-6 mt-4">
            <a
                href="https://github.com/Flan8er"
                target="_blank"
                rel="noopener noreferrer"
                class="social-button p-2 hover:scale-110 transition origin-center"
            >
                <Icon icon=icondata::ChGithub width="24px" height="24px" style="transition: color 0.2s ease;" />
            </a>

            <a
                href="https://linkedin.com/in/casey-vaughn-1a8ba72b2"
                target="_blank"
                rel="noopener noreferrer"
                class="social-button p-2 hover:scale-110 transition origin-center"
            >
                <Icon icon=icondata::FiLinkedin width="24px" height="24px" style="transition: color 0.2s ease;" />
            </a>

            <button
                class="social-button p-2 hover:scale-110 transition relative origin-center"
                on:click=copy_email
            >
                <Icon icon=icondata::FiMail width="24px" height="24px" style="transition: color 0.2s ease;" />

                {move || if copy_success.get() {
                    view! { <span class="text-sm text-green-400 absolute right-1/2 translate-x-1/2">"Copied!"</span> }.into_any()
                } else {
                    view!{<></>}.into_any()
                }}
            </button>
        </span>
    }
}
