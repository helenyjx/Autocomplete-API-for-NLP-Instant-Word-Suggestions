// A command-line tool that plays a game of Macro Polo
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Jiaxin Ying", about = " A Macro Polo game.")]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Jiaxin Ying", about = " A Macro Polo game.")]
    Marco {
        #[clap(short, long)]
        name: String,
    },
}

// This is the main function
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Marco { name }) => {
            if name == "Marco" {
                println!("Polo!");
            } else {
                println!("Marco! {}", name);
            }
        }
        None => println!("No command was used"),
    }
}
