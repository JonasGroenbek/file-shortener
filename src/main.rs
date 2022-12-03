use std::env;
use std::fs::OpenOptions;
use std::io::Seek;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if (args.len() as i32) < 3 {
        panic!("Require two arguments, filepath and desired amount of lines ouput. e.g \n./file_shortener.sh -- /tmp/my_logs 10000");
    }

    let length = &args[2].parse::<i32>().unwrap();

    let path = Path::new(&args[1]);

    let mut file = match OpenOptions::new().read(true).open(path) {
        Ok(file) => file,
        Err(error) => {
            panic!("error: {}", error.to_string())
        }
    };

    let mut filtered_data: Vec<Vec<u8>> = Vec::new();

    let lines_amount = BufReader::new(&file).lines().count();

    if lines_amount == 0 {
        panic!("The file has no lines");
    }

    // reset files position to start
    match file.seek(std::io::SeekFrom::Start(0)) {
        Ok(res) => res,
        Err(error) => {
            panic!("error: {}", error.to_string());
        }
    };

    for (i, line_result) in BufReader::new(&file).lines().enumerate() {
        let line = match line_result {
            Ok(line) => format!("\n{}", line),
            Err(error) => {
                panic!(
                    "Something went wrong retrieving the line of the file, {}",
                    error.to_string()
                )
            }
        };

        if (i as i32) > *length - 1 {
            filtered_data.remove(0);
        }
        filtered_data.push(line.as_bytes().to_vec());
    }

    if filtered_data.len() != 0 {
        //removes the first byte of the first line, which is the new line
        filtered_data[0].remove(0);
    }

    let mut file = match OpenOptions::new().truncate(true).write(true).open(path) {
        Ok(file) => file,
        Err(error) => {
            panic!("error: {}", error.to_string())
        }
    };

    let mut data: Vec<u8> = Vec::new();

    for v in filtered_data {
        for b in v {
            data.push(b);
        }
    }

    let slice = data.as_slice();

    file.write_all(slice).unwrap();
}
