use std::env;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut args = env::args();

    // Throw away program name
    let _ = args.next().expect("At least one argument must be provided");

    let path = env::current_exe().unwrap();
    let name = path.file_stem().unwrap();
    let full_path = path.canonicalize().unwrap();

    let file_path = [full_path.to_str().unwrap(), ".out"].concat();
    let mut f = File::create(file_path)?;
    f.write_all(name.as_encoded_bytes())?;
    for arg in args {
        f.write_all([" ", &arg].concat().as_bytes())?;
    }

    Ok(())
}
