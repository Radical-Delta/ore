use std::fmt::{ Debug, Display, Formatter, Result };

#[derive(Clone, Copy)]
pub enum Category {
    AdminTools,
    Chat,
    DeveloperTools,
    Economy,
    Gameplay,
    Games,
    Protection,
    RolePlaying,
    WorldManagement,
    Miscellaneous,
    Undefined
}

impl Category {
    pub fn from_string(name: &str) -> Self {
        match name {
            "AdminTools" => Category::AdminTools,
            "Chat" => Category::Chat,
            "DeveloperTools" => Category::DeveloperTools,
            "Economy" => Category::Economy,
            "Gameplay" => Category::Gameplay,
            "Games" => Category::Games,
            "Protection" => Category::Protection,
            "RolePlaying" => Category::RolePlaying,
            "WorldManagement" => Category::WorldManagement,
            "Miscellaneous" => Category::Miscellaneous,
            _ => Category::Undefined
        }
    }

    pub fn from_id(id: &u8) -> Self {
        match id {
            &0u8 => Category::AdminTools,
            &1u8 => Category::Chat,
            &2u8 => Category::DeveloperTools,
            &3u8 => Category::Economy,
            &4u8 => Category::Gameplay,
            &5u8 => Category::Games,
            &6u8 => Category::Protection,
            &7u8 => Category::RolePlaying,
            &8u8 => Category::WorldManagement,
            &9u8 => Category::Miscellaneous,
            _ => Category::Undefined
        }
    }

    pub fn to_id(&self) -> u8 {
        match *self {
            Category::AdminTools => 0u8,
            Category::Chat => 1u8,
            Category::DeveloperTools => 2u8,
            Category::Economy => 3u8,
            Category::Gameplay => 4u8,
            Category::Games => 5u8,
            Category::Protection => 6u8,
            Category::RolePlaying => 7u8,
            Category::WorldManagement => 8u8,
            Category::Miscellaneous => 9u8,
            _ => 10u8
        }
    }

    fn as_string(&self) -> String {
        match *self {
            Category::AdminTools => "AdminTools",
            Category::Chat => "Chat",
            Category::DeveloperTools => "DeveloperTools",
            Category::Economy => "Economy",
            Category::Gameplay => "Gameplay",
            Category::Games => "Games",
            Category::Protection => "Protection",
            Category::RolePlaying => "RolePlaying",
            Category::WorldManagement => "WorldManagement",
            Category::Miscellaneous => "Miscellaneous",
            _ => "Undefined"
        }.to_string()
    }
}

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Category {{ id: {}, name: {} }}", self.to_id(), self.as_string())
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.as_string())
    }
}
