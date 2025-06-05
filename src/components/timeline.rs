use strum_macros::EnumIter;

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
        }
    }
}
