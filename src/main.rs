mod commands;

use commands::{
    GenerateDirectedCommand, GenerateUndirectedCommand, GeneratorsDirectedCommand,
    GeneratorsUndirectedCommand, LinkersDirectedCommand, LinkersUndirectedCommand,
};
use crusti_app_helper::{AppHelper, Command};

pub(crate) fn create_app_helper() -> AppHelper<'static> {
    let app_name = option_env!("CARGO_PKG_NAME").unwrap_or("unknown app name");
    let app_version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown version");
    let authors = option_env!("CARGO_PKG_AUTHORS").unwrap_or("unknown authors");
    let mut app = AppHelper::new(
        app_name,
        app_version,
        authors,
        "crusti_g2io, a Graph Generator following an Inner/Outer pattern.",
    );
    let commands: Vec<Box<dyn Command>> = vec![
        Box::new(GenerateDirectedCommand::new()),
        Box::new(GenerateUndirectedCommand::new()),
        Box::new(GeneratorsDirectedCommand::new()),
        Box::new(GeneratorsUndirectedCommand::new()),
        Box::new(LinkersDirectedCommand::new()),
        Box::new(LinkersUndirectedCommand::new()),
    ];
    for c in commands {
        app.add_command(c);
    }
    app
}

fn main() {
    let app = create_app_helper();
    app.launch_app();
}
