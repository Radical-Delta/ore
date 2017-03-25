use std::fmt::{Debug, Display, Formatter, Result};

/// Specifies the category of a project.
///
/// Every project can only have one `Category`
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
    Undefined,
}

/// Specifies the sort query type.
#[derive(Clone, Copy)]
pub enum SortType {
    MostStars,
    MostDownloads,
    MostViews,
    Newest,
    RecentlyUpdated,
}

impl Category {
    /// Gets a `Category` from a string.
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
            _ => Category::Undefined,
        }
    }

    /// Gets a `Category` from an id.
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
            _ => Category::Undefined,
        }
    }

    /// Converts a `Category` to its corresponding id.
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
            _ => 10u8,
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
                _ => "Undefined",
            }
            .to_string()
    }
}

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,
               "Category {{ id: {}, name: {} }}",
               self.to_id(),
               self.as_string())
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.as_string())
    }
}

impl SortType {
    /// Gets a `SortType` from a string.
    pub fn from_string(name: &str) -> Self {
        match name {
            "MostDownloads" => SortType::MostDownloads,
            "MostViews" => SortType::MostViews,
            "Newest" => SortType::Newest,
            "RecentlyUpdated" => SortType::RecentlyUpdated,
            _ => SortType::MostStars,
        }
    }

    /// Gets a `SortType` from an id.
    pub fn from_id(id: &u8) -> Self {
        match id {
            &1u8 => SortType::MostDownloads,
            &2u8 => SortType::MostViews,
            &3u8 => SortType::Newest,
            &4u8 => SortType::RecentlyUpdated,
            _ => SortType::MostStars,
        }
    }

    /// Converts a `SortType` to its corresponding id.
    pub fn to_id(&self) -> u8 {
        match *self {
            SortType::MostStars => 0u8,
            SortType::MostDownloads => 1u8,
            SortType::MostViews => 2u8,
            SortType::Newest => 3u8,
            SortType::RecentlyUpdated => 4u8,
        }
    }

    fn as_string(&self) -> String {
        match *self {
                SortType::MostStars => "MostStars",
                SortType::MostDownloads => "MostDownloads",
                SortType::MostViews => "MostViews",
                SortType::Newest => "Newest",
                SortType::RecentlyUpdated => "RecentlyUpdated",
            }
            .to_string()
    }
}

impl Debug for SortType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,
               "SortType {{ id: {}, name: {} }}",
               self.to_id(),
               self.as_string())
    }
}

impl Display for SortType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.as_string())
    }
}
