use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let mut args = env::args();

    let name = args.next().expect("At least one argument must be provided");
    let full_path = Path::new(&name);
    let name = full_path.file_stem().unwrap();

    let file_path = [full_path.to_str().unwrap(), ".out"].concat();
    let mut f = File::create(file_path)?;
    f.write_all(name.as_encoded_bytes())?;
    for arg in args {
        f.write_all([" ", &arg].concat().as_bytes())?;
    }

    Ok(())
}
