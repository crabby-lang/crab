use std::fs;
use std::io::Write;

pub fn create_project_structure(name: &str) {
    let dirs = vec![
        format!("{}/src", name),
        format!("{}/bin", name),
        format!("{}/tests", name),
    ];
    for dir in dirs {
        fs::crate_dir_all(&dir).expect("Failed to create directories");
    }

    let crab_toml = format!("{}/Crab.toml", name);
    let mut file = fs::File::create(crab_toml).expect("Failed to create Crab.toml");
    file.write_all(b"[package]\nname = \"example\""\nversion = \"0.1.0\"\n").unwrap();

    println!("Project {} initialized successfully!", name);
}
