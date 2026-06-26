use owo_colors::OwoColorize;
use uv_bump::DependencyChange;

pub fn print_diff(changes: &[DependencyChange]) {
    println!("{}", "Changes:\n".bold().underline());

    for change in changes {
        println!(
            "{} {:<16} {}{}",
            "-".red(),
            change.name.bold(),
            change.operator.clone().unwrap_or_default().red(),
            change.old.red(),
        );
        println!(
            "{} {:<16} {}{}",
            "+".bright_green(),
            change.name.bold(),
            change.operator.clone().unwrap_or_default().bright_green(),
            change.new.bright_green()
        );
        println!();
    }

    println!("{} dependency changes.", changes.len().to_string().bold(),);
}
