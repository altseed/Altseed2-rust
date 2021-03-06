use crate::auto_generated_core_binding::*;

/// [Engine](struct.Engine.html)初期化時の設定を表します。
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Config {
    /// 全画面モードかどうか。
    pub fullscreen: bool,
    /// 画面サイズ可変かどうか。
    pub resizable: bool,
    /// ログをコンソールに出力するかどうか。
    pub log_console: bool,
    /// ログファイル名。
    pub log_filename: Option<String>,
    /// ツール機能を有効化するかどうか
    pub tool: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            fullscreen: false,
            resizable: false,
            log_console: true,
            log_filename: Some("Log.txt".to_owned()),
            tool: false,
        }
    }
}

use crate::component::{camera::*, drawn::*, Entity};

use crate::error::*;
use std::{
    collections::{HashSet, VecDeque},
    future::Future,
    marker::PhantomData,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use crate::task::{Cont, SpinWaker};

/// ファイルを読み込む機能を提供します。
#[derive(Clone)]
pub struct Loader {
    pub(crate) phantom: PhantomData<()>,
}

/// AltseedのCoreとの橋渡しやオブジェクトの管理を行います。
pub struct Engine {
    core: Core,
    resources: Resources,
    window: Window,
    graphics: Graphics,
    renderer: Renderer,
    culling: CullingSystem,

    file: File,
    keyboard: Keyboard,
    mouse: Mouse,
    joystick: Joystick,
    sound: crate::sound::SoundMixer,
    log: crate::log::Log,
    tool: Option<Tool>,
    loader: Loader,

    default_camera: RenderedCamera,

    drawn_storage: DrawnStorage,
    camera_storage: CameraStorage,

    context: Context<'static>,
    pins: VecDeque<Pin<Box<dyn Future<Output = Result<Cont, AltseedError>>>>>,

    closed: bool,
    called_begin: bool,
}

lazy_static! {
    /// Engineが初期化済みかどうか
    static ref INITIALIZED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

impl Drop for Engine {
    fn drop(&mut self) {
        if self.called_begin {
            self.graphics.end_frame();
        }
        Core::terminate();
        *INITIALIZED.lock().unwrap() = false;
    }
}

impl Engine {
    /// [Core](../core/index.html)を初期化します。
    fn initialize_core(title: &str, width: i32, height: i32, config: Config) -> Option<Engine> {
        let mut configuration = Configuration::new().unwrap();
        match config.log_filename {
            Some(filename) => {
                configuration.set_file_logging_enabled(true);
                configuration.set_log_file_name(filename);
            }
            _ => {
                configuration.set_file_logging_enabled(false);
            }
        }
        configuration.set_is_fullscreen(config.fullscreen);
        configuration.set_is_resizable(config.resizable);
        configuration.set_console_logging_enabled(config.log_console);
        configuration.set_tool_enabled(config.tool);

        if Core::initialize(title, width, height, &mut configuration) {
            lazy_static! {
                static ref WAKER: std::task::Waker = SpinWaker::waker();
            }

            let e = Engine {
                core: Core::get_instance()?,
                resources: Resources::get_instance()?,
                window: Window::get_instance()?,
                graphics: Graphics::get_instance()?,
                renderer: Renderer::get_instance()?,
                culling: CullingSystem::get_instance()?,

                file: File::__get_instance()?,
                keyboard: Keyboard::__get_instance()?,
                mouse: Mouse::__get_instance()?,
                joystick: Joystick::__get_instance()?,
                sound: crate::sound::SoundMixer::new()?,
                log: crate::log::Log::new()?,
                tool: Tool::__get_instance(),
                loader: Loader {
                    phantom: PhantomData,
                },

                default_camera: RenderedCamera::create()?,

                drawn_storage: DrawnStorage::new(),
                camera_storage: CameraStorage::new(),

                context: Context::from_waker(&WAKER),
                pins: VecDeque::new(),

                closed: false,
                called_begin: false,
            };

            Some(e)
        } else {
            None
        }
    }

    /// [Config](struct.Config.html)を利用してAltseedを初期化します。
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
        let mut initialized = INITIALIZED.lock().unwrap();
        if *initialized {
            return Err(AltseedError::AlreadyInitialized);
        }

        *initialized = true;

        Engine::initialize_core(title, width, height, config)
            .ok_or(AltseedError::InitializationFailed)
    }

    /// Altseedを初期化します。
    /// * `title` - ウィンドウのタイトル
    /// * `width` - ウィンドウの横幅
    /// * `height` - ウィンドウの縦幅
    pub fn initialize(title: &str, width: i32, height: i32) -> AltseedResult<Engine> {
        let mut initialized = INITIALIZED.lock().unwrap();
        if *initialized {
            return Err(AltseedError::AlreadyInitialized);
        }

        *initialized = true;

        Engine::initialize_core(title, width, height, Config::default())
            .ok_or(AltseedError::InitializationFailed)
    }

    /// ウィンドウを閉じます。
    pub fn close(&mut self) {
        self.closed = true;
    }

    fn do_events(&mut self) -> AltseedResult<bool> {
        if self.closed {
            self.closed = false;
            return Ok(false);
        }

        if self.core.do_event() && self.graphics.do_events() {
            if !self.graphics.begin_frame() {
                return Err(AltseedError::CoreError(
                    "Graphics::begin_frame failed".to_owned(),
                ));
            }

            if let Some(tool) = &mut self.tool {
                tool.__new_frame();
            }

            self.called_begin = true;
            return Ok(true);
        }

        Ok(false)
    }

    fn update_task(&mut self) -> AltseedResult<()> {
        if self.pins.is_empty() {
            return Ok(());
        }

        for _ in 0..self.pins.len() {
            let mut p = self.pins.pop_front().unwrap();

            match p.as_mut().poll(&mut self.context) {
                Poll::Pending => self.pins.push_back(p),
                Poll::Ready(Ok(Cont::Fin)) => (),
                Poll::Ready(Ok(Cont::Then(f))) => f(self)?,
                Poll::Ready(Err(e)) => {
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    fn update_graphics(
        default_camera: &mut RenderedCamera,
        culling: &mut CullingSystem,
        graphics: &mut Graphics,
        renderer: &mut Renderer,
        drawn_storage: &mut DrawnStorage,
        camera_storage: &mut CameraStorage,
    ) -> AltseedResult<()> {
        // 削除の反映, 必要があればソート
        CAMERA_STORAGE.with(|camera_storage| {
            camera_storage.borrow_mut().update();
        });

        let mut memoried_updated_drawns: Vec<Entity> = Vec::new();

        let mut screen_target_drawns = Vec::new();

        // 削除の反映, 必要があればソート後にupdaterを実行
        DRAWN_STORAGE.with(|drawn_storage| {
            drawn_storage.borrow_mut().update_with::<AltseedError, _>(
                |e: &Entity, d: &mut DrawnComponent| {
                    d.on_drawing(*e, camera_storage);

                    if d.camera_group() == 0 {
                        screen_target_drawns.push(e.clone());
                    }

                    if d.camera_group.is_updated() {
                        memoried_updated_drawns.push(e.clone());
                    }

                    Ok(())
                },
            )
        })?;

        // カリング用AABBの更新
        culling.update_aabb();

        if !screen_target_drawns.is_empty() {
            // カメラをリセット
            renderer.set_camera(default_camera);
            // スクリーンへ描画
            DRAWN_STORAGE.with::<_, AltseedResult<()>>(|drawn_storage| {
                let culling_ids: HashSet<i32> = culling
                    .get_drawing_rendered_ids()
                    .unwrap()
                    .borrow_mut()
                    .to_vec()
                    .into_iter()
                    .collect();

                let mut drawn_storage = drawn_storage.borrow_mut();
                for e in screen_target_drawns.into_iter() {
                    // screen_target_drawnsへの追加はcomponent削除の反映後なのでunwrap
                    let d = drawn_storage.get_mut(e).unwrap();
                    if culling_ids.contains(&d.culling_id()) {
                        d.draw(graphics, renderer)?;
                    }
                }
                Ok(())
            })?;
            renderer.render();
        }

        // カメラへ描画
        CAMERA_STORAGE.with::<_, AltseedResult<()>>(|camera_storage| {
            for (_, c) in camera_storage.borrow_mut().iter_mut() {
                c.draw(drawn_storage, graphics, renderer, culling)?;
            }
            Ok(())
        })?;

        // z_orderとcamera_groupを更新状態をリセット
        DRAWN_STORAGE.with(|drawn_storage| {
            let mut storage = drawn_storage.borrow_mut();
            for e in memoried_updated_drawns.into_iter() {
                let d = storage.get_mut(e).unwrap();
                d.update_memoried();
            }
        });

        Ok(())
    }

    fn update(&mut self) -> AltseedResult<()> {
        // 非同期処理の継続を取り出す
        self.update_task()?;

        Self::update_graphics(
            &mut self.default_camera,
            &mut self.culling,
            &mut self.graphics,
            &mut self.renderer,
            &mut self.drawn_storage,
            &mut self.camera_storage,
        )?;

        if let Some(tool) = &mut self.tool {
            tool.__render();
        }

        if !self.graphics.end_frame() {
            return Err(AltseedError::CoreError(
                "Graphics::end_frame failed".to_owned(),
            ));
        }
        self.called_begin = false;

        Ok(())
    }

    /// メインループを実行します
    pub fn run(mut self) -> AltseedResult<Engine> {
        while self.do_events()? {
            self.update()?;
        }

        Ok(self)
    }

    /// 毎フレーム実行する関数を指定してメインループを実行します。
    #[inline]
    pub fn run_with<F: FnMut(&mut Engine) -> AltseedResult<()>>(
        mut self,
        mut on_updating: F,
    ) -> AltseedResult<Engine> {
        while self.do_events()? {
            on_updating(&mut self)?;
            self.update()?;
        }

        Ok(self)
    }

    /// 非同期処理を発生させます。
    pub fn spawn_task<F: Future<Output = AltseedResult<Cont>> + 'static>(&mut self, future: F) {
        let future = Pin::from(Box::new(future));
        self.pins.push_back(future);
    }

    /// DrawnComponentを格納しているStorage
    pub fn drawn_storage(&self) -> &DrawnStorage {
        &self.drawn_storage
    }

    /// DrawnComponentを格納しているStorage
    pub fn drawn_storage_mut(&mut self) -> &mut DrawnStorage {
        &mut self.drawn_storage
    }

    /// CameraComponentを格納しているStorage
    pub fn camera_storage(&self) -> &CameraStorage {
        &self.camera_storage
    }

    /// CameraComponentを格納しているStorage
    pub fn camera_storage_mut(&mut self) -> &mut CameraStorage {
        &mut self.camera_storage
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
    pub fn log(&mut self) -> &mut crate::log::Log {
        &mut self.log
    }

    /// 音を管理するクラスを取得します。
    pub fn sound(&mut self) -> &mut crate::sound::SoundMixer {
        &mut self.sound
    }

    /// ツールを管理するクラスを取得します。
    pub unsafe fn tool(&mut self) -> Option<&mut Tool> {
        self.tool.as_mut()
    }

    /// ファイル読み込みを管理するクラスを取得します。
    pub fn loader(&self) -> &Loader {
        &self.loader
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

    /// 指定した種類のリソースの個数を取得します。
    /// # Arguments
    /// * `type_` - 個数を検索するリソースの種類
    pub fn count_resources(&mut self, type_: ResourceType) -> i32 {
        self.resources.get_resources_count(type_)
    }

    /// 登録されたリソースをすべて削除します。
    pub fn clear_resources(&mut self) {
        self.resources.clear()
    }

    /// リソースの再読み込みを行います。
    pub fn reload_resources(&mut self) {
        self.resources.reload()
    }
}

impl Loader {
    /// 指定したファイルからテクスチャを読み込みます。
    pub fn load_texture2d(&self, path: &str) -> AltseedResult<Arc<Mutex<Texture2D>>> {
        Texture2D::__load(path).ok_or(AltseedError::FailedToCreateResource(
            ResourceType::Texture2D,
            path.to_owned(),
        ))
    }

    /// 指定したファイルから動的にフォントを生成します。
    pub fn load_dynamic_font(&self, path: &str, size: i32) -> AltseedResult<Arc<Mutex<Font>>> {
        Font::__load_dynamic_font(path, size).ok_or(AltseedError::FailedToCreateResource(
            ResourceType::Font,
            path.to_owned(),
        ))
    }

    /// 指定したファイルから静的にフォントを生成します。
    pub fn load_static_font(&self, path: &str) -> AltseedResult<Arc<Mutex<Font>>> {
        Font::__load_static_font(path).ok_or(AltseedError::FailedToCreateResource(
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
    ) -> AltseedResult<Arc<Mutex<Sound>>> {
        Sound::__load(path, is_decompressed).ok_or(AltseedError::FailedToCreateResource(
            ResourceType::Sound,
            path.to_owned(),
        ))
    }

    /// 指定ファイルを読み込んだStaticFileの新しいインスタンスを生成します。
    /// # Arguments
    /// * `path` - 読み込むファイルのパス
    pub fn create_static_file(&self, path: &str) -> AltseedResult<Arc<Mutex<StaticFile>>> {
        StaticFile::__create(path).ok_or(AltseedError::FailedToCreateResource(
            ResourceType::StaticFile,
            path.to_owned(),
        ))
    }

    /// 指定ファイルを読み込むStreamFileの新しいインスタンスを生成します。
    /// # Arguments
    /// * `path` - 読み込むファイルのパス
    pub fn create_stream_file(&self, path: &str) -> AltseedResult<Arc<Mutex<StreamFile>>> {
        StreamFile::__create(path).ok_or(AltseedError::FailedToCreateResource(
            ResourceType::StreamFile,
            path.to_owned(),
        ))
    }
}
