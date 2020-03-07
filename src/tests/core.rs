use crate::auto_generated_core_binding::*;

#[test]
fn core() {
    let config = Configuration::new();

    Core::initialize("core", 800, 600, &mut config.borrow_mut());
    let core = Core::get_instance();

    let mut count = 0;
    while count < 20 && core.borrow_mut().do_event() {
        count += 1;
    }

    Core::terminate();
}