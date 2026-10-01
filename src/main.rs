mod args;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
mod doigen;
use crate::doigen::generatetag;
use figlet_rs::FIGfont;

/*
Gaurav Sablok,
gsablok@proton.me
*/

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("doiTAG");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::Doigen { pathfile, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = generatetag(pathfile).unwrap();
                println!("The command has finished:{}", command);
            });
        }
    }
}
