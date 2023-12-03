#[test]
fn test_correct_parallelism(){
    use crate::commons::fields::{structs::parallelism::ParallelismData, traits::validatable::Validatable};

    let test_cases = [
        "1","99999999","34567","89012","52","90123","56789","5125","512","32",
        "89012","45678","123","67890","34567","89012","34","90123","123","12345"
    ];

    for number in test_cases{
        let parallelism_data = ParallelismData { parallelism: number.to_string() };
        assert_eq!(parallelism_data.parallelism, number);
        let parallelism_validation_result = parallelism_data.validate();
        assert_eq!(parallelism_validation_result.0, true);
        let paralelism_number = parallelism_data.get_raw_value().parse::<u32>();
        assert_eq!(paralelism_number.unwrap(), number.parse::<u32>().unwrap()); 
    }
}

#[test]
fn test_wrong_parallelism(){
    use crate::commons::fields::{structs::parallelism::ParallelismData, traits::validatable::Validatable};

    let test_cases = [
        "100000000", "a123", "123b", "1a2b3c", "-1",
        "0","-123", "-1a", "-1a2c", " 32"
    ];

    for number in test_cases{
        let parallelism_data = ParallelismData { parallelism: number.to_string() };
        assert_eq!(parallelism_data.parallelism, number);
        let parallelism_validation_result = parallelism_data.validate();
        assert_eq!(parallelism_validation_result.0, false);
    }
}