use strum::{AsRefStr, EnumString};

macro_rules! impl_menu_option {
    ($enum_name:ident, $($variant:ident),*) => {
        #[derive(EnumString, AsRefStr, Debug)]
        pub enum $enum_name {
            $($variant),*
        }

        impl MenuOption for $enum_name {
            fn all_options() -> Vec<&'static str> {
                vec![$(Self::$variant.as_ref()),*]
            }

            fn from_str(s: &str) -> Option<$enum_name> {
                s.parse().ok()
            }
        }
    };
}

// Trait that captures the common behavior of the enums
pub trait MenuOption: Sized {
    fn all_options() -> Vec<&'static str>;
    fn from_str(s: &str) -> Option<Self>;
}

impl_menu_option!(MainOption, Search, Settings, Quit);
impl_menu_option!(
    PlayerOption,
    Play,
    Next,
    Previous,
    Download,
    Select,
    Menu,
    Quit
);
impl_menu_option!(SettingOption, Audio, Player, Quality);
impl_menu_option!(ErrorOption, Menu, Quit);
