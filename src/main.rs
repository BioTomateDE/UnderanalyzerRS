use libgm::prelude::GMListChunk;
use underanalyzer::GameContext;

fn run() -> libgm::Result<()> {
    let mut args = std::env::args().skip(1);
    let data_file_path = args.next().ok_or("Please specify data file path via commandline")?;
    if args.next().is_some() {
        return Err("Only expected one commandline argument".into());
    }

    let data = libgm::wad::load_file(data_file_path)?;
    let ctx = GameContext::new(&data)?;

    let dir = "/tmp/UNDERANALYZER/";
    _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir(dir).unwrap();

    for (code_ref, code) in data.codes.element_refs() {
        if !code.is_root() {
            continue;
        }
        let name = data.strings.by_ref(code.name)?;

        let output = match ctx.decompile(code_ref, &data) {
            Ok(str) => str,
            Err(e) => {
                println!("Decompilation of {name:?} failed:\n{}", e.chain());
                // This can easily be changed between `continue` and `break` depending on needs.
                break;
            }
        };

        println!("Decompilation of {name:?}:\n{output}\n");

        std::fs::write(std::path::Path::new(dir).join(name).with_added_extension("gml"), output).unwrap();
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e.chain());
    }
}
