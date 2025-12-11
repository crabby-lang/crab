use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn print_header() {
    println!(
        "{}",
        r#"
    ╔═══════════════════════════════════╗
    ║     🦀 CRAB - Crabby Builder 🦀   ║
    ║     Build. Run. Publish. Thrive.   ║
    ╚═══════════════════════════════════╝
"#
        .cyan()
    );
}

pub fn print_success(msg: &str) {
    println!("{} {}", "✓".green(), msg.green());
}

pub fn print_error(msg: &str) {
    eprintln!("{} {}", "✗".red(), msg.red());
}

pub fn print_warning(msg: &str) {
    println!("{} {}", "⚠".yellow(), msg.yellow());
}

pub fn print_info(msg: &str) {
    println!("{} {}", "ℹ".blue(), msg.blue());
}

pub fn create_progress_bar(total: u64, msg: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.cyan} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(msg.to_string());
    pb.set_position(0);
    pb
}

pub fn simulate_download_animation() {
    let total_steps = 100;
    let pb = create_progress_bar(total_steps, "Downloading dependencies...");

    for i in 0..=total_steps {
        pb.set_position(i);
        std::thread::sleep(Duration::from_millis(30));
    }

    pb.finish_with_message("✓ Dependencies downloaded successfully");
    println!();
}

pub fn simulate_build_animation() {
    let total_steps = 100;
    let pb = create_progress_bar(total_steps, "Compiling project...");

    for i in 0..=total_steps {
        pb.set_position(i);
        std::thread::sleep(Duration::from_millis(25));
    }

    pb.finish_with_message("✓ Build completed successfully");
    println!();
}

pub fn simulate_cleanup_animation() {
    let total_steps = 50;
    let pb = create_progress_bar(total_steps, "Cleaning up artifacts...");

    for i in 0..=total_steps {
        pb.set_position(i);
        std::thread::sleep(Duration::from_millis(20));
    }

    pb.finish_with_message("✓ Cleanup completed");
    println!();
}

pub fn simulate_upload_animation() {
    let total_steps = 100;
    let pb = create_progress_bar(total_steps, "Uploading package...");

    for i in 0..=total_steps {
        pb.set_position(i);
        std::thread::sleep(Duration::from_millis(30));
    }

    pb.finish_with_message("✓ Package uploaded successfully");
    println!();
}
