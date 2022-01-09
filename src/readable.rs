pub mod readable{
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub trait Readable<I,O> {
        fn read(&self, input: I) ->  O;
    }

    pub struct FileReader {}

    impl Readable<String, Vec<String>> for FileReader {
        fn read(&self, input: String) -> Vec<String> {
            let result = File::open(input);
            let file =  match result {
                Ok(file) => file,
                Err(err) => panic!("Problem opening the file: {:?}", err)
            };

            let reader = BufReader::new(file);
            let mut data_list = Vec::new();

            for (_index, line) in reader.lines().enumerate() {
                let line = line.unwrap();
                data_list.push(line);
            }

            return data_list;
        }
    }
}

