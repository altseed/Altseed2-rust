use crate::auto_generated_core_binding::*;

#[test]
fn core() {
    Core::initialize("core", 800, 600, &mut Configuration::new().unwrap());
    let mut core = Core::get_instance().unwrap();

    let mut count = 0;
    while count < 20 && core.do_event() {
        count += 1;
    }

    Core::terminate();
}
