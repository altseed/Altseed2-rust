use crate::auto_generated_core_binding::*;

/// Engine初期化時の設定
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Config {
    /// 全画面モードかどうか
    fullscreen: bool,
    /// 画面サイズ可変かどうか
    resizable: bool,
    /// ログをコンソールに出力するかどうか
    console_logging: bool,
    /// ログファイル名
    log_filename: Option<String>,
    tool: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            fullscreen: false,
            resizable: false,
            console_logging: true,
            log_filename: Some("Log.txt".to_owned()),
            tool: false,
        }
    }
}

use crate::error::*;
use crate::node::*;
use std::{
    cell::RefCell,
    future::Future,
    marker::PhantomData,
    rc::{Rc, Weak},
    sync::{Arc, Mutex},
};

use crate::runner::{SpinWaker, TaskRunner};

#[derive(Clone)]
pub struct Loader {
    pub(crate) phantom: PhantomData<()>,
}

pub struct CoreContainer {
    pub(crate) core: Core,
    pub(crate) resources: Resources,
    pub(crate) window: Window,
}

impl CoreContainer {
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
}

pub struct Engine {
    container: Rc<RefCell<CoreContainer>>,
    graphics: Graphics,
    renderer: Renderer,
    file: Rc<RefCell<File>>,
    keyboard: Rc<RefCell<Keyboard>>,
    mouse: Rc<RefCell<Mouse>>,
    joystick: Rc<RefCell<Joystick>>,
    sound: Rc<RefCell<SoundMixer>>,
    log: Rc<RefCell<Log>>,
    tool: Option<Rc<RefCell<Tool>>>,
    root_node: Rc<RefCell<BaseNode>>,
    loader: Loader,

    drawn_nodes: list::SortVec<i32, DrawnNode>,
    camera_nodes: list::SortVec<u32, CameraNode>,
    runner: TaskRunner<'static, AltseedError>,

    closed: bool,
}

impl Drop for Engine {
    fn drop(&mut self) {
        Core::terminate();
    }
}

impl Engine {
    fn initialize_core(title: &str, width: i32, height: i32, config: Config) -> Option<Engine> {
        let mut configuration = Configuration::new().unwrap();
        match config.log_filename {
            Some(filename) => configuration
                .set_file_logging_enabled(true)
                .set_log_file_name(filename),
            _ => configuration.set_file_logging_enabled(false),
        }
        .set_is_fullscreen(config.fullscreen)
        .set_is_resizable(config.resizable)
        .set_console_logging_enabled(config.console_logging)
        .set_tool_enabled(config.tool);

        if Core::initialize(title, width, height, &mut configuration) {
            lazy_static! {
                static ref WAKER: std::task::Waker = SpinWaker::waker();
            }

            let e = Engine {
                container: Rc::new(RefCell::new(CoreContainer {
                    core: Core::get_instance()?,
                    resources: Resources::get_instance()?,
                    window: Window::get_instance()?,
                })),
                graphics: Graphics::get_instance()?,
                renderer: Renderer::get_instance()?,
                file: File::get_instance()?,
                keyboard: Keyboard::get_instance()?,
                mouse: Mouse::get_instance()?,
                joystick: Joystick::get_instance()?,
                sound: SoundMixer::get_instance()?,
                log: Log::get_instance()?,
                tool: Tool::get_instance(),
                loader: Loader {
                    phantom: PhantomData,
                },
                root_node: Rc::new(RefCell::new(BaseNode::default())),
                drawn_nodes: list::SortVec::new(),
                camera_nodes: list::SortVec::new(),
                runner: TaskRunner::new(&WAKER),

                closed: false,
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
        Engine::initialize_core(title, width, height, config)
            .ok_or(AltseedError::InitializationFailed)
    }

    /// エンジンを初期化します。
    /// * `title` - ウィンドウのタイトル
    /// * `width` - ウィンドウの横幅
    /// * `height` - ウィンドウの縦幅
    pub fn initialize(title: &str, width: i32, height: i32) -> AltseedResult<Engine> {
        Engine::initialize_core(title, width, height, Config::default())
            .ok_or(AltseedError::InitializationFailed)
    }

    /// ウィンドウを閉じます。
    pub fn close(&mut self) {
        self.closed = true;
    }

    fn do_events(&mut self) -> AltseedResult<bool> {
        if self.closed {
            return Ok(false);
        }

        if self.container.borrow_mut().core.do_event() && self.graphics.do_events() {
            if !self.graphics.begin_frame() {
                return Err(AltseedError::CoreError(
                    "Graphics::begin_frame failed".to_owned(),
                ));
            }

            if let Some(tool) = &self.tool {
                tool.borrow_mut().new_frame();
            }

            return Ok(true);
        }

        Ok(false)
    }

    fn update(&mut self) -> AltseedResult<()> {
        // 非同期処理の継続を取り出す
        self.runner.update()?;

        // 再帰的にノードを更新
        update_node_recursive(
            &unsafe { Rc::from_raw(Rc::into_raw(self.root_node.clone()) as *const _) },
            self,
            None,
        )?;

        // レンダーターゲットの指定
        self.graphics
            .get_command_list()
            .ok_or(AltseedError::CoreError(
                "Graphics::get_command_list failed".to_owned(),
            ))?
            .set_render_target_with_screen();

        // filterとsortを実行
        self.drawn_nodes.update();
        self.camera_nodes.update();

        // for camera in self.camera_nodes.iter().filter_map(Weak::upgrade) {
        //     match camera.borrow_mut().get_target_texture() {
        //         Some(tex) => {
        //             // self.renderer.set_camera(camera.instance())
        //         },
        //         None => self.renderer.reset_camera(),
        //     }
        // }

        // DrawnNodeの呼び出し
        for node in self.drawn_nodes.iter().map(|x| Weak::upgrade(x).unwrap()) {
            node.borrow_mut()
                .on_drawn(&mut self.graphics, &mut self.renderer);
        }

        {
            let mut cmdlist = self
                .graphics
                .get_command_list()
                .ok_or(AltseedError::CoreError(
                    "Graphics::get_command_list failed".to_owned(),
                ))?;

            // コマンドリストに描画
            self.renderer.render(&mut cmdlist);
        }

        if let Some(tool) = &self.tool {
            tool.borrow_mut().render();
        }

        if !self.graphics.end_frame() {
            return Err(AltseedError::CoreError(
                "Graphics::end_frame failed".to_owned(),
            ));
        }

        Ok(())
    }

    /// メインループを実行します
    pub fn run(mut self) -> AltseedResult<()> {
        while self.do_events()? {
            self.update()?;
        }

        Ok(())
    }

    /// 毎フレーム実行する関数を指定してメインループを実行します。
    pub fn run_with<F: FnMut(&mut Engine) -> AltseedResult<()>>(
        mut self,
        mut f: F,
    ) -> AltseedResult<()> {
        while self.do_events()? {
            f(&mut self)?;
            self.update()?;
        }

        Ok(())
    }

    /// DrawnNodeのon_addedの中から呼び出される。
    pub(crate) fn add_drawn_node(&mut self, item: Weak<RefCell<DrawnNode>>) {
        self.drawn_nodes.add(item)
    }

    /// CameraNodeのon_addedの中から呼び出される。
    pub(crate) fn add_camera_node(&mut self, item: Weak<RefCell<CameraNode>>) {
        self.camera_nodes.add(item)
    }

    /// エンジンに新しいノードを追加するフラグを立てます。実際の更新はフレームの終わりに実行されます。
    pub fn add_node<T: Node + 'static>(&mut self, child: Rc<RefCell<T>>) -> AltseedResult<()> {
        self.root_node.borrow_mut().add_child(child)
    }

    /// エンジンに追加されているノードを削除するフラグを立てます。実際の更新はフレームの終わりに実行されます。
    pub fn remove_node<T: Node + 'static>(&mut self, child: &mut T) -> AltseedResult<()> {
        self.root_node.borrow_mut().remove_child(child)
    }

    /// ルートノードを取得します。
    pub fn root_node(&self) -> &Rc<RefCell<BaseNode>> {
        &self.root_node
    }

    /// 非同期タスクを実行します。
    pub fn run_task<F>(&mut self, future: F)
    where
        F: Future<Output = AltseedResult<()>> + 'static,
    {
        self.runner.run(future);
    }

    /// コアの機能
    pub fn core(&self) -> &Rc<RefCell<CoreContainer>> {
        &self.container
    }

    /// ファイルを管理するクラスを取得します。
    pub fn file(&self) -> &Rc<RefCell<File>> {
        &self.file
    }

    /// キーボードを管理するクラスを取得します。
    pub fn keyboard(&self) -> &Rc<RefCell<Keyboard>> {
        &self.keyboard
    }

    /// マウスを管理するクラスを取得します。
    pub fn mouse(&self) -> &Rc<RefCell<Mouse>> {
        &self.mouse
    }

    /// ジョイスティックを管理するクラスを取得します。
    pub fn joystick(&self) -> &Rc<RefCell<Joystick>> {
        &self.joystick
    }

    /// ログを管理するクラスを取得します。
    pub fn log(&self) -> &Rc<RefCell<Log>> {
        &self.log
    }

    /// 音を管理するクラスを取得します。
    pub fn sound(&self) -> &Rc<RefCell<SoundMixer>> {
        &self.sound
    }

    /// ツールを管理するクラスを取得します。
    pub unsafe fn tool(&self) -> &Option<Rc<RefCell<Tool>>> {
        &self.tool
    }

    /// ファイル読み込みを管理する
    pub fn loader(&self) -> &Loader {
        &self.loader
    }
}

impl Loader {
    /// 指定したファイルからテクスチャを読み込みます。
    pub fn load_texture2d(&self, path: &str) -> AltseedResult<Rc<RefCell<Texture2D>>> {
        Texture2D::load(path).ok_or(AltseedError::FailedToCreateResource(
            ResourceType::Texture2D,
            path.to_owned(),
        ))
    }

    /// 指定したファイルから動的にフォントを生成します。
    pub fn load_dynamic_font(&self, path: &str, size: i32) -> AltseedResult<Arc<Mutex<Font>>> {
        Font::load_dynamic_font(path, size).ok_or(AltseedError::FailedToCreateResource(
            ResourceType::Font,
            path.to_owned(),
        ))
    }

    /// 指定したファイルから静的にフォントを生成します。
    pub fn load_static_font(&self, path: &str) -> AltseedResult<Arc<Mutex<Font>>> {
        Font::load_static_font(path).ok_or(AltseedError::FailedToCreateResource(
            ResourceType::Font,
            path.to_owned(),
        ))
    }

    /// 指定したファイルからサウンドを生成します。
    /// # Arguments
    /// - is_decompressed: サウンド生成時に解凍するかどうか(falseの場合、解凍しながら再生されます。)
    pub fn load_sound(
        &self,
        path: &str,
        is_decompressed: bool,
    ) -> AltseedResult<Rc<RefCell<Sound>>> {
        Sound::load(path, is_decompressed).ok_or(AltseedError::FailedToCreateResource(
            ResourceType::Sound,
            path.to_owned(),
        ))
    }

    /// 指定ファイルを読み込んだStaticFileの新しいインスタンスを生成します。
    /// # Arguments
    /// * `path` - 読み込むファイルのパス
    pub fn create_static_file(&self, path: &str) -> AltseedResult<Arc<Mutex<StaticFile>>> {
        StaticFile::create(path).ok_or(AltseedError::FailedToCreateResource(
            ResourceType::StaticFile,
            path.to_owned(),
        ))
    }

    /// 指定ファイルを読み込むStreamFileの新しいインスタンスを生成します。
    /// # Arguments
    /// * `path` - 読み込むファイルのパス
    pub fn create_stream_file(&self, path: &str) -> AltseedResult<Arc<Mutex<StreamFile>>> {
        StreamFile::create(path).ok_or(AltseedError::FailedToCreateResource(
            ResourceType::StreamFile,
            path.to_owned(),
        ))
    }
}
