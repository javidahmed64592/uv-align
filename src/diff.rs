use owo_colors::OwoColorize;
use uv_bump::DependencyChange;

pub fn print_diff(changes: &[DependencyChange], path: String) {
    if changes.is_empty() {
        println!("{} Dependencies already up to date!", "✔".bright_green());
        return;
    }

    for change in changes {
        println!(
            "{} {:<16} {}",
            "-".red(),
            change.name.bold(),
            change.old.red()
        );
        println!(
            "{} {:<16} {}",
            "+".bright_green(),
            change.name.bold(),
            change.new.bright_green()
        );
        println!();
    }

    let apply_command;

    if path == "." {
        apply_command = "uv-bump apply".to_string();
    } else {
        apply_command = format!("uv-bump apply {}", path);
    }

    println!(
        "{} dependency changes. Run `{}` to apply them.",
        changes.len().to_string().bold(),
        apply_command.bright_green(),
    );
}
