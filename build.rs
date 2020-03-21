use std::{env, error, fs, path::Path};

fn copy(path: &str, output: &str) -> Result<(), Box<dyn error::Error>> {
    if Path::new(path).exists() {
        fs::copy(path, output)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let flag = env::var("PROFILE")?;

    let dll_name = match env::consts::OS {
        "windows" => "Altseed_Core.dll",
        "macos" => "libAltseed_Core.dylib",
        _ => "libAltseed_Core.so",
    };

    copy(
        &format!("Core/build/DEBUG/{}", dll_name),
        &format!("./{}", dll_name),
    )?;

    copy(
        &format!("Core/build/{}/{}", flag, dll_name),
        &format!("target/{}/{}", flag, dll_name),
    )?;
    println!("cargo:rustc-link-search=native=.");

    Ok(())
}
