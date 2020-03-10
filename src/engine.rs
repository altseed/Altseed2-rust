use crate::auto_generated_core_binding::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Config {
    is_fullscreen_mode: bool,
    is_resizable: bool,
    enabled_console_logging: bool,
    log_filename: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            is_fullscreen_mode: false,
            is_resizable: false,
            enabled_console_logging: true,
            log_filename: Some("Log.txt".to_owned()),
        }
    }
}

pub struct Engine {
    core: Core,
    graphics: Graphics,
    renderer: Renderer,
    resources: Resources,
    window: Window,
    file: File,
    keyboard: Keyboard,
    mouse: Mouse,
    joystick: Joystick,
    sound: SoundMixer,
    log: Log,
    tool: Tool,
}

impl Drop for Engine {
    fn drop(&mut self) {
        Core::terminate();
    }
}

impl Engine {
    fn initialize_core(
        title: &str,
        width: i32,
        height: i32,
        config: Option<Configuration>,
    ) -> Option<Engine> {
        if Core::initialize(
            title,
            width,
            height,
            &mut config.unwrap_or(Configuration::new()?),
        ) {
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

    /// Configurationを利用してエンジンを初期化します。
    /// * `title` - ウィンドウのタイトル
    /// * `width` - ウィンドウの横幅
    /// * `height` - ウィンドウの縦幅
    /// * `config` - 初期化時にのみ利用する設定
    pub fn initialize_with(title: &str, width: i32, height: i32, config: Config) -> Option<Engine> {
        let mut configuration = Configuration::new()?;
        configuration.set_is_fullscreen_mode(config.is_fullscreen_mode);
        configuration.set_is_resizable(config.is_resizable);
        configuration.set_enabled_console_logging(config.enabled_console_logging);
        match config.log_filename {
            Some(filename) => {
                configuration.set_enabled_file_logging(true);
                configuration.set_log_filename(filename);
            }
            None => {
                configuration.set_enabled_file_logging(false);
            }
        }

        Engine::initialize_core(title, width, height, Some(configuration))
    }

    /// エンジンを初期化します。
    /// * `title` - ウィンドウのタイトル
    /// * `width` - ウィンドウの横幅
    /// * `height` - ウィンドウの縦幅
    pub fn initialize(title: &str, width: i32, height: i32) -> Option<Engine> {
        Engine::initialize_core(title, width, height, None)
    }

    pub fn do_events(&mut self) -> bool {
        self.core.do_event() && self.graphics.do_events()
    }

    /// エンジンを更新します。
    pub fn update(&mut self) -> bool {
        if !self.graphics.begin_frame() {
            return false;
        }

        if let Some(cmd_list) = self.graphics.get_command_list() {
            let mut cmd_list = cmd_list.borrow_mut();
            cmd_list.set_render_target_with_screen();
            self.renderer.render(&mut cmd_list);
        } else {
            return false;
        }

        if !self.graphics.end_frame() {
            return false;
        }

        true
    }

    /// ウインドウのタイトルを取得します。
    pub fn get_window_title(&mut self) -> String {
        self.window.get_title()
    }

    /// ウインドウのタイトルを設定します。
    pub fn set_window_title(&mut self, title: &str) -> &mut Self {
        self.window.set_title(title.to_owned());
        self
    }

    /// フレームレートの制御方法を取得します。
    pub fn get_framerate_mode(&mut self) -> FramerateMode {
        self.core.get_framerate_mode()
    }

    /// フレームレートの制御方法を設定します。
    pub fn set_framerate_mode(&mut self, mode: FramerateMode) -> &mut Self {
        self.core.set_framerate_mode(mode);
        self
    }

    /// 目標フレームレートを取得します。
    pub fn get_target_fps(&mut self) -> f32 {
        self.core.get_target_fps()
    }

    /// 目標フレームレートを設定します。
    pub fn set_target_fps(&mut self, fps: f32) -> &mut Self {
        self.core.set_target_fps(fps);
        self
    }

    /// 現在のFPSを取得します。
    pub fn get_current_fps(&mut self) -> f32 {
        self.core.get_current_fps()
    }

    /// 前のフレームからの経過時間(秒)を取得します。
    pub fn get_delta_second(&mut self) -> f32 {
        self.core.get_delta_second()
    }

    /// ファイルを管理するクラスを取得します。
    pub fn file(&mut self) -> &mut File {
        &mut self.file
    }

    /// キーボードを管理するクラスを取得します。
    pub fn keyboard(&mut self) -> &mut Keyboard {
        &mut self.keyboard
    }

    /// マウスを管理するクラスを取得します。
    pub fn mouse(&mut self) -> &mut Mouse {
        &mut self.mouse
    }

    /// ジョイスティックを管理するクラスを取得します。
    pub fn joystick(&mut self) -> &mut Joystick {
        &mut self.joystick
    }

    /// ログを管理するクラスを取得します。
    pub fn log(&mut self) -> &mut Log {
        &mut self.log
    }

    /// 音を管理するクラスを取得します。
    pub fn sound(&mut self) -> &mut SoundMixer {
        &mut self.sound
    }

    /// リソースを管理するクラスを取得します。
    pub fn tool(&mut self) -> &mut Tool {
        &mut self.tool
    }
}
