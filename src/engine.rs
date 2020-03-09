use crate::auto_generated_core_binding::*;
use std::{cell::RefCell, rc::Rc};

pub struct Engine {
    core: Rc<RefCell<Core>>,
    graphics: Rc<RefCell<Graphics>>,
    renderer: Rc<RefCell<Renderer>>,
    resources: Rc<RefCell<Resources>>,
    window: Rc<RefCell<Window>>,
    file: Rc<RefCell<File>>,
    keyboard: Rc<RefCell<Keyboard>>,
    mouse: Rc<RefCell<Mouse>>,
    joystick: Rc<RefCell<Joystick>>,
    sound: Rc<RefCell<SoundMixer>>,
    log: Rc<RefCell<Log>>,
    tool: Rc<RefCell<Tool>>,
}

impl Engine {
    /// Configurationを利用してエンジンを初期化します。
    /// * `title` - ウィンドウのタイトル
    /// * `width` - ウィンドウの横幅
    /// * `height` - ウィンドウの縦幅
    /// * `config` - 初期化時にのみ利用する設定
    pub fn initialize_with(
        title: &str,
        width: i32,
        height: i32,
        config: &mut Configuration,
    ) -> Option<Engine> {
        if Core::initialize(title, width, height, config) {
            Some(Engine {
                core: Core::get_instance()?,
                graphics: Graphics::get_instance()?,
                renderer: Renderer::get_instance()?,
                resources: Resources::get_instance()?,
                window: Window::get_instance()?,
                file: File::get_instance()?,
                keyboard: Keyboard::get_instance()?,
                mouse: Mouse::get_instance()?,
                joystick: Joystick::get_instance()?,
                sound: SoundMixer::get_instance()?,
                log: Log::get_instance()?,
                tool: Tool::get_instance()?,
            })
        } else {
            None
        }
    }

    /// エンジンを初期化します。
    /// * `title` - ウィンドウのタイトル
    /// * `width` - ウィンドウの横幅
    /// * `height` - ウィンドウの縦幅
    pub fn initialize(title: &str, width: i32, height: i32) -> Option<Engine> {
        let config = Configuration::new()?;
        Engine::initialize_with(title, width, height, &mut config.clone().borrow_mut())
    }

    /// エンジンを終了します。
    pub fn terminate(self) {
        Core::terminate();
    }

    pub fn do_events(&mut self) -> bool {
        if !self.core.borrow_mut().do_event() {
            return false;
        }

        if !self.graphics.borrow_mut().do_events() {
            return false;
        }

        true
    }

    /// エンジンを更新します。
    pub fn update(&mut self) -> bool {
        if !self.graphics.borrow_mut().begin_frame() {
            return false;
        }

        if let Some(cmd_list) = self.graphics.borrow_mut().get_command_list() {
            cmd_list.borrow_mut().set_render_target_with_screen();
            self.renderer
                .borrow_mut()
                .render(&mut cmd_list.borrow_mut());
        } else {
            return false;
        }

        if !self.graphics.borrow_mut().end_frame() {
            return false;
        }

        true
    }

    /// ウインドウのタイトルを取得します。
    pub fn get_window_title(&mut self) -> String {
        self.window.borrow_mut().get_title()
    }

    /// ウインドウのタイトルを設定します。
    pub fn set_window_title(&mut self, title: &str) {
        self.window.borrow_mut().set_title(title.to_owned());
    }

    /// フレームレートの制御方法を取得します。
    pub fn get_framerate_mode(&mut self) -> FramerateMode {
        self.core.borrow_mut().get_framerate_mode()
    }

    /// フレームレートの制御方法を設定します。
    pub fn set_framerate_mode(&mut self, mode: FramerateMode) {
        self.core.borrow_mut().set_framerate_mode(mode);
    }

    /// 目標フレームレートを取得します。
    pub fn get_target_fps(&mut self) -> f32 {
        self.core.borrow_mut().get_target_fps()
    }

    /// 目標フレームレートを設定します。
    pub fn set_target_fps(&mut self, fps: f32) {
        self.core.borrow_mut().set_target_fps(fps);
    }

    /// 現在のFPSを取得します。
    pub fn get_current_fps(&mut self) -> f32 {
        self.core.borrow_mut().get_current_fps()
    }

    /// 前のフレームからの経過時間(秒)を取得します。
    pub fn get_delta_second(&mut self) -> f32 {
        self.core.borrow_mut().get_delta_second()
    }

    /// ファイルを管理するクラスを取得します。
    pub fn file(&mut self) -> Rc<RefCell<File>> {
        self.file.clone()
    }

    /// キーボードを管理するクラスを取得します。
    pub fn keyboard(&mut self) -> Rc<RefCell<Keyboard>> {
        self.keyboard.clone()
    }

    /// マウスを管理するクラスを取得します。
    pub fn mouse(&mut self) -> Rc<RefCell<Mouse>> {
        self.mouse.clone()
    }

    /// ジョイスティックを管理するクラスを取得します。
    pub fn joystick(&mut self) -> Rc<RefCell<Joystick>> {
        self.joystick.clone()
    }

    /// ログを管理するクラスを取得します。
    pub fn log(&mut self) -> Rc<RefCell<Log>> {
        self.log.clone()
    }

    /// 音を管理するクラスを取得します。
    pub fn sound(&mut self) -> Rc<RefCell<SoundMixer>> {
        self.sound.clone()
    }

    /// リソースを管理するクラスを取得します。
    pub fn tool(&mut self) -> Rc<RefCell<Tool>> {
        self.tool.clone()
    }
}
