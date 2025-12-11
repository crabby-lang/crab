use crate::error::{CrabError, CrabResult};
use crate::ui;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn validate_project_name(name: &str) -> CrabResult<()> {
    if name.is_empty() {
        return Err(CrabError::InvalidProjectName(
            "Project name cannot be empty".to_string(),
        ));
    }

    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Err(CrabError::InvalidProjectName(format!(
            "{}. Only alphanumeric characters, hyphens, and underscores are allowed",
            name
        )));
    }

    Ok(())
}

pub fn create_project_structure(name: &str) -> CrabResult<()> {
    validate_project_name(name)?;

    if Path::new(name).exists() {
        return Err(CrabError::ProjectAlreadyExists(name.to_string()));
    }

    let dirs = vec![
        format!("{}/src", name),
        format!("{}/bin", name),
        format!("{}/tests", name),
    ];

    for dir in &dirs {
        fs::create_dir_all(dir)?;
    }

    // Creates a Crab.toml
    let crab_toml_path = format!("{}/Crab.toml", name);
    let crab_toml_content = format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nauthor = \"You <example@mail.com>\"\ndescription = \"A Crabby project\"\n",
        name
    );
    fs::File::create(&crab_toml_path)?.write_all(crab_toml_content.as_bytes())?;

    // Creates a main.crab (example file)
    let main_file = format!("{}/src/main.cb", name);
    let main_content = "print('Hello World!')";
    fs::File::create(&main_file)?.write_all(main_content.as_bytes())?;

    // Creates a .gitignore file
    let gitignore = format!("{}/.gitignore", name);
    let gitignore_content = "/target\n/bin\nCrab.lock\n";
    fs::File::create(&gitignore)?.write_all(gitignore_content.as_bytes())?;

    // Creates a  README.md
    let readme = format!("{}/README.md", name);
    let readme_content = format!(
        "# {}\n\nA Crabby project created with crab.\n\n## Building\n\n```bash\ncrab build\n```\n\n## Running\n\n```bash\ncrab run\n```\n",
        name
    );
    fs::File::create(&readme)?.write_all(readme_content.as_bytes())?;

    ui::print_success(&format!(
        "Project '{}' created successfully!",
        name
    ));
    ui::print_info("Next steps:");
    println!("  1. cd {}", name);
    println!("  2. crab build");
    println!("  3. crab run");

    Ok(())
}

pub fn build_project() -> CrabResult<()> {
    if !Path::new("Crab.toml").exists() {
        return Err(CrabError::ProjectNotFound(
            "Crab.toml not found. Make sure you're in a Crab project directory".to_string(),
        ));
    }

    ui::print_info("Building project..");
    ui::simulate_build_animation();

    Ok(())
}

pub fn run_project() -> CrabResult<()> {
    if !Path::new("Crab.toml").exists() {
        return Err(CrabError::ProjectNotFound(
            "Crab.toml not found. Make sure you're in a Crab project directory".to_string(),
        ));
    }

    ui::print_info("Running project..");
    ui::simulate_download_animation();

    Ok(())
}

pub fn clean_project() -> CrabResult<()> {
    if !Path::new("Crab.toml").exists() {
        return Err(CrabError::ProjectNotFound(
            "Crab.toml not found. Make sure you're in a Crab project directory".to_string(),
        ));
    }

    ui::print_info("Cleaning project..");

    if Path::new("target").exists() {
        fs::remove_dir_all("target")?;
    }

    ui::simulate_cleanup_animation();

    Ok(())
}
