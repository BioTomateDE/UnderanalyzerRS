use libgm::prelude::{GMListChunk, GMRef};
use underanalyzer::GameContext;

fn run() -> libgm::Result<()> {
    let mut args = std::env::args().skip(1);
    let data_file_path = args
        .next()
        .ok_or("Please specify data file path via commandline")?;
    if args.next().is_some() {
        return Err("Only expected one commandline argument".into());
    }

    let data = libgm::parse_file(data_file_path)?;
    let ctx = GameContext::new(&data)?;

    for i in 0..data.codes.len() {
        let code_ref = GMRef::from(i);
        let code = data.codes.by_ref(code_ref)?;
        let name = &code.name;
        if !code.is_root() {
            continue;
        }

        let output = match ctx.decompile(code_ref, &data) {
            Ok(str) => str,
            Err(e) => {
                println!("Decompilation of {name:?} failed:\n{}", e.chain_pretty());
                // This can easily be changed between `continue` and `break` depending on needs.
                break;
            }
        };

        println!("Decompilation of {name:?}:\n{output}\n");
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e.chain_pretty());
    }
}
