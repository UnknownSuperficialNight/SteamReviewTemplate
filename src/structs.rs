use crate::egui::FontData;
use crate::egui::FontFamily;

// Struct to hold temporary data for the application
pub struct TempData {
    // String to store the final output
    pub final_output: String,
    // Vector of tuples containing menu items and their options
    pub menu_array: Vec<(&'static str, Vec<&'static str>)>, // Array of menu items and their options
    // Vector of strings to store parent menu item names
    pub menu_array_parent: Vec<String>, // Array of parent menu item names
    // Current position in the menu array
    pub menu_array_pos: u8, // Current position in the menu array
}

// Implementation of Default trait for TempData
impl Default for TempData {
    // Default function to create a new TempData instance
    fn default() -> Self {
        // Define the menu array with categories and their options
        let menu_array: Vec<(&'static str, Vec<&'static str>)> = vec![
            ("Graphics", vec!["You forget what reality is", "Beautiful", "Good", "Decent", "Bad", r"Don't look too long at it", "MS-DOS "]),
            // Gameplay category and its options
            ("Gameplay", vec!["Very good", "Good", "It's just gameplay", "Mehh", "Watch paint dry instead", "Just don't"]),
            // Audio category and its options
            ("Audio", vec!["Eargasm", "Very good", "Good", "Not too bad", "Bad", "I'm now deaf"]),
            // Audience category and its options
            ("Audience", vec!["Kids", "Teens", "Adults", "Grandma"]),
            // PC Requirements category and its options
            ("PC Requirements", vec!["Check if you can run paint", "Potato", "Decent", "Fast", "Rich boi", "Ask NASA if they have a spare computer"]),
            // Game Size category and its options
            (
                "Game Size",
                vec![
                    "Floppy Disk",
                    "Old Fashioned",
                    "Workable",
                    "Big",
                    "Will eat 10% of your 1TB hard drive",
                    "You will want an entire hard drive to hold it",
                    "You will need to invest in a black hole to hold all the data",
                ],
            ),
            // Difficulty category and its options
            ("Difficulty", vec!["Just press 'W'", "Easy", "Easy to learn / Hard to master", "Significant brain usage", "Difficult", "Dark Souls"]),
            // Grind category and its options
            (
                "Grind",
                vec!["Nothing to grind", "Only if u care about leaderboards/ranks", "Isn't necessary to progress", "Average grind level", "Too much grind", "You'll need a second life for grinding"],
            ),
            // Story category and its options
            ("Story", vec!["No Story", "Some lore", "Average", "Good", "Lovely", "It'll replace your life"]),
            // Game Time category and its options
            ("Game Time", vec!["Long enough for a cup of coffee", "Short", "Average", "Long", "To infinity and beyond"]),
            // Price category and its options
            ("Price", vec!["It's free!", "Worth the price", "If it's on sale", "If u have some spare money left", "Not recommended", "You could also just burn your money"]),
            // Bugs category and its options
            ("Bugs", vec!["Never heard of", "Minor bugs", "Can get annoying", "ARK: Survival Evolved", "The game itself is a big terrarium for bugs"]),
            // Rating category and its options
            ("? / 10", vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]),
        ];
        // Create a vector of parent menu item names
        let mut menu_array_parent: Vec<String> = vec![];
        // Iterate through the menu array and extract parent names
        for (inner_vec_name, _) in &menu_array {
            // Add each parent name to the menu_array_parent vector
            menu_array_parent.push(inner_vec_name.to_string());
        }
        // Return the TempData struct with initialized values
        TempData {
            // Initialize final_output as an empty string
            final_output: String::new(),
            // Set the menu_array field with the defined menu options
            menu_array,
            // Set the menu_array_parent field with the extracted parent names
            menu_array_parent,
            // Initialize menu_array_pos to 0
            menu_array_pos: 0,
        }
    }
}

// Struct to hold a collection of strings
#[derive(Default)]
pub struct StringContainer {
    // Vector to store strings
    pub strings: Vec<String>,
}

// Implementation of methods for StringContainer
impl StringContainer {
    // Create a new StringContainer with 13 empty strings
    pub fn new() -> Self {
        // Create a new vector with a capacity of 13
        let mut strings = Vec::with_capacity(13);
        // Add 13 empty strings to the vector
        for _ in 0..13 {
            strings.push(String::new());
        }
        // Return a new StringContainer with the initialized vector
        StringContainer { strings }
    }

    // Set the string at a specific index
    pub fn set_string(&mut self, index: usize, value: String) {
        // Check if the index is within bounds
        if index < self.strings.len() {
            // Set the string at the specified index
            self.strings[index] = value;
        } else {
            // Panic if the index is out of bounds
            panic!("Index out of bounds");
        }
    }

    // Get a reference to the string at a specific index
    pub fn get_string(&self, index: usize) -> Option<&String> {
        // Return a reference to the string at the specified index, or None if out of bounds
        self.strings.get(index)
    }
}

#[derive(Default)]
pub struct MyEguiApp {
    pub selected_data_vec: StringContainer, // Stores the selected data for each category in a custom container
    pub temp_data: TempData,                // Holds temporary data for the application, including menu items and current position
    pub update_toggle_bool: bool,           // Flag to control UI updates, specifically for toggling menu items
    pub refresh_output_bool: bool,          // Flag to control UI updates, specifically for refreshing the output display
}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Create a new FontDefinitions object with default settings
        let mut fonts = eframe::egui::FontDefinitions::default();

        // Install custom font from a file, potentially supporting non-latin characters
        fonts.font_data.insert("my_font".to_owned(), FontData::from_static(include_bytes!("fonts/font.ttf")));

        // Set the custom font as the highest priority for proportional text
        if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
            family.insert(0, "my_font".to_owned());
        }

        // Add the custom font as the last fallback option for monospace text
        if let Some(family) = fonts.families.get_mut(&FontFamily::Monospace) {
            family.push("my_font".to_owned());
        }

        // Apply the custom font settings to the application context
        cc.egui_ctx.set_fonts(fonts);

        // Return a new instance of MyEguiApp with initialized values
        Self { selected_data_vec: StringContainer::new(), temp_data: TempData::default(), update_toggle_bool: true, refresh_output_bool: true }
    }
}
