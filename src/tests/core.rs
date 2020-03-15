use crate::auto_generated_core_binding::*;

#[test]
fn core() {
    Core::initialize("core", 800, 600, &mut Configuration::new().unwrap());
    let core = Core::get_instance().unwrap();
    let mut core = core;
    let mut graphics = Graphics::get_instance().unwrap();
    let mut renderer = Renderer::get_instance().unwrap();

    let mut count = 0;
    while count < 20 && core.do_event() && graphics.do_events() {
        count += 1;

        assert!(graphics.begin_frame());

        let mut cmdlist = graphics.get_command_list().unwrap();
        cmdlist.set_render_target_with_screen();
        renderer.render(&mut cmdlist);
        assert!(graphics.end_frame());
    }

    Core::terminate();
}
