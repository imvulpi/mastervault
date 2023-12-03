#[test]
fn test_correct_hash_length(){
    use crate::commons::fields::{structs::hash_length::HashLengthData, traits::validatable::Validatable};

    let test_cases = [
        4, 7, 9, 12, 15,
        23, 26, 31, 48, 50,
        55, 63, 72, 89, 97,
        105, 112, 127, 134, 149,
        156, 162, 178, 183, 195,
        207, 215, 222, 236, 240,
        253, 268, 274, 289, 297,
        305, 312, 327, 333, 345,
        359, 365, 378, 384, 399,
        402, 416, 421, 437, 444,
        5000, 7500, 10000, 15000, 20000,
        30000, 50000, 75000, 100000, 150000
    ];

    for length in test_cases{
        let hash_length = HashLengthData { hash_length: length.to_string()};
        assert_eq!(hash_length.hash_length, length.to_string());
        let hash_length_validation_results = hash_length.validate();
        assert_eq!(hash_length_validation_results.0, true);
        match hash_length.get_raw_value().parse::<i32>(){
            Ok(value) => {
                assert_eq!(value, length);
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}

#[test]
fn test_incorrect_hash_length(){
    use crate::commons::fields::{structs::hash_length::HashLengthData, traits::validatable::Validatable};

    let test_cases = [
        "E12a", "2", "1", "-32", "a",
        "3", "La3", "3p", "2%", "es",
        "E1a", "5-2", "A#-1.8", "-32", "&C1",
        "D2.5", "@-1", "Y-2.7", "Z%0.3", "P3*",
        "F8q", "B9x", "K2.2", "M-0.7", "#6", 
        "X-1.2", "3a", "G4*", "V1.5", "H-3",
        "I2$", "R0.9", "N-2", "J0.5", "L-2.4",
        "U3.8", "W-1", "Q-0.3", "S7*", "T-1.6",
        "Z0.7", "C5!", "P3^", "E2.1", "X-2*",
        "G6%", "A1@", "B-1.5", "D3.2", "Y0.8"
    ];

    for length in test_cases{
        let hash_length = HashLengthData { hash_length: length.to_string()};
        assert_eq!(hash_length.hash_length, length.to_string());
        let hash_length_validation_results = hash_length.validate();
        assert_eq!(hash_length_validation_results.0, false);
    }
}