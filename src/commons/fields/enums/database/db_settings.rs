use crate::vault::database::get;

/// The ids of `DatabaseSettings` must be kept the same as the ids in the database.
/// 
/// Example: `Email` will have an id of 1
/// 
/// Unless you know what you are doing:
/// DO NOT change these texts manually! (use text_operations.py in developer tools)
pub enum DatabaseSettings {
    Username,
    Email,
    HashLength,
    Memory,
    Parallelism,
    Algorithm,
    Iterations,
    NotFound,   
    ExampleEnum,
    Example2Enum,
    Exmple2Enum,
}

impl Into<u32> for DatabaseSettings{
    fn into(self) -> u32 {
        self as u32
    }
}

impl DatabaseSettings{
    pub fn text(self) -> String {
        match get(self.into(), "setting", "other_translations"){
            Ok(data) => {
                return data;
            },
            Err(error) => {
                return format!("Database Access Error\nDetailed: {}", error);
            },
        }
    }
}