use std::path::PathBuf;

use clap::Parser;
use eu4_patcher::{PatchError, PatchTyp};
use strum::IntoEnumIterator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut input = std::fs::read(&args.input)?;

    if args.list_available {
        for pt in PatchTyp::iter() {
            match pt.find_patch(&input) {
                Ok(_) => {
                    println!("{:?} is available", pt)
                }
                Err(_) => {
                    println!("{:?} is NOT available", pt)
                }
            };
        }
        return Ok(());
    }

    for patch in args.patch {
        let patch = match patch.find_patch(&input) {
            Ok(patch) => patch,
            Err(PatchError::AlreadyApplied) => {
                eprintln!("Skipping {:?} (already applied)", patch);
                continue;
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        };

        patch.apply(&mut input)?;
    }
    let output = args.output_file.unwrap_or(args.input);
    std::fs::write(output, &input)?;
    Ok(())
}

/// A patcher for Europa Universalis 4. Mainly to enable features restricted
/// by ironman
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    input: PathBuf,

    /// Name of the person to greet
    #[arg(short, long, conflicts_with = "patch")]
    list_available: bool,

    /// The patches, that should get applied to the exe
    #[arg(short, long, value_delimiter = ',')]
    patch: Vec<PatchTyp>,

    /// The path to which the patched exe should be writen to. If this is not
    /// specified, it will default to the input. Yout have to specify the
    /// filename for the exe here as well. Just the folder is not enough  
    #[arg(short, long, requires = "patch")]
    output_file: Option<PathBuf>,
}
