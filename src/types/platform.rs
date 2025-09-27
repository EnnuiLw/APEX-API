#[derive(Debug, Default)]
#[non_exhaustive]
pub enum Platform {
    #[default]
    PC,
    Xbox,
    PS,
    Switch,
    // Any,
}

impl ToString for Platform {
    fn to_string(&self) -> String {
        match self {
            Platform::PC => "PC",
            Platform::Xbox => "Xbox",
            Platform::PS => "PlayStation",
            Platform::Switch => "Switch",
            // _ => "Unknown"
        }
        .to_string()
    }
}
