use std::fs;

fn main() -> std::io::Result<()> {
    let flag = if Ok("release".to_owned()) == std::env::var("PROFILE") {
        "RELEASE"
    } else {
        "DEBUG"
    };

    let dll_name = match std::env::consts::OS {
        "windows" => "Altseed_Core.dll",
        "macos" => "libAltseed_Core.dylib",
        _ => "libAltseed_Core.so",
    };

    fs::copy(
        format!("Core/build/DEBUG/{}", dll_name),
        format!("./{}", dll_name),
    )?;

    fs::copy(
        format!("Core/build/{}/{}", flag, dll_name),
        format!("target/{}/{}", flag, dll_name),
    )?;
    println!("cargo:rustc-link-search=native=.");

    Ok(())
}
