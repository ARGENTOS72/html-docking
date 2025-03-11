use std::{
    fs::{create_dir, read_dir, read_to_string, File},
    io::{stdin, stdout, Write},
    path::PathBuf,
};

fn read_html_files(dir_path: PathBuf, count: &mut u32, file_dir: PathBuf) {
    let dir = read_dir(dir_path.clone()).unwrap();
    _ = create_dir(file_dir.clone());

    dir.into_iter().flatten().for_each(|file| {
        let path = file.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if path.is_dir() {
            println!("DIR: {}", file_name);

            read_html_files(dir_path.join(file_name), count, file_dir.join(file_name));
        }

        if file_name.ends_with(".html") {
            use scraper::{Html, Selector};

            let content = read_to_string(dir_path.join(file_name)).unwrap();
            let document = Html::parse_document(&content);
            let selector = Selector::parse("main").unwrap();

            let file_path = file_dir.join(file_name);

            println!("{:?}", file_path);

            let mut file = File::create(file_path).unwrap();

            for element in document.select(&selector) {
                file.write_all(element.inner_html().as_bytes()).unwrap();
            }

            *count += 1;
        }
    });
}

fn main() {
    let mut path_of_docking = String::new();
    print!("Inserisci il path del docking: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut path_of_docking).unwrap();

    let path_of_docking = PathBuf::from(path_of_docking.trim());

    let mut count = 0u32;

    let main_path = PathBuf::from(path_of_docking);

    let mut additional_dir = String::new();
    print!("Inserisci una directory: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut additional_dir).unwrap();

    let main_path = main_path.join(additional_dir.trim());

    println!();

    let file_path = PathBuf::from("test");

    read_html_files(main_path, &mut count, file_path);

    println!("COUNT: {}", count);
}
