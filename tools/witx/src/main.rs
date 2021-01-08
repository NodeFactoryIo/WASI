use glob::glob;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;
use structopt::{clap::AppSettings, StructOpt};
use witx::{load, Document, Documentation};

/// Validate and process witx files
#[derive(StructOpt, Debug)]
#[structopt(
    name = "witx",
    version = env!("CARGO_PKG_VERSION"),
    global_settings = &[
        AppSettings::VersionlessSubcommands,
        AppSettings::ColoredHelp
    ]
)]
struct Args {
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Output documentation
    Docs {
        /// Path to root of witx document
        #[structopt(number_of_values = 1, value_name = "INPUT")]
        input: String,
    },
}

pub fn main() {
    let args = Args::from_args();
    pretty_env_logger::init();
    let verbose = args.verbose;

    match args.cmd {
        Command::Docs { input } => {
            for file in glob(&input).expect("Failed to read glob pattern") {
                match file {
                    Ok(path) => {
                        let mut md_path = path.clone();
                        let doc = load_witx(&[path], "input", verbose);

                        md_path.set_extension("md");
                        write_docs(&doc, Path::new(&md_path));

                        println!("Generated {}", md_path.display());
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        if verbose {
                            println!("{:?}", e);
                        }
                        process::exit(1)
                    }
                }
            }
        }
    }
}

fn load_witx(input: &[PathBuf], field_name: &str, verbose: bool) -> Document {
    match load(input) {
        Ok(doc) => {
            if verbose {
                println!("{}: {:?}", field_name, doc);
            }
            doc
        }
        Err(e) => {
            eprintln!("{}", e.report());
            if verbose {
                println!("{:?}", e);
            }
            process::exit(1)
        }
    }
}

fn write_docs<P: AsRef<Path>>(document: &Document, path: P) {
    let mut file = File::create(path.as_ref()).expect("create output file");
    file.write_all(document.to_md().as_bytes())
        .expect("write output file");
}
