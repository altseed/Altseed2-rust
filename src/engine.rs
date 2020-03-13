use crate::auto_generated_core_binding::*;

/// Engine初期化時の設定
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Config {
    /// 全画面モードかどうか
    is_fullscreen_mode: bool,
    /// 画面サイズ可変かどうか
    is_resizable: bool,
    /// ログをコンソールに出力するかどうか
    enabled_console_logging: bool,
    /// ログファイル名
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

use crate::error::*;
use crate::node::*;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub struct Engine {
    pub(crate) core: Core,
    pub(crate) graphics: RefCell<Graphics>,
    pub(crate) renderer: RefCell<Renderer>,
    pub(crate) resources: Resources,
    pub(crate) window: Window,
    file: File,
    keyboard: Keyboard,
    mouse: Mouse,
    joystick: Joystick,
    sound: SoundMixer,
    log: Log,
    tool: Tool,
    root_node: Rc<RefCell<NodeBase>>,
    pub(crate) drawn_nodes: RefCell<Vec<Weak<RefCell<dyn Node>>>>,
    pub(crate) sort_drawn_nodes: bool,
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
            let e = Engine {
                core: Core::get_instance()?,
                graphics: RefCell::new(Graphics::get_instance()?),
                renderer: RefCell::new(Renderer::get_instance()?),
                resources: Resources::get_instance()?,
                window: Window::get_instance()?,
                file: File::get_instance()?,
                keyboard: Keyboard::get_instance()?,
                mouse: Mouse::get_instance()?,
                joystick: Joystick::get_instance()?,
                sound: SoundMixer::get_instance()?,
                log: Log::get_instance()?,
                tool: Tool::get_instance()?,
                root_node: Rc::new(RefCell::new(NodeBase::default())),
                drawn_nodes: RefCell::new(Vec::new()),
                sort_drawn_nodes: false,
            };

            Some(e)
        } else {
            None
        }
    }

    /// Configurationを利用してエンジンを初期化します。
    /// * `title` - ウィンドウのタイトル
    /// * `width` - ウィンドウの横幅
    /// * `height` - ウィンドウの縦幅
    /// * `config` - 初期化時にのみ利用する設定
    pub fn initialize_with(
        title: &str,
        width: i32,
        height: i32,
        config: Config,
    ) -> AltseedResult<Engine> {
        let mut configuration = Configuration::new().unwrap();
        match config.log_filename {
            Some(filename) => configuration
                .set_enabled_file_logging(true)
                .set_log_filename(filename),
            _ => configuration.set_enabled_file_logging(false),
        }
        .set_is_fullscreen_mode(config.is_fullscreen_mode)
        .set_is_resizable(config.is_resizable)
        .set_enabled_console_logging(config.enabled_console_logging);

        Engine::initialize_core(title, width, height, Some(configuration))
            .ok_or(AltseedError::InitializationFailed)
    }

    /// エンジンを初期化します。
    /// * `title` - ウィンドウのタイトル
    /// * `width` - ウィンドウの横幅
    /// * `height` - ウィンドウの縦幅
    pub fn initialize(title: &str, width: i32, height: i32) -> AltseedResult<Engine> {
        Engine::initialize_core(title, width, height, None)
            .ok_or(AltseedError::InitializationFailed)
    }

    pub fn do_events(&mut self) -> bool {
        self.core.do_event() && self.graphics.borrow_mut().do_events()
    }

    /// エンジンを更新します。
    pub fn update(&mut self) -> AltseedResult<()> {
        if !self.graphics.borrow_mut().begin_frame() {
            return Err(AltseedError::CoreError(
                "Graphics::begin_frame failed".to_owned(),
            ));
        }

        // 再帰的にノードを更新
        update_node_base(
            &unsafe { Rc::from_raw(Rc::into_raw(self.root_node.clone()) as *const _) },
            self,
        )?;

        // 生存していないDrawnNodeは取り除く
        self.drawn_nodes.borrow_mut().retain(|x| match x.upgrade() {
            None => false,
            Some(x) => x.borrow().node_base().state == NodeState::Registered,
        });

        // 更新があったらz_order順にソート
        if self.sort_drawn_nodes {
            self.drawn_nodes
                .borrow_mut()
                .sort_by_key(|x| unsafe { x.upgrade().unwrap().borrow().downcast_z_order() });
            self.sort_drawn_nodes = false;
        }

        let mut graphics = self.graphics.borrow_mut();
        let mut renderer = self.renderer.borrow_mut();

        // レンダーターゲットの指定
        graphics
            .get_command_list()
            .ok_or(AltseedError::CoreError(
                "Graphics::get_command_list failed".to_owned(),
            ))?
            .set_render_target_with_screen();

        // DrawnNodeの呼び出し
        for node in self.drawn_nodes.borrow().iter().filter_map(Weak::upgrade) {
            unsafe {
                node.borrow_mut()
                    .downcast_on_drawn(&mut graphics, &mut renderer);
            }
        }

        {
            let mut cmdlist = graphics.get_command_list().ok_or(AltseedError::CoreError(
                "Graphics::get_command_list failed".to_owned(),
            ))?;

            // コマンドリストに描画
            renderer.render(&mut cmdlist);
        }

        if !graphics.end_frame() {
            return Err(AltseedError::CoreError(
                "Graphics::end_frame failed".to_owned(),
            ));
        }

        Ok(())
    }

    /// エンジンに新しいノードを追加します
    pub fn add_node<T: Node + 'static>(&mut self, child: Rc<RefCell<T>>) -> AltseedResult<()> {
        self.root_node.borrow().add_child(child)
    }

    /// エンジンに追加されているノードを削除します。
    pub fn remove_node<T: Node + 'static>(&mut self, child: Rc<RefCell<T>>) -> AltseedResult<()> {
        self.root_node.borrow().remove_child(child)
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

    /// 指定したファイルからテクスチャを読み込みます。
    pub fn create_texture2d(&mut self, path: &str) -> AltseedResult<Rc<RefCell<Texture2D>>> {
        if !self.file().exists(path) {
            return Err(AltseedError::FileNotFound(path.to_owned()));
        }

        Texture2D::load(path).ok_or(AltseedError::FailedToCreateTexture2D(path.to_owned()))
    }

    /// 指定したファイルから動的にフォントを生成します。
    pub fn create_dynamic_font(
        &mut self,
        path: &str,
        size: i32,
    ) -> AltseedResult<Arc<Mutex<Font>>> {
        if !self.file().exists(path) {
            return Err(AltseedError::FileNotFound(path.to_owned()));
        }

        Font::load_dynamic_font(path, size)
            .ok_or(AltseedError::FailedToCreateDynamicFont(path.to_owned()))
    }

    /// 指定したファイルから静的にフォントを生成します。
    pub fn create_static_font(&mut self, path: &str) -> AltseedResult<Arc<Mutex<Font>>> {
        if !self.file().exists(path) {
            return Err(AltseedError::FileNotFound(path.to_owned()));
        }

        Font::load_static_font(path).ok_or(AltseedError::FailedToCreateStaticFont(path.to_owned()))
    }

    /// 指定したファイルからサウンドを生成します。
    /// # Arguments
    /// - is_decompressed: サウンド生成時に解凍するかどうか(falseの場合、解凍しながら再生されます。)
    pub fn create_sound(
        &mut self,
        path: &str,
        is_decompressed: bool,
    ) -> AltseedResult<Rc<RefCell<Sound>>> {
        if !self.file().exists(path) {
            return Err(AltseedError::FileNotFound(path.to_owned()));
        }

        Sound::load(path, is_decompressed).ok_or(AltseedError::FailedToCreateSound(path.to_owned()))
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
