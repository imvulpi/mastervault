#[test]
fn test_correct_algorithm(){
    use crate::commons::fields::{structs::algorithm::AlgorithmData, traits::validatable::Validatable};

    let test_cases = [
        "argon2id", "argon2", "argon2i",
    ];

    for algorithm in test_cases{
        let algorithm_data = AlgorithmData{algorithm: algorithm.to_string()};
        assert_eq!(algorithm_data.algorithm, algorithm);
        let algorithm_validation_result = algorithm_data.validate();
        assert_eq!(algorithm_validation_result.0,true);
    }
}

#[test]
fn test_incorrect_algorithm(){
    use crate::commons::fields::{structs::algorithm::AlgorithmData, traits::validatable::Validatable};

    let test_cases = [
        "1", "2", "3", "algon", "algorithm1", "algorithm2", "algorithm3", "aragon1", "1argon", "2argon",
        "idargon", "iargon", "1@", "!@#$", "!@#rsa", "", " ", " argon2", "a", "b"
    ];

    for algorithm in test_cases {
        let algorithm_data = AlgorithmData{algorithm: algorithm.to_string()};
        assert_eq!(algorithm_data.algorithm, algorithm);
        let algorithm_validation_result = algorithm_data.validate();
        assert_eq!(algorithm_validation_result.0,false);
    }
}