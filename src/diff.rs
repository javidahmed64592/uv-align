use owo_colors::OwoColorize;
use uv_bump::DependencyChange;

pub fn print_diff(changes: &[DependencyChange]) {
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

    println!("{} dependency changes.", changes.len().to_string().bold(),);
}
