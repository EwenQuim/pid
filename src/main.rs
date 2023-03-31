use clap::Parser;
use sysinfo::{ProcessExt, System, SystemExt};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();

    let system = System::new_all();

    match args.name {
        None => {
            for (pid, process) in system.processes() {
                println!("{} {}", pid, process.name());
            }
        }
        Some(name) => {
            let processes = system.processes_by_exact_name(name.as_str());

            let mut i = 0;
            for p in processes {
                println!("{} {}", p.pid(), p.name());
                i += 1
            }
            if i == 0 {
                eprintln!("No process found with exact name \"{name}\"");
                eprintln!("Here are the processes with a similar name :");
                let processes = system.processes_by_name(name.as_str());
                for p in processes {
                    println!("{} {}", p.pid(), p.name());
                }
            }
        }
    }
}
