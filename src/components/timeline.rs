use leptos::prelude::*;
use strum_macros::EnumIter;

use crate::{
    components::page::Page,
    pages::{
        about::page::AboutMe,
        intro::page::Intro,
        skills::page::Skills,
        timeline::{
            twenty_four::TwentyTwentyFour, twenty_three::TwentyTwentyThree,
            twenty_two::TwentyTwentyTwo,
        },
    },
};

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Default)]
pub enum Timeline {
    #[default]
    Intro,
    About,
    Skills,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
    Appendix,
}

impl Timeline {
    pub fn into_string(&self) -> String {
        match self {
            Self::Intro => String::from("Introduction"),
            Self::About => String::from("About Me"),
            Self::Skills => String::from("Skills"),
            Self::TwentyTwo => String::from("2022"),
            Self::TwentyThree => String::from("2023"),
            Self::TwentyFour => String::from("2024"),
            Self::TwentyFive => String::from("2025"),
            Self::Appendix => String::from("Appendix"),
        }
    }

    pub fn render(&self) -> impl IntoView {
        match self {
            Self::Intro => view! {<Intro/>}.into_any(),
            Self::About => view! {<AboutMe/>}.into_any(),
            Self::Skills => view! {<Skills/>}.into_any(),
            Self::TwentyTwo => view! {<TwentyTwentyTwo/>}.into_any(),
            Self::TwentyThree => view! {<TwentyTwentyThree/>}.into_any(),
            Self::TwentyFour => view! {<TwentyTwentyFour/>}.into_any(),
            Self::TwentyFive => view! {
                <Page>
                    <div class="w-full h-full bg-zinc-600"></div>
                </Page>
            }
            .into_any(),
            Self::Appendix => view! {
                <Page>
                    <div>
                        "This entire app was built using the leptos frame work"
                        "Third wall image"
                        "Want to check out the source code? I'll leave a link to it here."
                    </div>
                </Page>
            }
            .into_any(),
        }
    }
}
