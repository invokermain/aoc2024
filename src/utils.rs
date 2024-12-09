use std::{fs::File, io::Read, path, path::Path};

pub fn load_input_for_day(day: usize) -> String {
    load_file(day, "input.txt")
}

pub fn load_file(day: usize, name: &str) -> String {
    let file_path = format!("src/day{day}/{name}");
    let file_path = Path::new(&file_path);
    let mut file = File::open(file_path).unwrap_or_else(|_| {
        panic!(
            "Unable to find file at path {:?}",
            path::absolute(file_path).unwrap()
        )
    });

    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents).unwrap();

    file_contents
}
