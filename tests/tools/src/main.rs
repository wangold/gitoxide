use std::{fs, io, io::prelude::*, path::PathBuf};
fn mess_in_the_middle(path: PathBuf) -> io::Result<()> {
    let mut file = fs::OpenOptions::new().read(false).write(true).open(path)?;
    file.seek(io::SeekFrom::Start(file.metadata()?.len() / 2))?;
    file.write_all(b"hello")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let scmd = args.next().expect("sub command");
    match &*scmd {
        "mess-in-the-middle" => mess_in_the_middle(PathBuf::from(args.next().expect("path to file to mess with")))?,
        _ => unreachable!("Unknown subcommand: {}", scmd),
    };
    Ok(())
}
