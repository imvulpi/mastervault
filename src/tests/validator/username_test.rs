#[test]
fn test_incorrect_username(){
    use crate::commons::fields::{structs::username::UsernameData, traits::validatable::Validatable};

    let test_cases = [
        "Q", "42", "x", "Jo", "AB",
        "7z", "Y", "Z1", "V", "K",

        "P", "9a", "L", "6b", "X3",
        "U", "2h", "J", "E", "g8",

        "i", "w", "c4", "f", "1x",
        "o", "m2", "r", "d5", "s",

        "3v", "a", "8y", "b", "n6",
        "q", "5z", "h", "t7", "0i",

        "u", "4k", "e", "y9", "3c",
        "1", "z", "2", "6", "7",
    ];

    for &username in &test_cases{
        let username_data = UsernameData { username: username.to_string() };
        assert_eq!(username_data.username, username);
        let username_validation_result = username_data.validate();
        assert_eq!(username_validation_result.0, false);
    }   
}

#[test]
fn test_correct_username(){
    use crate::commons::fields::{structs::username::UsernameData, traits::validatable::Validatable};

    let test_cases = [
        "Sunflower123", "BlueSky_87", "GamerGirl", "TechWizard42", "AdventureTime",
        "StarlightDreamer", "CoffeeLover", "MusicMaster", "PizzaNinja", "Bookworm21",

        "MysticDragon", "RainbowSparkle", "Traveler_Explorer", "PixelPirate", "NaturePhotographer",
        "SerenitySeeker", "EpicGamerX", "ChocoChipCookie", "GalacticSorcerer", "FitnessFanatic",

        "ArtisticSoul", "ScienceGeek", "MoonlightShadow", "SunnyDayz", "AquaMarine", 
        "Wanderlust_23", "MemeLord", "NightOwl_99", "CaffeineQueen", "SoccerStar_10",

        "TechieGuy", "JazzVibes", "MountainHiker", "CosmicAdventurer", "PixelPainter",
        "Dreamer_365", "GreenTeaZen", "RockClimber_7", "PianoMaestro", "EternalOptimist",

        "StarryNightSky", "GourmetExplorer", "FloralDream", "ChessMaster_64", "RainyDayReads",
        "StarGazer_88", "CyclingNomad", "CodeNinja", "GuitarHero_21", "WildlifeWatcher",
    ];

    for &username in &test_cases{
        let username_data = UsernameData { username: username.to_string() };
        assert_eq!(username_data.username, username);
        let username_validation_result = username_data.validate();
        assert_eq!(username_validation_result.0, true);
    } 
}