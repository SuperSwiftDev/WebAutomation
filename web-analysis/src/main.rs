extern crate super_html_ast as html_ast;

pub mod pass;
pub mod process;
pub mod cli;

fn main() {
    let command_line_interface = cli::CommandLineInterface::load();
    command_line_interface.execute();
}
