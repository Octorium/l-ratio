#![doc = include_str!("../README.md")]

macro_rules! traits {
    ($($name:ident)&*) => {
        $(
            pub trait $name {}
        )*
    }
}

macro_rules! define_traits_and_impl_macro {
    ($($name:ident)&*) => {
        traits!($($name)&*);

        /// Implements all of the L&Ratio traits on a struct.
        #[macro_export]
        macro_rules! ratio {
            ($target:ident) => {
                $(
                    impl $name for $target {}
                )*
            }
        }
    }
}

define_traits_and_impl_macro! {
    L & Ratio
    & DontCare & DidntAsk & CryAboutIt & StayMad & GetReal
    & Mald & Seethe & Cope
    & Basic & SkillIssue & YouFellOff
    & Triggered & Redpilled
    & GetALife & OkAnd
    & Cringe & NotBased & TouchGrass
    & Donowalled
}
