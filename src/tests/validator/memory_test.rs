#[test]
fn test_correct_memory(){
    use crate::commons::fields::{structs::memory::MemoryData, traits::validatable::Validatable};

    let test_cases = [
        "16384", "17105", "17826", "18547", "19268",
        "20000", "20721", "21442", "22163", "22884",
        "23616", "24337", "25058", "25779", "26500",
        "27232", "27953", "28674", "29395", "30116",
        "30848", "31569", "32290", "33011", "33732",
        "34464", "35185", "35906", "36627", "37348",
        "50000", "60000", "70000", "80000", "90000",
        "100000", "110000", "120000", "130000", "140000",
        "150000", "160000", "170000", "180000", "190000",
        "200000", "210000", "220000", "230000", "240000"
    ];

    for number in test_cases{
        let memory_data = MemoryData {memory_in_kib: number.to_string()};
        assert_eq!(memory_data.memory_in_kib, number);
        let memory_validation_results = memory_data.validate();
        assert_eq!(memory_validation_results.0, true);
        match memory_data.get_raw_value().parse::<u32>(){
            Ok(num) => {
                assert_eq!(num, number.parse::<u32>().unwrap());
            }
            Err(e) =>{
                panic!("{}", e);
            }
        }
    }
}

#[test]
fn test_incorrect_memory(){
    use crate::commons::fields::{structs::memory::MemoryData, traits::validatable::Validatable};

    let test_cases = [
        "16000", "15700", "15400", "15100", "14800",
        "14500", "14200", "13900", "13600", "13300",
        "13000", "12700", "12400", "12100", "11800",
        "11500", "11200", "10900", "10600", "10300",
        "10000", "9700", "9400", "9100", "8800",
        "8500", "8200", "7900", "7600", "7300",
        "7000", "6700", "6400", "6100", "5800",
        "5500", "5200", "4900", "4600", "4300",
        "4000", "3700", "3400", "3100", "2800",
        "2500", "2200", "1900", "1600", "1300",
        "-1600", "-1900", "-2200", "-2500", "-2800",
        "-3100", "-3400", "-3700", "-4000", "-4300"
    ];
    
    for number in test_cases{
        let memory_data = MemoryData{memory_in_kib: number.to_string()};
        assert_eq!(memory_data.memory_in_kib, number);
        let memory_validation_results = memory_data.validate();
        assert_eq!(memory_validation_results.0, false);
    }
}