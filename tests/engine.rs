use altseed2::prelude::*;

#[test]
fn engine() {
    match Engine::initialize("engine", 800, 600) {
        None => eprintln!("Failed to initialize the Engine"),
        Some(mut engine) => {
            let mut count = 0;
            println!("window title: {:?}", engine.get_window_title());
            while count < 20 && engine.do_events() {
                count += 1;
                engine.update();
            }
        }
    }
}
