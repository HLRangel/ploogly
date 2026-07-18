use crate::commands::command_table::COMMANDS;

/// Prints a list of all template commands and their expected arguments.
pub fn print_commands() {
    println!("Available template commands:\n");
    for cmd in COMMANDS {
        println!("  {} {}", cmd.name, cmd.signature);
        println!("      {}\n", cmd.description);
    }
}
