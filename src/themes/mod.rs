pub struct Theme {
    primary: String,
    primary_dark: String,
    primary_light: String,

    background: String,
    background_dark: String,
    background_light: String,

    text: String,
    placeholder: String,
    borders: String,
    highlight: String,
}

// When dealing with colors and backgrounds we should only use values provided within the Themes
impl Default for Theme {
    fn default() -> Self {
        Self { 
            primary: String::from("#e693d6"), 
            primary_dark: String::from("#da89ca"), 
            primary_light: String::from("#f29ae1"), 
            background: String::from("#0e0d17"), 
            background_dark: String::from("#0e0d17"), 
            background_light: String::from("#191729"), 
            text: String::from("#c0c0c2"),
            placeholder: String::from("#5f4c8b"),
            borders: String::from("#3c334b"), 
            highlight: String::from("#2b2843")
        }
    }
}

impl Theme {
    fn load_or_default() -> Self {
        // TODO: Support loading themes in memory
        Self::default()
    }
}