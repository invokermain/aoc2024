use std::{fs::File, io::Read, path, path::Path};

pub fn load_file_contents(day: usize) -> String {
    let file_path = format!("src/day{day}/input.txt");
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
