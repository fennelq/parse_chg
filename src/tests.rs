//! Тесты
//!
//! Реализован комплексный тест на расборку/сборку всех фалов из директории test_cases
#[cfg(test)]
mod complex_tests {

    use crate::read_write::*;
    use crate::sig::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    fn write_test<T: HasWrite>(sig: &T) -> Vec<u8> {
        sig.write()
    }
    #[test]
    fn complex_read_write_raw() {
        let test_path = Path::new("test_cases");
        for entry in test_path
            .read_dir()
            .expect("read_dir call failed")
            .flatten()
        {
            let input = entry.path();
            let display = input.display();
            let mut file = match File::open(&input) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(file) => file,
            };
            let mut original_in: Vec<u8> = vec![];
            if let Err(why) = file.read_to_end(&mut original_in) {
                panic!("couldn't read {}: {}", display, why)
            };
            assert_eq!(original_in, write_test(&read_file_raw(&input)));
        }
    }

    #[test]
    fn complex_read_write() {
        let test_path = Path::new("test_cases");
        for entry in test_path
            .read_dir()
            .expect("read_dir call failed")
            .flatten()
        {
            let input = entry.path();
            let display = input.display();
            let mut file = match File::open(&input) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(file) => file,
            };
            let mut original_in: Vec<u8> = vec![];
            if let Err(why) = file.read_to_end(&mut original_in) {
                panic!("couldn't read {}: {}", display, why)
            };
            assert_eq!(original_in, write_test(&read_file_raw(&input)));
        }
    }
}

#[cfg(test)]
pub mod rab_e_sig_test {
    pub fn read_test_sig(path_str: &str) -> Vec<u8> {
        use std::io::Read;
        let path = std::path::Path::new(path_str);
        let display = path.display();
        let mut file = match std::fs::File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        let mut original_in: Vec<u8> = vec![];
        if let Err(why) = file.read_to_end(&mut original_in) {
            panic!("couldn't read {}: {}", display, why)
        };
        original_in
    }
}
