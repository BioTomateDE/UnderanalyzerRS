use std::{
    env,
    error::Error,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").ok_or("OUT_DIR not set")?;
    let out_dir = PathBuf::from(out_dir);
    let runtime: &str = get_runtime()?;

    println!("cargo::rerun-if-changed=csharp/UnderanalyzerRS.csproj");
    println!("cargo::rerun-if-changed=csharp/FFI");

    let exit_code = Command::new("dotnet")
        .arg("publish")
        .arg("csharp")
        .arg("-c")
        .arg("Release")
        .arg("-r")
        .arg(runtime)
        .arg("-o")
        .arg(&out_dir)
        .spawn()?
        .wait()?;
    if !exit_code.success() {
        return Err("Dotnet failed".into());
    }

    let old: PathBuf = find_lib(&out_dir)?;
    let new = old.with_file_name("dynlib");
    fs::rename(old, new)?;

    Ok(())
}

fn find_lib(out_dir: &Path) -> Result<PathBuf> {
    let dir = std::fs::read_dir(out_dir)?;
    for entry in dir {
        let path = entry?.path();
        println!("Found {path:?}");
        let Some(ext) = path.extension().and_then(OsStr::to_str) else {
            continue;
        };

        if cfg!(windows) && ext == "dll" {
            return Ok(path);
        }

        if cfg!(unix) && ext == "so" {
            return Ok(path);
        }
    }

    Err("Could not find built C# library in OUT_DIR".into())
}

fn get_runtime() -> Result<&'static str> {
    let rt = if cfg!(target_os = "windows") {
        if cfg!(target_arch = "x86") {
            "win-x86"
        } else if cfg!(target_arch = "x86_64") {
            "win-x64"
        } else if cfg!(target_arch = "aarch64") {
            "win-arm64"
        } else {
            return Err("Unknown target architecture".into());
        }
    } else if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            "linux-x64"
        } else if cfg!(target_arch = "arm") {
            "linux-arm"
        } else if cfg!(target_arch = "aarch64") {
            "linux-arm64"
        } else {
            return Err("Unknown target architecture".into());
        }
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "x86_64") {
            "osx-x64"
        } else if cfg!(target_arch = "aarch64") {
            "osx-arm64"
        } else {
            return Err("Unknown target architecture".into());
        }
    } else if cfg!(target_os = "android") {
        if cfg!(target_arch = "x86") {
            "android-x86"
        } else if cfg!(target_arch = "x86_64") {
            "android-x64"
        } else if cfg!(target_arch = "arm") {
            "android-arm"
        } else if cfg!(target_arch = "aarch64") {
            "android-arm64"
        } else {
            return Err("Unknown target architecture".into());
        }
    } else {
        return Err("Unknown target operating system".into());
    };
    Ok(rt)
}
