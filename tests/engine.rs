use altseed2::prelude::*;

#[test]
fn error_from_loop() -> AltseedResult<()> {
    let engine = Engine::initialize("Error from loop", 800, 600)?;

    let res = engine.run_with(|_| Err(AltseedError::msg("hoge")));

    match res {
        Ok(_) => Err(AltseedError::msg("Error should occurs")),
        Err(e) => {
            println!("{}", e);
            Ok(())
        }
    }
}
