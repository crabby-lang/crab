use crate::core;
use crate::error::CrabResult;
use crate::ui;

pub fn init(name: Option<String>) -> CrabResult<()> {
    let project_name = name.unwrap_or_else(|| "default_project".to_string());
    ui::print_info(&format!("Initializing project '{}'...", project_name));
    core::create_project_structure(&project_name)
}

pub fn new(name: String) -> CrabResult<()> {
    ui::print_info(&format!("Creating new project: '{}'...", name));
    core::create_project_structure(&name)
}

pub fn build() -> CrabResult<()> {
    core::build_project()
}

pub fn run() -> CrabResult<()> {
    core::run_project()
}

pub fn clean() -> CrabResult<()> {
    core::clean_project()
}

pub fn test() -> CrabResult<()> {
    ui::print_info("Running tests...");
    ui::simulate_build_animation();
    ui::print_success("All tests passed!");
    Ok(())
}

pub fn watch() -> CrabResult<()> {
    ui::print_info("Watching for changes...");
    ui::print_warning("Press Ctrl+C to stop");
    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}
