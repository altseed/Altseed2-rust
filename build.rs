use std::{
    env, error, fs,
    io::{BufWriter, Write},
    path::Path,
};

fn generate_examples() -> Result<(), Box<dyn error::Error>> {
    let timer_frame = 10;

    let examples = vec![
        "hello_world",
        "sprite",
        "text",
        "sound",
        "log",
        "load_async",
        "custom_node",
    ];

    fs::remove_dir_all("./src/examples")?;

    fs::create_dir("./src/examples")?;

    let mut module = BufWriter::new(fs::File::create("./src/examples/mod.rs")?);

    writeln!(
        module,
        r#"//! Examples
// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。
"#
    )?;

    let mut count = 0;

    for name in examples {
        let mod_name = format!("_{0:02}_{1}", count, name);
        count += 1;
        writeln!(module, "pub mod {};", mod_name)?;
        writeln!(module, "mod test{};", mod_name)?;
        let content = fs::read_to_string(format!("./examples/{}.rs", name))?;
        let mut file = BufWriter::new(fs::File::create(format!("./src/examples/{}.rs", mod_name))?);
        let mut test_file = BufWriter::new(fs::File::create(format!(
            "./src/examples/test{}.rs",
            mod_name
        ))?);

        {
            let s = r#"// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。"#;
            writeln!(file, "{}", s)?;
            writeln!(test_file, "{}", s)?;
        }

        let lines: Vec<_> = content
            .split('\n')
            .map(|s| (s, s.starts_with("//!")))
            .collect();
        for (line, _) in lines.iter().filter(|(_, c)| *c) {
            writeln!(file, "{}", line)?;
        }

        writeln!(file, r#"//! ```no_run"#)?;
        writeln!(test_file, r#"//! ```"#)?;

        let mut enabled_timer = false;
        let mut added_timer = false;
        for (line, _) in lines.iter().filter(|(_, c)| !c) {
            if !enabled_timer && line.contains("add timer") {
                writeln!(test_file, r#"//! include!("../tests/timer.rs");"#)?;
                enabled_timer = true;
                continue;
            }

            writeln!(file, "//! {}", line)?;
            writeln!(test_file, "//! {}", line)?;

            if enabled_timer && !added_timer && line.contains("Engine::initialize") {
                writeln!(
                    test_file,
                    r#"//! 
//!     engine.add_node(timer::TimerNode::new({0}))?;"#,
                    timer_frame
                )?;
                added_timer = true;
            }
        }

        writeln!(
            file,
            r#"//! ```
//! 
//! ## Run this example
//! ```shell
//! cargo run --example {}
//! ```
"#,
            name
        )?;
    }

    Ok(())
}

fn copy(path: &str, output: &str) -> Result<(), Box<dyn error::Error>> {
    if Path::new(path).exists() {
        fs::copy(path, output)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    generate_examples()?;

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
