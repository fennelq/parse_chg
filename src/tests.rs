#[cfg(test)]
mod tests {

    use std::path::Path;
    use std::io::prelude::*;
    use std::fs::File;
    use std::error::Error;
    use sig::*;
    use read_write::*;

    fn write_test<T: HasWrite> (sig: &T) -> Vec<u8> {
        sig.write()
    }
    #[test]
    fn complex_read_write() {
        let test_path = Path::new("test_cases");
        for entry in test_path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                let input = entry.path();
                let display = input.display();
                println!("{:?}", input.file_name().expect("no file"));
                let mut file = match File::open(&input) {
                    Err(why) => panic!("couldn't open {}: {}", display,
                                       why.description()),
                    Ok(file) => file,
                };
                let mut original_in: Vec<u8> = vec![];
                match file.read_to_end(&mut original_in) {
                    Err(why) => panic!("couldn't read {}: {}", display,
                                       why.description()),
                    Ok(_) => (),
                };
                assert_eq!(original_in, write_test(&read_file_raw(&input)));
            }
        }
    }
}