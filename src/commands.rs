use crate::core;

pub fn init(name: Option<String>) {
    let project_name = name.unwrap_or("default_project".to_string());
    core::create_project_structure(&project_name);
}

pub fn new(name: String) {
    println!("Creating a new project: {}", name);
    core::create_project_structure(&name);
}

pub fn build() {
    println!("Building the project...");
}

pub fn run() {
    println!("Running the project...");
}

pub fn clean() {
    println!("Cleaning up build artifacts...");
}
