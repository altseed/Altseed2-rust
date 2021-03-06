// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
//
//   このファイルは自動生成されました。
//   このファイルへの変更は消失することがあります。
//
//   THIS FILE IS AUTO GENERATED.
//   YOUR COMMITMENT ON THIS FILE WILL BE WIPED.
//
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

#![allow(dead_code)]
#[allow(unused_imports)]
use std::ffi::c_void;
use std::os::raw::*;

const NULLPTR: *mut RawPtr = 0 as *mut RawPtr;

fn decode_string(source: *const u16) -> String {
    unsafe {
        let len = (0..).take_while(|&i| *source.offset(i) != 0).count();
        let slice = std::slice::from_raw_parts(source, len);
        String::from_utf16_lossy(slice)
    }
}

fn encode_string(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{self, Rc};
use std::sync::{self, Arc, Mutex, RwLock};

pub enum RawPtr {}

pub trait HasRawPtr {
    fn self_ptr(&mut self) -> *mut RawPtr;
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct RawPtrStorage(*mut RawPtr);

unsafe impl Send for RawPtrStorage {}
unsafe impl Sync for RawPtrStorage {}

/// 描画方法を表します。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphicsDeviceType {
    /// 実行環境をもとに自動選択
    Default,
    /// DirectX
    DirectX,
    /// Metal
    Metal,
    /// Vulkan
    Vulkan,
}

/// フレームレートモード
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FramerateMode {
    /// 可変フレームレート
    Variable,
    /// 固定フレームレート
    Constant,
}

/// リソースの種類を表します。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    /// StaticFileを表す
    StaticFile,
    /// StreamFileを表す
    StreamFile,
    /// Texture2Dを表す
    Texture2D,
    /// Fontを表す
    Font,
    /// Soundを表す
    Sound,
    MAX,
}

/// キーボードのキーの種類を表します。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keys {
    /// 未知のキー
    Unknown,
    Space,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    /// テンキーの0
    Num0,
    /// テンキーの1
    Num1,
    /// テンキーの2
    Num2,
    /// テンキーの3
    Num3,
    /// テンキーの4
    Num4,
    /// テンキーの5
    Num5,
    /// テンキーの6
    Num6,
    /// テンキーの7
    Num7,
    /// テンキーの8
    Num8,
    /// テンキーの9
    Num9,
    Semicolon,
    Equal,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBracket,
    Backslash,
    RightBracket,
    GraveAccent,
    World1,
    World2,
    Escape,
    Enter,
    Tab,
    Backspace,
    Insert,
    Delete,
    /// 矢印キー右
    Right,
    /// 矢印キー左
    Left,
    /// 矢印キー下
    Down,
    /// 矢印キー上
    Up,
    PageUp,
    PageDown,
    Home,
    End,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    Keypad0,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    KeypadDecimal,
    KeypadDivide,
    KeypadMultiply,
    KeypadSubstract,
    KeypadAdd,
    KeypadEnter,
    KeypadEqual,
    /// 左側のShiftキー
    LeftShift,
    /// 左側のCtrlキー
    LeftControl,
    /// 左側のAltキー
    LeftAlt,
    /// 左側のWinキー
    LeftWin,
    /// 右側のShiftキー
    RightShift,
    /// 右側のCtrlキー
    RightControl,
    /// 右側のAltキー
    RightAlt,
    /// 右側のWinキー
    RightWin,
    Menu,
    Last,
    MAX,
}

/// ボタンの押下状態を表します。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonState {
    /// ボタンが押されていない状態
    Free = 0,
    /// ボタンが押された瞬間の状態
    Push = 1,
    /// ボタンが押されている状態
    Hold = 3,
    /// ボタンが話された瞬間の状態
    Release = 2,
}

/// マウスのボタンの種類を表します。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButtons {
    /// 左ボタン
    ButtonLeft = 0,
    /// 中央ボタン
    ButtonRight = 1,
    /// 右ボタン
    ButtonMiddle = 2,
    /// サブボタン1
    SubButton1 = 3,
    /// サブボタン2
    SubButton2 = 4,
    /// サブボタン3
    SubButton3 = 5,
    /// サブボタン4
    SubButton4 = 6,
    /// サブボタン5
    SubButton5 = 7,
}

/// カーソルの状態を表します。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorMode {
    /// 通常の状態
    Normal = 212993,
    /// 隠れている状態
    Hidden = 212994,
    /// 使用できない状態
    Disable = 212995,
}

/// ジョイスティックの種類を表します。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JoystickType {
    /// 未知の種類のジョイスティック
    Other = 0,
    /// PlayStation4のジョイスティック
    PS4 = 8200,
    /// XBOX360のジョイスティック
    XBOX360 = 8199,
    /// NintendoSwitchの左ジョイスティック
    JoyconL = 8198,
    /// NintendoSwitchの右ジョイスティック
    JoyconR = 8197,
}

/// ジョイスティックのボタンの種類を表します。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JoystickButtonType {
    /// スタートボタン
    Start,
    /// セレクトボタン
    Select,
    /// ホームボタン
    Home,
    /// リリースボタン
    Release,
    /// キャプチャーボタン
    Capture,
    /// 左十字キー上
    LeftUp,
    /// 左十字キー下
    LeftDown,
    /// 左十字キー左
    LeftLeft,
    /// 左十字キー右
    LeftRight,
    /// 左
    LeftPush,
    /// 右十字キー上
    RightUp,
    /// 右十字キー右
    RightRight,
    /// 右十字キー左
    RightLeft,
    /// 右十字キー下
    RightDown,
    /// 右
    RightPush,
    /// Lボタン1
    L1,
    /// Rボタン1
    R1,
    /// Lボタン2
    L2,
    /// Rボタン2
    R2,
    /// Lボタン3
    L3,
    /// Rボタン3
    R3,
    /// 左スタートボタン
    LeftStart,
    /// 右スタートボタン
    RightStart,
    Max,
}

/// ジョイスティックの軸の種類を表します。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JoystickAxisType {
    Start,
    LeftH,
    LeftV,
    RightH,
    RightV,
    L2,
    R2,
    Max,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderStageType {
    Vertex,
    Pixel,
}

/// ビルド済みシェーダの種類を表します
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinShaderType {
    SpriteUnlitVS,
    SpriteUnlitPS,
    FontUnlitPS,
}

/// テキストの描画方向
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WritingDirection {
    /// 縦書き
    Vertical,
    /// 横書き
    Horizontal,
}

///
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderTargetCareType {
    /// クリアしない
    DontCare,
    /// クリアする
    Clear,
}

/// ツール機能で使用する方向
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolDir {
    None = -1,
    /// 左方向
    Left = 0,
    /// 右方向
    Right = 1,
    /// 上方向
    Up = 2,
    /// 下方向
    Down = 3,
}

/// バイナリ演算子を使用して複数の値を結合しないでください
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolCond {
    None = 0,
    /// 常に変数を設定します
    Always = 1,
    /// ランタイムセッションごとに変数を1回設定します（成功した最初の呼び出しのみ）
    Once = 2,
    /// オブジェクト/ウィンドウに永続的に保存されたデータがない場合（.iniファイルにエントリがない場合）、変数を設定します
    FirstUseEver = 4,
    /// オブジェクト/ウィンドウが非表示/非アクティブになった後（または初めて）表示される場合は、変数を設定します
    Appearing = 8,
}

bitflags! {
    pub struct ToolTreeNode: i32 {
        const NONE = 0;
        /// Draw as selected
        const SELECTED = 1;
        ///
        const FRAMED = 2;
        ///
        const ALLOW_ITEM_OVERLAP = 4;
        ///
        const NO_TREE_PUSH_ON_OPEN = 8;
        ///
        const NO_AUTO_OPEN_ON_LOG = 16;
        ///
        const DEFAULT_OPEN = 32;
        ///
        const OPEN_ON_DOUBLE_CLICK = 64;
        ///
        const OPEN_ON_ARROW = 128;
        ///
        const LEAF = 256;
        ///
        const BULLET = 512;
        ///
        const FRAME_PADDING = 1024;
        ///
        const SPAN_AVAIL_WIDTH = 2048;
        ///
        const SPAN_FULL_WIDTH = 4096;
        ///
        const NAV_LEFT_JUMPS_BACK_HERE = 8192;
        const COLLAPSING_HEADER = 26;
    }
}

bitflags! {
    /// ツール機能においてインプットされるテキストの設定を表します
    pub struct ToolInputText: i32 {
        const NONE = 0;
        /// 0123456789.+-*/ を許可します。
        const CHARS_DECIMAL = 1;
        /// 0123456789ABCDEFabcdef を許可します
        const CHARS_HEXADECIMAL = 2;
        /// a..z を A..Z に変換します
        const CHARS_UPPERCASE = 4;
        /// スペースとタブを除外します
        const CHARS_NO_BLANK = 8;
        /// 最初にマウスフォーカスしたときにテキスト全体を選択します
        const AUTO_SELECT_ALL = 16;
        /// （値が変更されるたびにではなく）Enterが押されたときに `true` を返します。 `IsItemDeactivatedAfterEdit()` 関数を調べることを検討してください。
        const ENTER_RETURNS_TRUE = 32;
        /// Tabキーを押したときのコールバック（完了処理のため）
        const CALLBACK_COMPLETION = 64;
        /// 上下矢印を押すとコールバック（履歴処理用）
        const CALLBACK_HISTORY = 128;
        /// 各反復でのコールバック。 ユーザーコードは、カーソル位置を照会し、テキストバッファーを変更できます。
        const CALLBACK_ALWAYS = 256;
        /// 置換または破棄する文字入力のコールバック。 'EventChar'を変更して置換または破棄するか、コールバックで1を返して破棄します。
        const CALLBACK_CHAR_FILTER = 512;
        /// Tabキーを押すと、テキストフィールドに'\t'という文字が入力されます。
        const ALLOW_TAB_INPUT = 1024;
        /// 複数行モードでは、Enterでフォーカスを外し、Ctrl + Enterで新しい行を追加します（デフォルトは反対です：Ctrl + Enterでフォーカスを外し、Enterで行を追加します）。
        const CTRL_ENTER_FOR_NEW_LINE = 2048;
        /// カーソルの水平方向のフォローを無効にする
        const NO_HORIZONTAL_SCROLL = 4096;
        /// インサートモード
        const ALWAYS_INSERT_MODE = 8192;
        /// 読み取り専用モード
        const READ_ONLY = 16384;
        /// パスワードモード。すべての文字を'*'として表示します。
        const PASSWORD = 32768;
        /// 元に戻す/やり直しを無効にします。 アクティブな間は入力テキストがテキストデータを所有していることに注意してください。独自の元に戻す/やり直しスタックを提供する場合は、たとえば ClearActiveID（）を呼び出します。
        const NO_UNDO_REDO = 65536;
        /// 0123456789.+-*/eE (科学表記法の入力) を許可します
        const CHARS_SCIENTIFIC = 131072;
        /// バッファ容量のコールバックはリクエストを変更し（'buf_size 'パラメータ値を超えて）、文字列が大きくなります。 文字列のサイズを変更する必要がある場合に通知します（サイズのキャッシュを保持する文字列タイプの場合）。 コールバックで新しいBufSizeが提供され、それを尊重する必要があります。
        const CALLBACK_RESIZE = 262144;
    }
}

bitflags! {
    /// ツール機能における色の設定を表します
    pub struct ToolColorEdit: i32 {
        const NONE = 0;
        /// `ColorEdit, ColorPicker, ColorButton`: Alphaコンポーネントを無視します（入力ポインターから3つのコンポーネントのみを読み取ります）。
        const NO_ALPHA = 2;
        /// `ColorEdit`: 色付きの正方形をクリックしたときにピッカーを無効にします。
        const NO_PICKER = 4;
        /// `ColorEdit`: 入力/小さなプレビューを右クリックしたときのオプションメニューの切り替えを無効にします。
        const NO_OPTIONS = 8;
        /// `ColorEdit, ColorPicker`: 入力の横にある色付きの正方形プレビューを無効にします。 （例：入力のみを表示する）
        const NO_SMALL_PREVIEW = 16;
        /// `ColorEdit, ColorPicker: 入力スライダー/テキストウィジェットを無効にします（たとえば、小さなプレビューの色付きの四角形のみを表示します）。
        const NO_INPUTS = 32;
        /// `ColorEdit, ColorPicker, ColorButton`: プレビューをホバーするときにツールチップを無効にします。
        const NO_TOOLTIP = 64;
        /// `ColorEdit, ColorPicker`: インラインテキストラベルの表示を無効にします（ラベルは引き続きツールチップとピッカーに転送されます）。
        const NO_LABEL = 128;
        /// `ColorPicker`: ピッカーの右側の大きなカラープレビューを無効にし、代わりに小さな色付きの正方形プレビューを使用します。
        const NO_SIDE_PREVIEW = 256;
        /// `ColorEdit`: ドラッグアンドドロップターゲットを無効にします。 `ColorButton`: ドラッグアンドドロップソースを無効にします。
        const NO_DRAG_DROP = 512;
        /// `ColorEdit, ColorPicker`: ピッカーに垂直アルファバー/グラデーションを表示します。
        const ALPHA_BAR = 65536;
        /// `ColorEdit, ColorPicker, ColorButton`: プレビューを不透明ではなく、チェッカーボード上の透明色として表示します。
        const ALPHA_PREVIEW = 131072;
        /// `ColorEdit, ColorPicker, ColorButton`: 不透明ではなく、半不透明/半市松模様を表示します。
        const ALPHA_PREVIEW_HALF = 262144;
        /// `(WIP) ColorEdit`: 現在、RGBAエディションで0.0f..1.0fの制限のみを無効にします（注：おそらくFloatフラグも使用したいでしょう）。
        const HDR = 524288;
        /// `ColorEdit`: RGB/HSV/Hexの_display_タイプをオーバーライドします。 `ColorPicker`: 1つ以上のRGB/HSV/Hexを使用して任意の組み合わせを選択します。
        const DISPLAY_RGB = 1048576;
        ///
        const DISPLAY_HSV = 2097152;
        ///
        const DISPLAY_HEX = 4194304;
        /// `ColorEdit, ColorPicker, ColorButton`: 0..255としてフォーマットされた_display_値。
        const UINT8 = 8388608;
        /// `ColorEdit, ColorPicker, ColorButton`: _display_値は、0..255整数ではなく0.0f..1.0f浮動小数点としてフォーマットされます。 整数による値の往復はありません。
        const FLOAT = 16777216;
        /// `ColorPicker`: Hueのバー、Sat/Valueの長方形。
        const PICKER_HUE_BAR = 33554432;
        /// `ColorPicker`: Hueのホイール、Sat/Valueの三角形。
        const PICKER_HUE_WHEEL = 67108864;
        /// `ColorEdit, ColorPicker`: RGB形式の入出力データ
        const INPUT_RGB = 134217728;
        /// `ColorEdit, ColorPicker`: HSV形式の入力および出力データ。
        const INPUT_HSV = 268435456;
        /// デフォルトオプション。 `SetColorEditOptions()` を使用して、アプリケーションのデフォルトを設定できます。 意図はおそらくあなたの呼び出しのほとんどでそれらをオーバーライドしたくないことです。 ユーザーがオプションメニューから選択できるようにするか、起動時に`SetColorEditOptions()`を1回呼び出します。
        const OPTIONS_DEFAULT = 177209344;
    }
}

bitflags! {
    pub struct ToolSelectable: i32 {
        ///
        const NONE = 0;
        /// このボタンをクリックしても、親ポップアップウィンドウは閉じません
        const DONT_CLOSE_POPUPS = 1;
        /// 選択可能なフレームはすべての列にまたがることができます（テキストは現在の列に収まります）
        const SPAN_ALL_COLUMNS = 2;
        /// ダブルクリックした場合もプレスイベントを生成
        const ALLOW_DOUBLE_CLICK = 4;
        /// 選択できません、グレー表示されたテキストを表示します
        const DISABLED = 8;
        /// (WIP) ヒットテストにより、後続のウィジェットがこのウィジェットとオーバーラップできるようにします
        const ALLOW_ITEM_OVERLAP = 16;
    }
}

bitflags! {
    /// ツール機能のウィンドウにおける設定を表します
    pub struct ToolWindow: i32 {
        const NONE = 0;
        /// タイトルバーを無効にする
        const NO_TITLE_BAR = 1;
        /// 右下のグリップを使ったユーザーのサイズ変更を無効にします
        const NO_RESIZE = 2;
        /// ユーザーがウィンドウを移動できないようにする
        const NO_MOVE = 4;
        /// スクロールバーを無効にします（ウィンドウはマウスまたはプログラムでスクロールできます）
        const NO_SCROLLBAR = 8;
        /// ユーザーがマウスホイールで垂直にスクロールできないようにします。 子ウィンドウでは、NoScrollbarも設定されていない限り、マウスホイールは親に転送されます。
        const NO_SCROLL_WITH_MOUSE = 16;
        /// ユーザー折りたたみウィンドウをダブルクリックして無効にします
        const NO_COLLAPSE = 32;
        /// フレームごとにコンテンツごとにウィンドウのサイズを変更します
        const ALWAYS_AUTO_RESIZE = 64;
        /// 描画背景色(`WindowBg`など)および外枠を無効にします。 `SetNextWindowBgAlpha(0.0f)`を使用する場合と同様です。
        const NO_BACKGROUND = 128;
        /// .iniファイルの設定をロード/保存しない
        const NO_SAVED_SETTINGS = 256;
        /// パススルーでテストをホバリング、キャッチマウスを無効にします。
        const NO_MOUSE_INPUTS = 512;
        /// メニューバーがあります
        const MENU_BAR = 1024;
        /// 水平スクロールバーの表示を許可します（デフォルトではオフ）。 `Begin()`を呼び出す前に、`SetNextWindowContentSize(Vector2F(width, 0.0f));`を使用して幅を指定できます。
        const HORIZONTAL_SCROLLBAR = 2048;
        /// 非表示から表示状態に移行するときにフォーカスを取得できないようにします
        const NO_FOCUS_ON_APPEARING = 4096;
        /// フォーカスを取得するときにウィンドウを前面に移動することを無効にします（たとえば、クリックするか、プログラムでフォーカスを与える）
        const NO_BRING_TO_FRONT_ON_FOCUS = 8192;
        /// 常に垂直スクロールバーを表示します（`ContentSize.Y < Size.Y`の場合でも）
        const ALWAYS_VERTICAL_SCROLLBAR = 16384;
        /// 常に水平スクロールバーを表示します（`ContentSize.x < Size.x`であっても）
        const ALWAYS_HORIZONTAL_SCROLLBAR = 32768;
        /// 境界線のない子ウィンドウが`style.WindowPadding`を使用するようにします（境界線のない子ウィンドウではデフォルトで無視されるため、より便利です）
        const ALWAYS_USE_WINDOW_PADDING = 65536;
        /// ウィンドウ内にゲームパッド/キーボードナビゲーションはありません
        const NO_NAV_INPUTS = 262144;
        /// ゲームパッド/キーボードナビゲーションでこのウィンドウにフォーカスしない（たとえば、CTRL + TABでスキップ）
        const NO_NAV_FOCUS = 524288;
        /// ###演算子の使用を避けるために、IDに影響を与えずにタイトルに'*'を追加します。 タブ/ドッキングコンテキストで使用する場合、クロージャーでタブが選択され、クロージャーは1フレーム延期され、コードがちらつきなしに（確認ポップアップなどを使用して）クロージャーをキャンセルできるようにします。
        const UNSAVED_DOCUMENT = 1048576;
        const NO_NAV = 786432;
        const NO_DECORATION = 43;
        const NO_INPUTS = 786944;
    }
}

bitflags! {
    /// ツール機能のタブバーにおける設定を表します
    pub struct ToolTabBar: i32 {
        const NONE = 0;
        /// タブを手動でドラッグして並べ替えることができます+リストの最後に新しいタブが追加されます
        const REORDERABLE = 1;
        /// 新しいタブが表示されたら自動的に選択する
        const AUTO_SELECT_NEW_TABS = 2;
        /// ボタンを無効にしてタブリストポップアップを開きます
        const TAB_LIST_POPUP_BUTTON = 4;
        /// マウスの中ボタンでタブを閉じる（p_open！= NULLで送信される）動作を無効にします。 `if（IsItemHovered（）&& IsMouseClicked（2））* p_open = false`を使用すると、ユーザー側でこの動作を再現できます。
        const NO_CLOSE_WITH_MIDDLE_MOUSE_BUTTON = 8;
        /// スクロールボタンを無効にする（フィッティングポリシーが`FittingPolicyScroll`の場合に適用）
        const NO_TAB_LIST_SCROLLING_BUTTONS = 16;
        /// タブをホバーするときにツールチップを無効にする
        const NO_TOOLTIP = 32;
        /// 収まらないタブのサイズを変更する
        const FITTING_POLICY_RESIZE_DOWN = 64;
        /// タブが収まらない場合にスクロールボタンを追加する
        const FITTING_POLICY_SCROLL = 128;
        const FITTING_POLICY_MASK = 192;
        const FITTING_POLICY_DEFAULT = 64;
    }
}

/// ツール機能を使ってフォントを読み込む際の範囲を指定します。ビット演算は行わないでください。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolGlyphRanges {
    Default,
    /// キリル文字
    Cyrillic,
    /// 日本語
    Japanese,
    /// 繁体字中国語
    ChineseFull,
    /// 簡体字中国語
    ChineseSimplifiedCommon,
    /// 韓国語
    Korean,
    /// タイ語
    Thai,
}

/// 音のスペクトル解析に使用する窓関数
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FFTWindow {
    Rectangular,
    Triangle,
    Hamming,
    Hanning,
    Blackman,
    BlackmanHarris,
}

/// ログレベルを表します。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Critical = 5,
    Off = 6,
}

/// ログの範囲を表します。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogCategory {
    Core = 0,
    Graphics = 1,
    Engine = 2,
    User = 3,
}

#[link(name = "Altseed_Core")]
extern "C" {
    fn cbg_Configuration_Constructor_0() -> *mut RawPtr;

    fn cbg_Configuration_GetIsFullscreen(self_ptr: *mut RawPtr) -> bool;
    fn cbg_Configuration_SetIsFullscreen(self_ptr: *mut RawPtr, value: bool) -> ();

    fn cbg_Configuration_GetIsResizable(self_ptr: *mut RawPtr) -> bool;
    fn cbg_Configuration_SetIsResizable(self_ptr: *mut RawPtr, value: bool) -> ();

    fn cbg_Configuration_GetDeviceType(self_ptr: *mut RawPtr) -> c_int;
    fn cbg_Configuration_SetDeviceType(self_ptr: *mut RawPtr, value: c_int) -> ();

    fn cbg_Configuration_GetWaitVSync(self_ptr: *mut RawPtr) -> bool;
    fn cbg_Configuration_SetWaitVSync(self_ptr: *mut RawPtr, value: bool) -> ();

    fn cbg_Configuration_GetConsoleLoggingEnabled(self_ptr: *mut RawPtr) -> bool;
    fn cbg_Configuration_SetConsoleLoggingEnabled(self_ptr: *mut RawPtr, value: bool) -> ();

    fn cbg_Configuration_GetFileLoggingEnabled(self_ptr: *mut RawPtr) -> bool;
    fn cbg_Configuration_SetFileLoggingEnabled(self_ptr: *mut RawPtr, value: bool) -> ();

    fn cbg_Configuration_GetLogFileName(self_ptr: *mut RawPtr) -> *const u16;
    fn cbg_Configuration_SetLogFileName(self_ptr: *mut RawPtr, value: *const u16) -> ();

    fn cbg_Configuration_GetToolEnabled(self_ptr: *mut RawPtr) -> bool;
    fn cbg_Configuration_SetToolEnabled(self_ptr: *mut RawPtr, value: bool) -> ();

    fn cbg_Configuration_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Core_Initialize(
        title: *const u16,
        width: c_int,
        height: c_int,
        config: *mut RawPtr,
    ) -> bool;

    fn cbg_Core_DoEvent(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Core_Terminate() -> ();

    fn cbg_Core_GetInstance() -> *mut RawPtr;

    fn cbg_Core_GetDeltaSecond(self_ptr: *mut RawPtr) -> c_float;

    fn cbg_Core_GetCurrentFPS(self_ptr: *mut RawPtr) -> c_float;

    fn cbg_Core_GetTargetFPS(self_ptr: *mut RawPtr) -> c_float;
    fn cbg_Core_SetTargetFPS(self_ptr: *mut RawPtr, value: c_float) -> ();

    fn cbg_Core_GetFramerateMode(self_ptr: *mut RawPtr) -> c_int;
    fn cbg_Core_SetFramerateMode(self_ptr: *mut RawPtr, value: c_int) -> ();

    fn cbg_Core_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Int8Array_Clear(self_ptr: *mut RawPtr) -> ();

    fn cbg_Int8Array_Resize(self_ptr: *mut RawPtr, size: c_int) -> ();

    fn cbg_Int8Array_GetData(self_ptr: *mut RawPtr) -> *mut RawPtr;

    fn cbg_Int8Array_Assign(self_ptr: *mut RawPtr, ptr: *mut RawPtr, size: c_int) -> ();

    fn cbg_Int8Array_CopyTo(self_ptr: *mut RawPtr, ptr: *mut RawPtr) -> ();

    fn cbg_Int8Array_GetAt(self_ptr: *mut RawPtr, index: c_int) -> c_uchar;

    fn cbg_Int8Array_SetAt(self_ptr: *mut RawPtr, index: c_int, value: c_uchar) -> ();

    fn cbg_Int8Array_Create(size: c_int) -> *mut RawPtr;

    fn cbg_Int8Array_GetCount(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Int8Array_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Int32Array_Clear(self_ptr: *mut RawPtr) -> ();

    fn cbg_Int32Array_Resize(self_ptr: *mut RawPtr, size: c_int) -> ();

    fn cbg_Int32Array_GetData(self_ptr: *mut RawPtr) -> *mut RawPtr;

    fn cbg_Int32Array_Assign(self_ptr: *mut RawPtr, ptr: *mut RawPtr, size: c_int) -> ();

    fn cbg_Int32Array_CopyTo(self_ptr: *mut RawPtr, ptr: *mut RawPtr) -> ();

    fn cbg_Int32Array_GetAt(self_ptr: *mut RawPtr, index: c_int) -> c_int;

    fn cbg_Int32Array_SetAt(self_ptr: *mut RawPtr, index: c_int, value: c_int) -> ();

    fn cbg_Int32Array_Create(size: c_int) -> *mut RawPtr;

    fn cbg_Int32Array_GetCount(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Int32Array_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_VertexArray_Clear(self_ptr: *mut RawPtr) -> ();

    fn cbg_VertexArray_Resize(self_ptr: *mut RawPtr, size: c_int) -> ();

    fn cbg_VertexArray_GetData(self_ptr: *mut RawPtr) -> *mut RawPtr;

    fn cbg_VertexArray_Assign(self_ptr: *mut RawPtr, ptr: *mut RawPtr, size: c_int) -> ();

    fn cbg_VertexArray_CopyTo(self_ptr: *mut RawPtr, ptr: *mut RawPtr) -> ();

    fn cbg_VertexArray_GetAt(self_ptr: *mut RawPtr, index: c_int) -> crate::structs::Vertex;

    fn cbg_VertexArray_SetAt(
        self_ptr: *mut RawPtr,
        index: c_int,
        value: crate::structs::Vertex,
    ) -> ();

    fn cbg_VertexArray_Create(size: c_int) -> *mut RawPtr;

    fn cbg_VertexArray_GetCount(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_VertexArray_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_FloatArray_Clear(self_ptr: *mut RawPtr) -> ();

    fn cbg_FloatArray_Resize(self_ptr: *mut RawPtr, size: c_int) -> ();

    fn cbg_FloatArray_GetData(self_ptr: *mut RawPtr) -> *mut RawPtr;

    fn cbg_FloatArray_Assign(self_ptr: *mut RawPtr, ptr: *mut RawPtr, size: c_int) -> ();

    fn cbg_FloatArray_CopyTo(self_ptr: *mut RawPtr, ptr: *mut RawPtr) -> ();

    fn cbg_FloatArray_GetAt(self_ptr: *mut RawPtr, index: c_int) -> c_float;

    fn cbg_FloatArray_SetAt(self_ptr: *mut RawPtr, index: c_int, value: c_float) -> ();

    fn cbg_FloatArray_Create(size: c_int) -> *mut RawPtr;

    fn cbg_FloatArray_GetCount(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_FloatArray_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Vector2FArray_Clear(self_ptr: *mut RawPtr) -> ();

    fn cbg_Vector2FArray_Resize(self_ptr: *mut RawPtr, size: c_int) -> ();

    fn cbg_Vector2FArray_GetData(self_ptr: *mut RawPtr) -> *mut RawPtr;

    fn cbg_Vector2FArray_Assign(self_ptr: *mut RawPtr, ptr: *mut RawPtr, size: c_int) -> ();

    fn cbg_Vector2FArray_CopyTo(self_ptr: *mut RawPtr, ptr: *mut RawPtr) -> ();

    fn cbg_Vector2FArray_GetAt(self_ptr: *mut RawPtr, index: c_int) -> crate::math::Vector2<f32>;

    fn cbg_Vector2FArray_SetAt(
        self_ptr: *mut RawPtr,
        index: c_int,
        value: crate::math::Vector2<f32>,
    ) -> ();

    fn cbg_Vector2FArray_Create(size: c_int) -> *mut RawPtr;

    fn cbg_Vector2FArray_GetCount(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Vector2FArray_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Resources_GetInstance() -> *mut RawPtr;

    fn cbg_Resources_GetResourcesCount(self_ptr: *mut RawPtr, type_: c_int) -> c_int;

    fn cbg_Resources_Clear(self_ptr: *mut RawPtr) -> ();

    fn cbg_Resources_Reload(self_ptr: *mut RawPtr) -> ();

    fn cbg_Resources_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Cursor_Create(path: *const u16, hot: crate::math::Vector2<i32>) -> *mut RawPtr;

    fn cbg_Cursor_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Keyboard_GetKeyState(self_ptr: *mut RawPtr, key: c_int) -> c_int;

    fn cbg_Keyboard_GetInstance() -> *mut RawPtr;

    fn cbg_Keyboard_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Mouse_GetInstance() -> *mut RawPtr;

    fn cbg_Mouse_GetMouseButtonState(self_ptr: *mut RawPtr, button: c_int) -> c_int;

    fn cbg_Mouse_SetCursorImage(self_ptr: *mut RawPtr, cursor: *mut RawPtr) -> ();

    fn cbg_Mouse_GetPosition(self_ptr: *mut RawPtr) -> crate::math::Vector2<f32>;
    fn cbg_Mouse_SetPosition(self_ptr: *mut RawPtr, value: crate::math::Vector2<f32>) -> ();

    fn cbg_Mouse_GetCursorMode(self_ptr: *mut RawPtr) -> c_int;
    fn cbg_Mouse_SetCursorMode(self_ptr: *mut RawPtr, value: c_int) -> ();

    fn cbg_Mouse_GetWheel(self_ptr: *mut RawPtr) -> c_float;

    fn cbg_Mouse_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Joystick_GetInstance() -> *mut RawPtr;

    fn cbg_Joystick_IsPresent(self_ptr: *mut RawPtr, joystickIndex: c_int) -> bool;

    fn cbg_Joystick_RefreshConnectedState(self_ptr: *mut RawPtr) -> ();

    fn cbg_Joystick_GetButtonStateByIndex(
        self_ptr: *mut RawPtr,
        joystickIndex: c_int,
        buttonIndex: c_int,
    ) -> c_int;

    fn cbg_Joystick_GetButtonStateByType(
        self_ptr: *mut RawPtr,
        joystickIndex: c_int,
        type_: c_int,
    ) -> c_int;

    fn cbg_Joystick_GetJoystickType(self_ptr: *mut RawPtr, index: c_int) -> c_int;

    fn cbg_Joystick_GetAxisStateByIndex(
        self_ptr: *mut RawPtr,
        joystickIndex: c_int,
        axisIndex: c_int,
    ) -> c_float;

    fn cbg_Joystick_GetAxisStateByType(
        self_ptr: *mut RawPtr,
        joystickIndex: c_int,
        type_: c_int,
    ) -> c_float;

    fn cbg_Joystick_GetJoystickName(self_ptr: *mut RawPtr, index: c_int) -> *const u16;

    fn cbg_Joystick_Vibrate(
        self_ptr: *mut RawPtr,
        index: c_int,
        frequency: c_float,
        amplitude: c_float,
    ) -> ();

    fn cbg_Joystick_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Graphics_GetInstance() -> *mut RawPtr;

    fn cbg_Graphics_BeginFrame(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Graphics_EndFrame(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Graphics_DoEvents(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Graphics_GetBuiltinShader(self_ptr: *mut RawPtr) -> *mut RawPtr;

    fn cbg_Graphics_GetClearColor(self_ptr: *mut RawPtr) -> crate::structs::Color;
    fn cbg_Graphics_SetClearColor(self_ptr: *mut RawPtr, value: crate::structs::Color) -> ();

    fn cbg_Graphics_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_TextureBase_Save(self_ptr: *mut RawPtr, path: *const u16) -> bool;

    fn cbg_TextureBase_GetSize(self_ptr: *mut RawPtr) -> crate::math::Vector2<i32>;

    fn cbg_TextureBase_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Texture2D_Load(path: *const u16) -> *mut RawPtr;

    fn cbg_Texture2D_Reload(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Texture2D_GetPath(self_ptr: *mut RawPtr) -> *const u16;

    fn cbg_Texture2D_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_RenderTexture_Create(size: crate::math::Vector2<i32>) -> *mut RawPtr;

    fn cbg_RenderTexture_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Material_Constructor_0() -> *mut RawPtr;

    fn cbg_Material_GetVector4F(
        self_ptr: *mut RawPtr,
        key: *const u16,
    ) -> crate::math::Vector4<f32>;

    fn cbg_Material_SetVector4F(
        self_ptr: *mut RawPtr,
        key: *const u16,
        value: crate::math::Vector4<f32>,
    ) -> ();

    fn cbg_Material_GetMatrix44F(
        self_ptr: *mut RawPtr,
        key: *const u16,
    ) -> crate::math::Matrix44<f32>;

    fn cbg_Material_SetMatrix44F(
        self_ptr: *mut RawPtr,
        key: *const u16,
        value: crate::math::Matrix44<f32>,
    ) -> ();

    fn cbg_Material_GetTexture(self_ptr: *mut RawPtr, key: *const u16) -> *mut RawPtr;

    fn cbg_Material_SetTexture(self_ptr: *mut RawPtr, key: *const u16, value: *mut RawPtr) -> ();

    fn cbg_Material_GetShader(self_ptr: *mut RawPtr, shaderStage: c_int) -> *mut RawPtr;

    fn cbg_Material_SetShader(self_ptr: *mut RawPtr, shader: *mut RawPtr) -> ();

    fn cbg_Material_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Renderer_GetInstance() -> *mut RawPtr;

    fn cbg_Renderer_DrawSprite(self_ptr: *mut RawPtr, sprite: *mut RawPtr) -> ();

    fn cbg_Renderer_DrawText(self_ptr: *mut RawPtr, text: *mut RawPtr) -> ();

    fn cbg_Renderer_DrawPolygon(self_ptr: *mut RawPtr, polygon: *mut RawPtr) -> ();

    fn cbg_Renderer_Render(self_ptr: *mut RawPtr) -> ();

    fn cbg_Renderer_SetCamera(self_ptr: *mut RawPtr, commandList: *mut RawPtr) -> ();

    fn cbg_Renderer_ResetCamera(self_ptr: *mut RawPtr) -> ();

    fn cbg_Renderer_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_CommandList_SetRenderTargetWithScreen(self_ptr: *mut RawPtr) -> ();

    fn cbg_CommandList_GetScreenTexture(self_ptr: *mut RawPtr) -> *mut RawPtr;

    fn cbg_CommandList_SetRenderTarget(
        self_ptr: *mut RawPtr,
        target: *mut RawPtr,
        renderPassParameter: crate::structs::RenderPassParameter,
    ) -> ();

    fn cbg_CommandList_RenderToRenderTarget(self_ptr: *mut RawPtr, material: *mut RawPtr) -> ();

    fn cbg_CommandList_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Rendered_GetTransform(self_ptr: *mut RawPtr) -> crate::math::Matrix44<f32>;
    fn cbg_Rendered_SetTransform(self_ptr: *mut RawPtr, value: crate::math::Matrix44<f32>) -> ();

    fn cbg_Rendered_GetId(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Rendered_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_RenderedSprite_Create() -> *mut RawPtr;

    fn cbg_RenderedSprite_GetTexture(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_RenderedSprite_SetTexture(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_RenderedSprite_GetSrc(self_ptr: *mut RawPtr) -> crate::structs::Rect<f32>;
    fn cbg_RenderedSprite_SetSrc(self_ptr: *mut RawPtr, value: crate::structs::Rect<f32>) -> ();

    fn cbg_RenderedSprite_GetMaterial(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_RenderedSprite_SetMaterial(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_RenderedSprite_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_RenderedText_Create() -> *mut RawPtr;

    fn cbg_RenderedText_GetMaterial(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_RenderedText_SetMaterial(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_RenderedText_GetText(self_ptr: *mut RawPtr) -> *const u16;
    fn cbg_RenderedText_SetText(self_ptr: *mut RawPtr, value: *const u16) -> ();

    fn cbg_RenderedText_GetFont(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_RenderedText_SetFont(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_RenderedText_GetWeight(self_ptr: *mut RawPtr) -> c_float;
    fn cbg_RenderedText_SetWeight(self_ptr: *mut RawPtr, value: c_float) -> ();

    fn cbg_RenderedText_GetColor(self_ptr: *mut RawPtr) -> crate::structs::Color;
    fn cbg_RenderedText_SetColor(self_ptr: *mut RawPtr, value: crate::structs::Color) -> ();

    fn cbg_RenderedText_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_RenderedPolygon_Create() -> *mut RawPtr;

    fn cbg_RenderedPolygon_SetVertexesByVector2F(
        self_ptr: *mut RawPtr,
        vertexes: *mut RawPtr,
    ) -> ();

    fn cbg_RenderedPolygon_GetVertexes(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_RenderedPolygon_SetVertexes(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_RenderedPolygon_GetTexture(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_RenderedPolygon_SetTexture(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_RenderedPolygon_GetSrc(self_ptr: *mut RawPtr) -> crate::structs::Rect<f32>;
    fn cbg_RenderedPolygon_SetSrc(self_ptr: *mut RawPtr, value: crate::structs::Rect<f32>) -> ();

    fn cbg_RenderedPolygon_GetMaterial(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_RenderedPolygon_SetMaterial(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_RenderedPolygon_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_RenderedCamera_Create() -> *mut RawPtr;

    fn cbg_RenderedCamera_GetCenterOffset(self_ptr: *mut RawPtr) -> crate::math::Vector2<f32>;
    fn cbg_RenderedCamera_SetCenterOffset(
        self_ptr: *mut RawPtr,
        value: crate::math::Vector2<f32>,
    ) -> ();

    fn cbg_RenderedCamera_GetTargetTexture(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_RenderedCamera_SetTargetTexture(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_RenderedCamera_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_BuiltinShader_Create(self_ptr: *mut RawPtr, type_: c_int) -> *mut RawPtr;

    fn cbg_BuiltinShader_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Shader_Create(name: *const u16, code: *const u16, shaderStage: c_int) -> *mut RawPtr;

    fn cbg_Shader_GetStageType(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Shader_GetCode(self_ptr: *mut RawPtr) -> *const u16;

    fn cbg_Shader_GetName(self_ptr: *mut RawPtr) -> *const u16;

    fn cbg_Shader_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Glyph_GetTextureSize(self_ptr: *mut RawPtr) -> crate::math::Vector2<i32>;

    fn cbg_Glyph_GetTextureIndex(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Glyph_GetPosition(self_ptr: *mut RawPtr) -> crate::math::Vector2<i32>;

    fn cbg_Glyph_GetSize(self_ptr: *mut RawPtr) -> crate::math::Vector2<i32>;

    fn cbg_Glyph_GetOffset(self_ptr: *mut RawPtr) -> crate::math::Vector2<i32>;

    fn cbg_Glyph_GetGlyphWidth(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Glyph_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Font_LoadDynamicFont(path: *const u16, size: c_int) -> *mut RawPtr;

    fn cbg_Font_LoadStaticFont(path: *const u16) -> *mut RawPtr;

    fn cbg_Font_GenerateFontFile(
        dynamicFontPath: *const u16,
        staticFontPath: *const u16,
        size: c_int,
        characters: *const u16,
    ) -> bool;

    fn cbg_Font_GetGlyph(self_ptr: *mut RawPtr, character: c_int) -> *mut RawPtr;

    fn cbg_Font_GetFontTexture(self_ptr: *mut RawPtr, index: c_int) -> *mut RawPtr;

    fn cbg_Font_GetKerning(self_ptr: *mut RawPtr, c1: c_int, c2: c_int) -> c_int;

    fn cbg_Font_CalcTextureSize(
        self_ptr: *mut RawPtr,
        text: *const u16,
        direction: c_int,
        isEnableKerning: bool,
    ) -> crate::math::Vector2<i32>;

    fn cbg_Font_CreateImageFont(baseFont: *mut RawPtr) -> *mut RawPtr;

    fn cbg_Font_AddImageGlyph(self_ptr: *mut RawPtr, character: c_int, texture: *mut RawPtr) -> ();

    fn cbg_Font_GetImageGlyph(self_ptr: *mut RawPtr, character: c_int) -> *mut RawPtr;

    fn cbg_Font_GetSize(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Font_GetAscent(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Font_GetDescent(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Font_GetLineGap(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Font_GetIsStaticFont(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Font_GetPath(self_ptr: *mut RawPtr) -> *const u16;

    fn cbg_Font_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_CullingSystem_GetInstance() -> *mut RawPtr;

    fn cbg_CullingSystem_UpdateAABB(self_ptr: *mut RawPtr) -> ();

    fn cbg_CullingSystem_GetDrawingRenderedCount(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_CullingSystem_GetDrawingRenderedIds(self_ptr: *mut RawPtr) -> *mut RawPtr;

    fn cbg_CullingSystem_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_GetInstance() -> *mut RawPtr;

    fn cbg_Tool_Begin(self_ptr: *mut RawPtr, name: *const u16, flags: c_int) -> bool;

    fn cbg_Tool_End(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_NewFrame(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_Render(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_Dummy(self_ptr: *mut RawPtr, size: crate::math::Vector2<f32>) -> ();

    fn cbg_Tool_Text(self_ptr: *mut RawPtr, text: *const u16) -> ();

    fn cbg_Tool_TextUnformatted(self_ptr: *mut RawPtr, text: *const u16) -> ();

    fn cbg_Tool_TextWrapped(self_ptr: *mut RawPtr, text: *const u16) -> ();

    fn cbg_Tool_TextColored(
        self_ptr: *mut RawPtr,
        color: crate::math::Vector4<f32>,
        text: *const u16,
    ) -> ();

    fn cbg_Tool_TextDisabled(self_ptr: *mut RawPtr, text: *const u16) -> ();

    fn cbg_Tool_BulletText(self_ptr: *mut RawPtr, text: *const u16) -> ();

    fn cbg_Tool_LabelText(self_ptr: *mut RawPtr, label: *const u16, text: *const u16) -> ();

    fn cbg_Tool_CollapsingHeader(self_ptr: *mut RawPtr, label: *const u16, flags: c_int) -> bool;

    fn cbg_Tool_TreeNode(self_ptr: *mut RawPtr, label: *const u16) -> bool;

    fn cbg_Tool_TreeNodeEx(self_ptr: *mut RawPtr, label: *const u16, flags: c_int) -> bool;

    fn cbg_Tool_TreePop(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_SetNextItemOpen(self_ptr: *mut RawPtr, isOpen: bool, cond: c_int) -> ();

    fn cbg_Tool_Button(
        self_ptr: *mut RawPtr,
        label: *const u16,
        size: crate::math::Vector2<f32>,
    ) -> bool;

    fn cbg_Tool_CheckBox(self_ptr: *mut RawPtr, label: *const u16, isOpen: *mut bool) -> bool;

    fn cbg_Tool_RadioButton(self_ptr: *mut RawPtr, label: *const u16, active: bool) -> bool;

    fn cbg_Tool_ArrowButton(self_ptr: *mut RawPtr, label: *const u16, dir: c_int) -> bool;

    fn cbg_Tool_InvisibleButton(
        self_ptr: *mut RawPtr,
        label: *const u16,
        size: crate::math::Vector2<f32>,
    ) -> bool;

    fn cbg_Tool_Selectable(
        self_ptr: *mut RawPtr,
        label: *const u16,
        selected: *const bool,
        flags: c_int,
    ) -> bool;

    fn cbg_Tool_InputInt(self_ptr: *mut RawPtr, label: *const u16, value: *const c_int) -> bool;

    fn cbg_Tool_InputFloat(self_ptr: *mut RawPtr, label: *const u16, value: *const c_float)
        -> bool;

    fn cbg_Tool_SliderInt(
        self_ptr: *mut RawPtr,
        label: *const u16,
        value: *const c_int,
        speed: c_float,
        valueMin: c_int,
        valueMax: c_int,
    ) -> bool;

    fn cbg_Tool_SliderFloat(
        self_ptr: *mut RawPtr,
        label: *const u16,
        value: *const c_float,
        speed: c_float,
        valueMin: c_float,
        valueMax: c_float,
    ) -> bool;

    fn cbg_Tool_SliderAngle(
        self_ptr: *mut RawPtr,
        label: *const u16,
        angle: *const c_float,
    ) -> bool;

    fn cbg_Tool_VSliderInt(
        self_ptr: *mut RawPtr,
        label: *const u16,
        size: crate::math::Vector2<f32>,
        value: *const c_int,
        valueMin: c_int,
        valueMax: c_int,
    ) -> bool;

    fn cbg_Tool_VSliderFloat(
        self_ptr: *mut RawPtr,
        label: *const u16,
        size: crate::math::Vector2<f32>,
        value: *const c_float,
        valueMin: c_float,
        valueMax: c_float,
    ) -> bool;

    fn cbg_Tool_DragInt(
        self_ptr: *mut RawPtr,
        label: *const u16,
        value: *const c_int,
        speed: c_float,
        valueMin: c_int,
        valueMax: c_int,
    ) -> bool;

    fn cbg_Tool_DragFloat(
        self_ptr: *mut RawPtr,
        label: *const u16,
        value: *const c_float,
        speed: c_float,
        valueMin: c_float,
        valueMax: c_float,
    ) -> bool;

    fn cbg_Tool_OpenPopup(self_ptr: *mut RawPtr, label: *const u16) -> ();

    fn cbg_Tool_BeginPopup(self_ptr: *mut RawPtr, label: *const u16) -> bool;

    fn cbg_Tool_BeginPopupModal(self_ptr: *mut RawPtr, label: *const u16) -> bool;

    fn cbg_Tool_EndPopup(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_BeginChild(
        self_ptr: *mut RawPtr,
        label: *const u16,
        size: crate::math::Vector2<f32>,
        border: bool,
        flags: c_int,
    ) -> bool;

    fn cbg_Tool_EndChild(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_BeginMenuBar(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Tool_EndMenuBar(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_BeginMenu(self_ptr: *mut RawPtr, label: *const u16, enabled: bool) -> bool;

    fn cbg_Tool_EndMenu(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_MenuItem(
        self_ptr: *mut RawPtr,
        label: *const u16,
        shortcut: *const u16,
        selected: bool,
        enabled: bool,
    ) -> bool;

    fn cbg_Tool_BeginTabBar(self_ptr: *mut RawPtr, label: *const u16, flags: c_int) -> bool;

    fn cbg_Tool_EndTabBar(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_BeginTabItem(self_ptr: *mut RawPtr, label: *const u16) -> bool;

    fn cbg_Tool_EndTabItem(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_Indent(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_Unindent(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_Separator(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_SetTooltip(self_ptr: *mut RawPtr, text: *const u16) -> ();

    fn cbg_Tool_BeginTooltip(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_EndTooltip(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_NewLine(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_SameLine(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_PushTextWrapPos(self_ptr: *mut RawPtr, wrapLocalPosX: c_float) -> ();

    fn cbg_Tool_PopTextWrapPos(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_SetNextItemWidth(self_ptr: *mut RawPtr, width: c_float) -> ();

    fn cbg_Tool_PushItemWidth(self_ptr: *mut RawPtr, width: c_float) -> ();

    fn cbg_Tool_PopItemWidth(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_PushButtonRepeat(self_ptr: *mut RawPtr, repeat: bool) -> ();

    fn cbg_Tool_PopButtonRepeat(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_Columns(self_ptr: *mut RawPtr, count: c_int, border: bool) -> ();

    fn cbg_Tool_NextColumn(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_PushID(self_ptr: *mut RawPtr, id: c_int) -> ();

    fn cbg_Tool_PopID(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_IsItemActive(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Tool_IsItemHovered(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Tool_SetScrollHere(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_SetScrollHereX(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_SetScrollHereY(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_GetTextLineHeight(self_ptr: *mut RawPtr) -> c_float;

    fn cbg_Tool_GetFontSize(self_ptr: *mut RawPtr) -> c_float;

    fn cbg_Tool_GetWindowSize(self_ptr: *mut RawPtr) -> crate::math::Vector2<f32>;

    fn cbg_Tool_SetWindowSize(self_ptr: *mut RawPtr, size: crate::math::Vector2<f32>) -> ();

    fn cbg_Tool_IsMousePosValid(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Tool_IsMouseDragging(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Tool_IsMouseDoubleClicked(self_ptr: *mut RawPtr, button: c_int) -> bool;

    fn cbg_Tool_GetMouseDragDelta(
        self_ptr: *mut RawPtr,
        button: c_int,
    ) -> crate::math::Vector2<f32>;

    fn cbg_Tool_ResetMouseDragDelta(self_ptr: *mut RawPtr, button: c_int) -> ();

    fn cbg_Tool_SetNextWindowContentSize(
        self_ptr: *mut RawPtr,
        size: crate::math::Vector2<f32>,
    ) -> ();

    fn cbg_Tool_SetNextWindowSize(self_ptr: *mut RawPtr, size: crate::math::Vector2<f32>) -> ();

    fn cbg_Tool_SetNextWindowPos(self_ptr: *mut RawPtr, pos: crate::math::Vector2<f32>) -> ();

    fn cbg_Tool_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_StreamFile_Create(path: *const u16) -> *mut RawPtr;

    fn cbg_StreamFile_Read(self_ptr: *mut RawPtr, size: c_int) -> c_int;

    fn cbg_StreamFile_GetTempBuffer(self_ptr: *mut RawPtr) -> *mut RawPtr;

    fn cbg_StreamFile_Reload(self_ptr: *mut RawPtr) -> bool;

    fn cbg_StreamFile_GetSize(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_StreamFile_GetCurrentPosition(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_StreamFile_GetTempBufferSize(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_StreamFile_GetIsInPackage(self_ptr: *mut RawPtr) -> bool;

    fn cbg_StreamFile_GetPath(self_ptr: *mut RawPtr) -> *const u16;

    fn cbg_StreamFile_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_StaticFile_Create(path: *const u16) -> *mut RawPtr;

    fn cbg_StaticFile_GetBuffer(self_ptr: *mut RawPtr) -> *mut RawPtr;

    fn cbg_StaticFile_Reload(self_ptr: *mut RawPtr) -> bool;

    fn cbg_StaticFile_GetPath(self_ptr: *mut RawPtr) -> *const u16;

    fn cbg_StaticFile_GetSize(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_StaticFile_GetIsInPackage(self_ptr: *mut RawPtr) -> bool;

    fn cbg_StaticFile_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_File_GetInstance() -> *mut RawPtr;

    fn cbg_File_AddRootDirectory(self_ptr: *mut RawPtr, path: *const u16) -> bool;

    fn cbg_File_AddRootPackageWithPassword(
        self_ptr: *mut RawPtr,
        path: *const u16,
        password: *const u16,
    ) -> bool;

    fn cbg_File_AddRootPackage(self_ptr: *mut RawPtr, path: *const u16) -> bool;

    fn cbg_File_ClearRootDirectories(self_ptr: *mut RawPtr) -> ();

    fn cbg_File_Exists(self_ptr: *mut RawPtr, path: *const u16) -> bool;

    fn cbg_File_Pack(self_ptr: *mut RawPtr, srcPath: *const u16, dstPath: *const u16) -> bool;

    fn cbg_File_PackWithPassword(
        self_ptr: *mut RawPtr,
        srcPath: *const u16,
        dstPath: *const u16,
        password: *const u16,
    ) -> bool;

    fn cbg_File_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Sound_Load(path: *const u16, isDecompressed: bool) -> *mut RawPtr;

    fn cbg_Sound_GetLoopStartingPoint(self_ptr: *mut RawPtr) -> c_float;
    fn cbg_Sound_SetLoopStartingPoint(self_ptr: *mut RawPtr, value: c_float) -> ();

    fn cbg_Sound_GetLoopEndPoint(self_ptr: *mut RawPtr) -> c_float;
    fn cbg_Sound_SetLoopEndPoint(self_ptr: *mut RawPtr, value: c_float) -> ();

    fn cbg_Sound_GetIsLoopingMode(self_ptr: *mut RawPtr) -> bool;
    fn cbg_Sound_SetIsLoopingMode(self_ptr: *mut RawPtr, value: bool) -> ();

    fn cbg_Sound_GetLength(self_ptr: *mut RawPtr) -> c_float;

    fn cbg_Sound_GetPath(self_ptr: *mut RawPtr) -> *const u16;

    fn cbg_Sound_GetIsDecompressed(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Sound_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_SoundMixer_GetInstance() -> *mut RawPtr;

    fn cbg_SoundMixer_Play(self_ptr: *mut RawPtr, sound: *mut RawPtr) -> c_int;

    fn cbg_SoundMixer_GetIsPlaying(self_ptr: *mut RawPtr, id: c_int) -> bool;

    fn cbg_SoundMixer_StopAll(self_ptr: *mut RawPtr) -> ();

    fn cbg_SoundMixer_Stop(self_ptr: *mut RawPtr, id: c_int) -> ();

    fn cbg_SoundMixer_Pause(self_ptr: *mut RawPtr, id: c_int) -> ();

    fn cbg_SoundMixer_Resume(self_ptr: *mut RawPtr, id: c_int) -> ();

    fn cbg_SoundMixer_SetVolume(self_ptr: *mut RawPtr, id: c_int, volume: c_float) -> ();

    fn cbg_SoundMixer_FadeIn(self_ptr: *mut RawPtr, id: c_int, second: c_float) -> ();

    fn cbg_SoundMixer_FadeOut(self_ptr: *mut RawPtr, id: c_int, second: c_float) -> ();

    fn cbg_SoundMixer_Fade(
        self_ptr: *mut RawPtr,
        id: c_int,
        second: c_float,
        targetedVolume: c_float,
    ) -> ();

    fn cbg_SoundMixer_GetIsPlaybackSpeedEnabled(self_ptr: *mut RawPtr, id: c_int) -> bool;

    fn cbg_SoundMixer_SetIsPlaybackSpeedEnabled(
        self_ptr: *mut RawPtr,
        id: c_int,
        isPlaybackSpeedEnabled: bool,
    ) -> ();

    fn cbg_SoundMixer_GetPlaybackSpeed(self_ptr: *mut RawPtr, id: c_int) -> c_float;

    fn cbg_SoundMixer_SetPlaybackSpeed(
        self_ptr: *mut RawPtr,
        id: c_int,
        playbackSpeed: c_float,
    ) -> ();

    fn cbg_SoundMixer_GetPanningPosition(self_ptr: *mut RawPtr, id: c_int) -> c_float;

    fn cbg_SoundMixer_SetPanningPosition(
        self_ptr: *mut RawPtr,
        id: c_int,
        panningPosition: c_float,
    ) -> ();

    fn cbg_SoundMixer_GetPlaybackPosition(self_ptr: *mut RawPtr, id: c_int) -> c_float;

    fn cbg_SoundMixer_SetPlaybackPosition(
        self_ptr: *mut RawPtr,
        id: c_int,
        position: c_float,
    ) -> ();

    fn cbg_SoundMixer_GetSpectrumData(
        self_ptr: *mut RawPtr,
        id: c_int,
        spectrums: *mut RawPtr,
        window: c_int,
    ) -> ();

    fn cbg_SoundMixer_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Log_GetInstance() -> *mut RawPtr;

    fn cbg_Log_Write(
        self_ptr: *mut RawPtr,
        category: c_int,
        level: c_int,
        message: *const u16,
    ) -> ();

    fn cbg_Log_Trace(self_ptr: *mut RawPtr, category: c_int, message: *const u16) -> ();

    fn cbg_Log_Debug(self_ptr: *mut RawPtr, category: c_int, message: *const u16) -> ();

    fn cbg_Log_Info(self_ptr: *mut RawPtr, category: c_int, message: *const u16) -> ();

    fn cbg_Log_Warn(self_ptr: *mut RawPtr, category: c_int, message: *const u16) -> ();

    fn cbg_Log_Error(self_ptr: *mut RawPtr, category: c_int, message: *const u16) -> ();

    fn cbg_Log_Critical(self_ptr: *mut RawPtr, category: c_int, message: *const u16) -> ();

    fn cbg_Log_SetLevel(self_ptr: *mut RawPtr, category: c_int, level: c_int) -> ();

    fn cbg_Log_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Window_GetInstance() -> *mut RawPtr;

    fn cbg_Window_GetTitle(self_ptr: *mut RawPtr) -> *const u16;
    fn cbg_Window_SetTitle(self_ptr: *mut RawPtr, value: *const u16) -> ();

    fn cbg_Window_GetSize(self_ptr: *mut RawPtr) -> crate::math::Vector2<i32>;

    fn cbg_Window_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Collider_Constructor_0() -> *mut RawPtr;

    fn cbg_Collider_GetIsCollidedWith(self_ptr: *mut RawPtr, collider: *mut RawPtr) -> bool;

    fn cbg_Collider_GetPosition(self_ptr: *mut RawPtr) -> crate::math::Vector2<f32>;
    fn cbg_Collider_SetPosition(self_ptr: *mut RawPtr, value: crate::math::Vector2<f32>) -> ();

    fn cbg_Collider_GetRotation(self_ptr: *mut RawPtr) -> c_float;
    fn cbg_Collider_SetRotation(self_ptr: *mut RawPtr, value: c_float) -> ();

    fn cbg_Collider_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_CircleCollider_Constructor_0() -> *mut RawPtr;

    fn cbg_CircleCollider_GetRadius(self_ptr: *mut RawPtr) -> c_float;
    fn cbg_CircleCollider_SetRadius(self_ptr: *mut RawPtr, value: c_float) -> ();

    fn cbg_CircleCollider_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_RectangleCollider_Constructor_0() -> *mut RawPtr;

    fn cbg_RectangleCollider_GetSize(self_ptr: *mut RawPtr) -> crate::math::Vector2<f32>;
    fn cbg_RectangleCollider_SetSize(self_ptr: *mut RawPtr, value: crate::math::Vector2<f32>)
        -> ();

    fn cbg_RectangleCollider_GetCenterPosition(self_ptr: *mut RawPtr) -> crate::math::Vector2<f32>;
    fn cbg_RectangleCollider_SetCenterPosition(
        self_ptr: *mut RawPtr,
        value: crate::math::Vector2<f32>,
    ) -> ();

    fn cbg_RectangleCollider_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_PolygonCollider_Constructor_0() -> *mut RawPtr;

    fn cbg_PolygonCollider_GetVertexes(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_PolygonCollider_SetVertexes(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_PolygonCollider_Release(self_ptr: *mut RawPtr) -> ();

}

/// Altseed2 の設定を表すクラス
#[derive(Debug)]
pub struct Configuration {
    self_ptr: *mut RawPtr,
    is_fullscreen: Option<bool>,
    is_resizable: Option<bool>,
    device_type: Option<GraphicsDeviceType>,
    wait_vsync: Option<bool>,
    console_logging_enabled: Option<bool>,
    file_logging_enabled: Option<bool>,
    log_file_name: Option<String>,
    tool_enabled: Option<bool>,
}

impl HasRawPtr for Configuration {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Configuration {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Configuration {
            self_ptr,
            is_fullscreen: None,
            is_resizable: None,
            device_type: None,
            wait_vsync: None,
            console_logging_enabled: None,
            file_logging_enabled: None,
            log_file_name: None,
            tool_enabled: None,
        })
    }

    /// 新しいインスタンスを生成します。

    pub fn new() -> Option<Configuration> {
        Self::cbg_create_raw(unsafe { cbg_Configuration_Constructor_0() })
    }

    /// 全画面モードかどうかを取得または設定します。
    pub fn get_is_fullscreen(&mut self) -> bool {
        if let Some(value) = &self.is_fullscreen {
            return value.clone();
        }
        let ret = unsafe { cbg_Configuration_GetIsFullscreen(self.self_ptr) };
        ret
    }

    /// 全画面モードかどうかを取得または設定します。
    pub fn set_is_fullscreen(&mut self, value: bool) {
        unsafe { cbg_Configuration_SetIsFullscreen(self.self_ptr, value) }
        self.is_fullscreen = Some(value.clone());
    }

    /// 画面サイズ可変かどうかを取得または設定します。
    pub fn get_is_resizable(&mut self) -> bool {
        if let Some(value) = &self.is_resizable {
            return value.clone();
        }
        let ret = unsafe { cbg_Configuration_GetIsResizable(self.self_ptr) };
        ret
    }

    /// 画面サイズ可変かどうかを取得または設定します。
    pub fn set_is_resizable(&mut self, value: bool) {
        unsafe { cbg_Configuration_SetIsResizable(self.self_ptr, value) }
        self.is_resizable = Some(value.clone());
    }

    /// 描画方法を取得または設定します。
    pub fn get_device_type(&mut self) -> GraphicsDeviceType {
        if let Some(value) = &self.device_type {
            return value.clone();
        }
        let ret = unsafe { cbg_Configuration_GetDeviceType(self.self_ptr) };
        unsafe { std::mem::transmute(ret) }
    }

    /// 描画方法を取得または設定します。
    pub fn set_device_type(&mut self, value: GraphicsDeviceType) {
        unsafe { cbg_Configuration_SetDeviceType(self.self_ptr, value as i32) }
        self.device_type = Some(value.clone());
    }

    /// 垂直同期信号を待つかどうかを取得または設定します。
    pub fn get_wait_vsync(&mut self) -> bool {
        if let Some(value) = &self.wait_vsync {
            return value.clone();
        }
        let ret = unsafe { cbg_Configuration_GetWaitVSync(self.self_ptr) };
        ret
    }

    /// 垂直同期信号を待つかどうかを取得または設定します。
    pub fn set_wait_vsync(&mut self, value: bool) {
        unsafe { cbg_Configuration_SetWaitVSync(self.self_ptr, value) }
        self.wait_vsync = Some(value.clone());
    }

    /// ログをコンソールに出力するかどうかを取得または設定します。
    pub fn get_console_logging_enabled(&mut self) -> bool {
        if let Some(value) = &self.console_logging_enabled {
            return value.clone();
        }
        let ret = unsafe { cbg_Configuration_GetConsoleLoggingEnabled(self.self_ptr) };
        ret
    }

    /// ログをコンソールに出力するかどうかを取得または設定します。
    pub fn set_console_logging_enabled(&mut self, value: bool) {
        unsafe { cbg_Configuration_SetConsoleLoggingEnabled(self.self_ptr, value) }
        self.console_logging_enabled = Some(value.clone());
    }

    /// ログをファイルに出力するかどうかを取得または設定します。
    pub fn get_file_logging_enabled(&mut self) -> bool {
        if let Some(value) = &self.file_logging_enabled {
            return value.clone();
        }
        let ret = unsafe { cbg_Configuration_GetFileLoggingEnabled(self.self_ptr) };
        ret
    }

    /// ログをファイルに出力するかどうかを取得または設定します。
    pub fn set_file_logging_enabled(&mut self, value: bool) {
        unsafe { cbg_Configuration_SetFileLoggingEnabled(self.self_ptr, value) }
        self.file_logging_enabled = Some(value.clone());
    }

    /// ログファイル名を取得または設定します。
    pub fn get_log_file_name(&mut self) -> String {
        if let Some(value) = &self.log_file_name {
            return value.clone();
        }
        let ret = unsafe { cbg_Configuration_GetLogFileName(self.self_ptr) };
        decode_string(ret)
    }

    /// ログファイル名を取得または設定します。
    pub fn set_log_file_name(&mut self, value: String) {
        unsafe { cbg_Configuration_SetLogFileName(self.self_ptr, encode_string(&value).as_ptr()) }
        self.log_file_name = Some(value.clone());
    }

    /// ツール機能を使用するかどうかを取得または設定します。
    pub fn get_tool_enabled(&mut self) -> bool {
        if let Some(value) = &self.tool_enabled {
            return value.clone();
        }
        let ret = unsafe { cbg_Configuration_GetToolEnabled(self.self_ptr) };
        ret
    }

    /// ツール機能を使用するかどうかを取得または設定します。
    pub fn set_tool_enabled(&mut self, value: bool) {
        unsafe { cbg_Configuration_SetToolEnabled(self.self_ptr, value) }
        self.tool_enabled = Some(value.clone());
    }
}

impl Drop for Configuration {
    fn drop(&mut self) {
        unsafe { cbg_Configuration_Release(self.self_ptr) };
    }
}

/// C++のCoreとの仲介を担うクラス
#[derive(Debug)]
pub(crate) struct Core {
    self_ptr: *mut RawPtr,
    target_fps: Option<f32>,
    framerate_mode: Option<FramerateMode>,
}

impl HasRawPtr for Core {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Core {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Core {
            self_ptr,
            target_fps: None,
            framerate_mode: None,
        })
    }

    /// 初期化処理を行います。
    /// # Arguments
    /// * `title` - ウィンドウの左上に表示されるウィンドウん名
    /// * `width` - ウィンドウの横幅
    /// * `height` - ウィンドウの縦幅
    /// * `config` - 初期化時の設定

    pub fn initialize(title: &str, width: i32, height: i32, config: &mut Configuration) -> bool {
        let ret = unsafe {
            cbg_Core_Initialize(
                encode_string(&title).as_ptr(),
                width,
                height,
                config.self_ptr(),
            )
        };
        ret
    }

    /// イベントを実行します。

    pub fn do_event(&mut self) -> bool {
        let ret = unsafe { cbg_Core_DoEvent(self.self_ptr) };
        ret
    }

    /// 終了処理を行います。

    pub fn terminate() -> () {
        unsafe { cbg_Core_Terminate() }
    }

    /// インスタンスを取得します。

    pub(crate) fn get_instance() -> Option<Core> {
        let ret = unsafe { cbg_Core_GetInstance() };
        Core::cbg_create_raw(ret)
    }

    /// 前のフレームからの経過時間(秒)を取得します。
    pub fn get_delta_second(&mut self) -> f32 {
        let ret = unsafe { cbg_Core_GetDeltaSecond(self.self_ptr) };
        ret
    }

    /// 現在のFPSを取得します。
    pub fn get_current_fps(&mut self) -> f32 {
        let ret = unsafe { cbg_Core_GetCurrentFPS(self.self_ptr) };
        ret
    }

    /// 目標のFPSを取得または設定します。
    pub fn get_target_fps(&mut self) -> f32 {
        if let Some(value) = &self.target_fps {
            return value.clone();
        }
        let ret = unsafe { cbg_Core_GetTargetFPS(self.self_ptr) };
        ret
    }

    /// 目標のFPSを取得または設定します。
    pub fn set_target_fps(&mut self, value: f32) {
        unsafe { cbg_Core_SetTargetFPS(self.self_ptr, value) }
        self.target_fps = Some(value.clone());
    }

    /// フレームレートモードを取得または設定します。デフォルトでは可変フレームレートです。
    pub fn get_framerate_mode(&mut self) -> FramerateMode {
        if let Some(value) = &self.framerate_mode {
            return value.clone();
        }
        let ret = unsafe { cbg_Core_GetFramerateMode(self.self_ptr) };
        unsafe { std::mem::transmute(ret) }
    }

    /// フレームレートモードを取得または設定します。デフォルトでは可変フレームレートです。
    pub fn set_framerate_mode(&mut self, value: FramerateMode) {
        unsafe { cbg_Core_SetFramerateMode(self.self_ptr, value as i32) }
        self.framerate_mode = Some(value.clone());
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        unsafe { cbg_Core_Release(self.self_ptr) };
    }
}

/// 8ビット整数の配列のクラスを表します。
#[derive(Debug)]
pub(crate) struct Int8Array {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for Int8Array {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Int8Array {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Int8Array { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static INT8ARRAY_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Int8Array>>>> = RefCell::new(HashMap::new());
        }
        INT8ARRAY_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// データをクリアします。

    pub fn clear(&mut self) -> () {
        unsafe { cbg_Int8Array_Clear(self.self_ptr) }
    }

    /// サイズを変更します。
    /// # Arguments
    /// * `size` - 要素数

    pub fn resize(&mut self, size: i32) -> () {
        unsafe { cbg_Int8Array_Resize(self.self_ptr, size) }
    }

    pub fn get_data(&mut self) -> *mut RawPtr {
        let ret = unsafe { cbg_Int8Array_GetData(self.self_ptr) };
        ret
    }

    pub fn assign(&mut self, ptr: *mut RawPtr, size: i32) -> () {
        unsafe { cbg_Int8Array_Assign(self.self_ptr, ptr, size) }
    }

    /// データを指定したポインタにコピーします。
    /// # Arguments
    /// * `ptr` - ポインタ

    pub fn copy_to(&mut self, ptr: *mut RawPtr) -> () {
        unsafe { cbg_Int8Array_CopyTo(self.self_ptr, ptr) }
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス

    pub fn get_at(&mut self, index: i32) -> u8 {
        let ret = unsafe { cbg_Int8Array_GetAt(self.self_ptr, index) };
        ret
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス
    /// * `value` - 値

    pub fn set_at(&mut self, index: i32, value: u8) -> () {
        unsafe { cbg_Int8Array_SetAt(self.self_ptr, index, value) }
    }

    /// インスタンスを作成します。
    /// # Arguments
    /// * `size` - 要素数

    pub fn create(size: i32) -> Option<Rc<RefCell<Int8Array>>> {
        let ret = unsafe { cbg_Int8Array_Create(size) };
        {
            let ret = Int8Array::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// 格納されている要素の数を取得します。
    pub fn get_count(&mut self) -> i32 {
        let ret = unsafe { cbg_Int8Array_GetCount(self.self_ptr) };
        ret
    }
}

impl Drop for Int8Array {
    fn drop(&mut self) {
        unsafe { cbg_Int8Array_Release(self.self_ptr) };
    }
}

/// 32ビット整数の配列のクラスを表します。
#[derive(Debug)]
pub(crate) struct Int32Array {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for Int32Array {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Int32Array {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Int32Array { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static INT32ARRAY_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Int32Array>>>> = RefCell::new(HashMap::new());
        }
        INT32ARRAY_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// データをクリアします。

    pub fn clear(&mut self) -> () {
        unsafe { cbg_Int32Array_Clear(self.self_ptr) }
    }

    /// サイズを変更します。
    /// # Arguments
    /// * `size` - 要素数

    pub fn resize(&mut self, size: i32) -> () {
        unsafe { cbg_Int32Array_Resize(self.self_ptr, size) }
    }

    pub fn get_data(&mut self) -> *mut RawPtr {
        let ret = unsafe { cbg_Int32Array_GetData(self.self_ptr) };
        ret
    }

    pub fn assign(&mut self, ptr: *mut RawPtr, size: i32) -> () {
        unsafe { cbg_Int32Array_Assign(self.self_ptr, ptr, size) }
    }

    /// データを指定したポインタにコピーします。
    /// # Arguments
    /// * `ptr` - ポインタ

    pub fn copy_to(&mut self, ptr: *mut RawPtr) -> () {
        unsafe { cbg_Int32Array_CopyTo(self.self_ptr, ptr) }
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス

    pub fn get_at(&mut self, index: i32) -> i32 {
        let ret = unsafe { cbg_Int32Array_GetAt(self.self_ptr, index) };
        ret
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス
    /// * `value` - 値

    pub fn set_at(&mut self, index: i32, value: i32) -> () {
        unsafe { cbg_Int32Array_SetAt(self.self_ptr, index, value) }
    }

    /// インスタンスを作成します。
    /// # Arguments
    /// * `size` - 要素数

    pub fn create(size: i32) -> Option<Rc<RefCell<Int32Array>>> {
        let ret = unsafe { cbg_Int32Array_Create(size) };
        {
            let ret = Int32Array::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// 格納されている要素の数を取得します。
    pub fn get_count(&mut self) -> i32 {
        let ret = unsafe { cbg_Int32Array_GetCount(self.self_ptr) };
        ret
    }
}

impl Drop for Int32Array {
    fn drop(&mut self) {
        unsafe { cbg_Int32Array_Release(self.self_ptr) };
    }
}

/// 頂点データの配列のクラスを表します。
#[derive(Debug)]
pub(crate) struct VertexArray {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for VertexArray {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl VertexArray {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(VertexArray { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static VERTEXARRAY_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<VertexArray>>>> = RefCell::new(HashMap::new());
        }
        VERTEXARRAY_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// データをクリアします。

    pub fn clear(&mut self) -> () {
        unsafe { cbg_VertexArray_Clear(self.self_ptr) }
    }

    /// サイズを変更します。
    /// # Arguments
    /// * `size` - 要素数

    pub fn resize(&mut self, size: i32) -> () {
        unsafe { cbg_VertexArray_Resize(self.self_ptr, size) }
    }

    pub fn get_data(&mut self) -> *mut RawPtr {
        let ret = unsafe { cbg_VertexArray_GetData(self.self_ptr) };
        ret
    }

    pub fn assign(&mut self, ptr: *mut RawPtr, size: i32) -> () {
        unsafe { cbg_VertexArray_Assign(self.self_ptr, ptr, size) }
    }

    /// データを指定したポインタにコピーします。
    /// # Arguments
    /// * `ptr` - ポインタ

    pub fn copy_to(&mut self, ptr: *mut RawPtr) -> () {
        unsafe { cbg_VertexArray_CopyTo(self.self_ptr, ptr) }
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス

    pub fn get_at(&mut self, index: i32) -> crate::structs::Vertex {
        let ret = unsafe { cbg_VertexArray_GetAt(self.self_ptr, index) };
        ret
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス
    /// * `value` - 値

    pub fn set_at(&mut self, index: i32, mut value: crate::structs::Vertex) -> () {
        unsafe { cbg_VertexArray_SetAt(self.self_ptr, index, value.clone()) }
    }

    /// インスタンスを作成します。
    /// # Arguments
    /// * `size` - 要素数

    pub fn create(size: i32) -> Option<Rc<RefCell<VertexArray>>> {
        let ret = unsafe { cbg_VertexArray_Create(size) };
        {
            let ret = VertexArray::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// 格納されている要素の数を取得します。
    pub fn get_count(&mut self) -> i32 {
        let ret = unsafe { cbg_VertexArray_GetCount(self.self_ptr) };
        ret
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { cbg_VertexArray_Release(self.self_ptr) };
    }
}

/// 浮動小数点数の配列のクラスを表します。
#[derive(Debug)]
pub(crate) struct FloatArray {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for FloatArray {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl FloatArray {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(FloatArray { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static FLOATARRAY_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<FloatArray>>>> = RefCell::new(HashMap::new());
        }
        FLOATARRAY_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// データをクリアします。

    pub fn clear(&mut self) -> () {
        unsafe { cbg_FloatArray_Clear(self.self_ptr) }
    }

    /// サイズを変更します。
    /// # Arguments
    /// * `size` - 要素数

    pub fn resize(&mut self, size: i32) -> () {
        unsafe { cbg_FloatArray_Resize(self.self_ptr, size) }
    }

    pub fn get_data(&mut self) -> *mut RawPtr {
        let ret = unsafe { cbg_FloatArray_GetData(self.self_ptr) };
        ret
    }

    pub fn assign(&mut self, ptr: *mut RawPtr, size: i32) -> () {
        unsafe { cbg_FloatArray_Assign(self.self_ptr, ptr, size) }
    }

    /// データを指定したポインタにコピーします。
    /// # Arguments
    /// * `ptr` - ポインタ

    pub fn copy_to(&mut self, ptr: *mut RawPtr) -> () {
        unsafe { cbg_FloatArray_CopyTo(self.self_ptr, ptr) }
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス

    pub fn get_at(&mut self, index: i32) -> f32 {
        let ret = unsafe { cbg_FloatArray_GetAt(self.self_ptr, index) };
        ret
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス
    /// * `value` - 値

    pub fn set_at(&mut self, index: i32, value: f32) -> () {
        unsafe { cbg_FloatArray_SetAt(self.self_ptr, index, value) }
    }

    /// インスタンスを作成します。
    /// # Arguments
    /// * `size` - 要素数

    pub fn create(size: i32) -> Option<Rc<RefCell<FloatArray>>> {
        let ret = unsafe { cbg_FloatArray_Create(size) };
        {
            let ret = FloatArray::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// 格納されている要素の数を取得します。
    pub fn get_count(&mut self) -> i32 {
        let ret = unsafe { cbg_FloatArray_GetCount(self.self_ptr) };
        ret
    }
}

impl Drop for FloatArray {
    fn drop(&mut self) {
        unsafe { cbg_FloatArray_Release(self.self_ptr) };
    }
}

/// 2次元ベクトルの配列のクラスを表します。
#[derive(Debug)]
pub(crate) struct Vector2FArray {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for Vector2FArray {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Vector2FArray {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Vector2FArray { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static VECTOR2FARRAY_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Vector2FArray>>>> = RefCell::new(HashMap::new());
        }
        VECTOR2FARRAY_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// データをクリアします。

    pub fn clear(&mut self) -> () {
        unsafe { cbg_Vector2FArray_Clear(self.self_ptr) }
    }

    /// サイズを変更します。
    /// # Arguments
    /// * `size` - 要素数

    pub fn resize(&mut self, size: i32) -> () {
        unsafe { cbg_Vector2FArray_Resize(self.self_ptr, size) }
    }

    pub fn get_data(&mut self) -> *mut RawPtr {
        let ret = unsafe { cbg_Vector2FArray_GetData(self.self_ptr) };
        ret
    }

    pub fn assign(&mut self, ptr: *mut RawPtr, size: i32) -> () {
        unsafe { cbg_Vector2FArray_Assign(self.self_ptr, ptr, size) }
    }

    /// データを指定したポインタにコピーします。
    /// # Arguments
    /// * `ptr` - ポインタ

    pub fn copy_to(&mut self, ptr: *mut RawPtr) -> () {
        unsafe { cbg_Vector2FArray_CopyTo(self.self_ptr, ptr) }
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス

    pub fn get_at(&mut self, index: i32) -> crate::math::Vector2<f32> {
        let ret = unsafe { cbg_Vector2FArray_GetAt(self.self_ptr, index) };
        ret
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス
    /// * `value` - 値

    pub fn set_at(&mut self, index: i32, mut value: crate::math::Vector2<f32>) -> () {
        unsafe { cbg_Vector2FArray_SetAt(self.self_ptr, index, value.clone()) }
    }

    /// インスタンスを作成します。
    /// # Arguments
    /// * `size` - 要素数

    pub fn create(size: i32) -> Option<Rc<RefCell<Vector2FArray>>> {
        let ret = unsafe { cbg_Vector2FArray_Create(size) };
        {
            let ret = Vector2FArray::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// 格納されている要素の数を取得します。
    pub fn get_count(&mut self) -> i32 {
        let ret = unsafe { cbg_Vector2FArray_GetCount(self.self_ptr) };
        ret
    }
}

impl Drop for Vector2FArray {
    fn drop(&mut self) {
        unsafe { cbg_Vector2FArray_Release(self.self_ptr) };
    }
}

/// リソースのクラスを表します。
#[derive(Debug)]
pub(crate) struct Resources {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for Resources {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Resources {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Resources { self_ptr })
    }

    /// インスタンスを取得します。

    pub(crate) fn get_instance() -> Option<Resources> {
        let ret = unsafe { cbg_Resources_GetInstance() };
        Resources::cbg_create_raw(ret)
    }

    /// 指定した種類のリソースの個数を返します。
    /// # Arguments
    /// * `type_` - 個数を検索するリソースの種類

    pub fn get_resources_count(&mut self, type_: ResourceType) -> i32 {
        let ret = unsafe { cbg_Resources_GetResourcesCount(self.self_ptr, type_ as i32) };
        ret
    }

    /// 登録されたリソースをすべて削除します。

    pub fn clear(&mut self) -> () {
        unsafe { cbg_Resources_Clear(self.self_ptr) }
    }

    /// リソースの再読み込みを行います。

    pub fn reload(&mut self) -> () {
        unsafe { cbg_Resources_Reload(self.self_ptr) }
    }
}

impl Drop for Resources {
    fn drop(&mut self) {
        unsafe { cbg_Resources_Release(self.self_ptr) };
    }
}

/// カーソルを表します。
#[derive(Debug)]
pub struct Cursor {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for Cursor {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Cursor {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Cursor { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static CURSOR_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Cursor>>>> = RefCell::new(HashMap::new());
        }
        CURSOR_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// 指定したpng画像を読み込んだCursorのインスタンスを生成します。
    /// # Arguments
    /// * `path` - 読み込むpng画像のパス
    /// * `hot` - カーソルのクリック判定座標を指定します。カーソル画像はここが中心となります。

    pub fn create(path: &str, mut hot: crate::math::Vector2<i32>) -> Option<Rc<RefCell<Cursor>>> {
        let ret = unsafe { cbg_Cursor_Create(encode_string(&path).as_ptr(), hot.clone()) };
        {
            let ret = Cursor::try_get_from_cache(ret)?;
            Some(ret)
        }
    }
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe { cbg_Cursor_Release(self.self_ptr) };
    }
}

/// キーボードを表します。
#[derive(Debug)]
pub struct Keyboard {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for Keyboard {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Keyboard {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Keyboard { self_ptr })
    }

    /// キーの状態を取得します。
    /// # Arguments
    /// * `key` - キー

    pub fn get_key_state(&mut self, key: Keys) -> ButtonState {
        let ret = unsafe { cbg_Keyboard_GetKeyState(self.self_ptr, key as i32) };
        unsafe { std::mem::transmute(ret) }
    }

    /// インスタンスを取得します。

    pub(crate) fn __get_instance() -> Option<Keyboard> {
        let ret = unsafe { cbg_Keyboard_GetInstance() };
        Keyboard::cbg_create_raw(ret)
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        unsafe { cbg_Keyboard_Release(self.self_ptr) };
    }
}

/// マウスを表します。
#[derive(Debug)]
pub struct Mouse {
    self_ptr: *mut RawPtr,
    position: Option<crate::math::Vector2<f32>>,
    cursor_mode: Option<CursorMode>,
}

impl HasRawPtr for Mouse {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Mouse {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Mouse {
            self_ptr,
            position: None,
            cursor_mode: None,
        })
    }

    /// インスタンスを取得します。

    pub(crate) fn __get_instance() -> Option<Mouse> {
        let ret = unsafe { cbg_Mouse_GetInstance() };
        Mouse::cbg_create_raw(ret)
    }

    /// マウスボタンの状態を取得します。
    /// # Arguments
    /// * `button` - 状態を取得するマウスのボタン

    pub fn get_mouse_button_state(&mut self, button: MouseButtons) -> ButtonState {
        let ret = unsafe { cbg_Mouse_GetMouseButtonState(self.self_ptr, button as i32) };
        unsafe { std::mem::transmute(ret) }
    }

    /// カーソル画像をセットします。
    /// # Arguments
    /// * `cursor` - 状態を取得するマウスのボタン

    pub fn set_cursor_image(&mut self, cursor: &mut Cursor) -> () {
        unsafe { cbg_Mouse_SetCursorImage(self.self_ptr, cursor.self_ptr()) }
    }

    /// マウスカーソルの座標を取得または設定します。
    pub fn get_position(&mut self) -> crate::math::Vector2<f32> {
        if let Some(value) = &self.position {
            return value.clone();
        }
        let ret = unsafe { cbg_Mouse_GetPosition(self.self_ptr) };
        ret
    }

    /// マウスカーソルの座標を取得または設定します。
    pub fn set_position(&mut self, mut value: crate::math::Vector2<f32>) {
        unsafe { cbg_Mouse_SetPosition(self.self_ptr, value.clone()) }
        self.position = Some(value.clone());
    }

    /// カーソルのモードを取得または設定します。
    pub fn get_cursor_mode(&mut self) -> CursorMode {
        if let Some(value) = &self.cursor_mode {
            return value.clone();
        }
        let ret = unsafe { cbg_Mouse_GetCursorMode(self.self_ptr) };
        unsafe { std::mem::transmute(ret) }
    }

    /// カーソルのモードを取得または設定します。
    pub fn set_cursor_mode(&mut self, value: CursorMode) {
        unsafe { cbg_Mouse_SetCursorMode(self.self_ptr, value as i32) }
        self.cursor_mode = Some(value.clone());
    }

    /// マウスホイールの回転量を取得します。
    pub fn get_wheel(&mut self) -> f32 {
        let ret = unsafe { cbg_Mouse_GetWheel(self.self_ptr) };
        ret
    }
}

impl Drop for Mouse {
    fn drop(&mut self) {
        unsafe { cbg_Mouse_Release(self.self_ptr) };
    }
}

/// ジョイスティックを表すクラス
#[derive(Debug)]
pub struct Joystick {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for Joystick {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Joystick {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Joystick { self_ptr })
    }

    /// インスタンスを取得します。

    pub(crate) fn __get_instance() -> Option<Joystick> {
        let ret = unsafe { cbg_Joystick_GetInstance() };
        Joystick::cbg_create_raw(ret)
    }

    /// 指定したジョイスティックが親であるかどうかを取得します。
    /// # Arguments
    /// * `joystick_index` - ジョイスティックのインデックス

    pub fn is_present(&mut self, joystick_index: i32) -> bool {
        let ret = unsafe { cbg_Joystick_IsPresent(self.self_ptr, joystick_index) };
        ret
    }

    /// 接続の状態をリセットします。

    pub fn refresh_connected_state(&mut self) -> () {
        unsafe { cbg_Joystick_RefreshConnectedState(self.self_ptr) }
    }

    /// ボタンの状態をインデックスで取得します。
    /// # Arguments
    /// * `joystick_index` - 検索するジョイスティックのインデックス
    /// * `button_index` - 状態を検索するボタンのインデックス

    pub(crate) fn __get_button_state_by_index(
        &mut self,
        joystick_index: i32,
        button_index: i32,
    ) -> ButtonState {
        let ret = unsafe {
            cbg_Joystick_GetButtonStateByIndex(self.self_ptr, joystick_index, button_index)
        };
        unsafe { std::mem::transmute(ret) }
    }

    /// ボタンの状態を種類から取得します。
    /// # Arguments
    /// * `joystick_index` - 検索するジョイスティックのインデックス
    /// * `type_` - 状態を検索するボタンの種類

    pub(crate) fn __get_button_state_by_type(
        &mut self,
        joystick_index: i32,
        type_: JoystickButtonType,
    ) -> ButtonState {
        let ret = unsafe {
            cbg_Joystick_GetButtonStateByType(self.self_ptr, joystick_index, type_ as i32)
        };
        unsafe { std::mem::transmute(ret) }
    }

    /// 指定インデックスのジョイスティックの種類を取得します。
    /// # Arguments
    /// * `index` - 種類を取得するジョイスティックのインデックス

    pub fn get_joystick_type(&mut self, index: i32) -> JoystickType {
        let ret = unsafe { cbg_Joystick_GetJoystickType(self.self_ptr, index) };
        unsafe { std::mem::transmute(ret) }
    }

    /// 軸の状態をインデックスで取得します。
    /// # Arguments
    /// * `joystick_index` - 検索するジョイスティックのインデックス
    /// * `axis_index` - 状態を検索する軸のインデックス

    pub(crate) fn __get_axis_state_by_index(
        &mut self,
        joystick_index: i32,
        axis_index: i32,
    ) -> f32 {
        let ret =
            unsafe { cbg_Joystick_GetAxisStateByIndex(self.self_ptr, joystick_index, axis_index) };
        ret
    }

    /// 軸の状態を軸の種類で取得します。
    /// # Arguments
    /// * `joystick_index` - 検索するジョイスティックのインデックス
    /// * `type_` - 状態を検索する軸の種類

    pub(crate) fn __get_axis_state_by_type(
        &mut self,
        joystick_index: i32,
        type_: JoystickAxisType,
    ) -> f32 {
        let ret =
            unsafe { cbg_Joystick_GetAxisStateByType(self.self_ptr, joystick_index, type_ as i32) };
        ret
    }

    /// ジョイスティックの名前を取得します。
    /// # Arguments
    /// * `index` - 名前を検索するジョイスティックのインデックス

    pub fn get_joystick_name(&mut self, index: i32) -> String {
        let ret = unsafe { cbg_Joystick_GetJoystickName(self.self_ptr, index) };
        decode_string(ret)
    }

    /// 指定したジョイスティックコントローラーを振動させます
    /// # Arguments
    /// * `index` - ジョイスティックのインデックス
    /// * `frequency` - 周波数
    /// * `amplitude` - 振幅

    pub fn vibrate(&mut self, index: i32, frequency: f32, amplitude: f32) -> () {
        unsafe { cbg_Joystick_Vibrate(self.self_ptr, index, frequency, amplitude) }
    }
}

impl Drop for Joystick {
    fn drop(&mut self) {
        unsafe { cbg_Joystick_Release(self.self_ptr) };
    }
}

/// グラフィックの制御を行うクラス
#[derive(Debug)]
pub(crate) struct Graphics {
    self_ptr: *mut RawPtr,
    clear_color: Option<crate::structs::Color>,
}

impl HasRawPtr for Graphics {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Graphics {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Graphics {
            self_ptr,
            clear_color: None,
        })
    }

    /// インスタンスを取得します。

    pub(crate) fn get_instance() -> Option<Graphics> {
        let ret = unsafe { cbg_Graphics_GetInstance() };
        Graphics::cbg_create_raw(ret)
    }

    /// 描画を開始します。

    pub(crate) fn begin_frame(&mut self) -> bool {
        let ret = unsafe { cbg_Graphics_BeginFrame(self.self_ptr) };
        ret
    }

    /// 描画を終了します。

    pub(crate) fn end_frame(&mut self) -> bool {
        let ret = unsafe { cbg_Graphics_EndFrame(self.self_ptr) };
        ret
    }

    /// イベントを処理します。

    pub(crate) fn do_events(&mut self) -> bool {
        let ret = unsafe { cbg_Graphics_DoEvents(self.self_ptr) };
        ret
    }

    /// 組み込みのシェーダを取得します。
    pub(crate) fn get_builtin_shader(&mut self) -> Option<Rc<RefCell<BuiltinShader>>> {
        let ret = unsafe { cbg_Graphics_GetBuiltinShader(self.self_ptr) };
        {
            let ret = BuiltinShader::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// クリア色を取得または設定します。
    pub fn get_clear_color(&mut self) -> crate::structs::Color {
        if let Some(value) = &self.clear_color {
            return value.clone();
        }
        let ret = unsafe { cbg_Graphics_GetClearColor(self.self_ptr) };
        ret
    }

    /// クリア色を取得または設定します。
    pub fn set_clear_color(&mut self, mut value: crate::structs::Color) {
        unsafe { cbg_Graphics_SetClearColor(self.self_ptr, value.clone()) }
        self.clear_color = Some(value.clone());
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {
        unsafe { cbg_Graphics_Release(self.self_ptr) };
    }
}

/// テクスチャのベースクラス
#[derive(Debug)]
pub struct TextureBase {
    self_ptr: *mut RawPtr,
}

unsafe impl Send for TextureBase {}
unsafe impl Sync for TextureBase {}

impl HasRawPtr for TextureBase {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

pub trait AsTextureBase: std::fmt::Debug + HasRawPtr {
    /// png画像として保存します
    /// # Arguments
    /// * `path` - 保存先

    fn save(&mut self, path: &str) -> bool;
    /// テクスチャの大きさ(ピクセル)を取得します。
    fn get_size(&mut self) -> crate::math::Vector2<i32>;
}
impl AsTextureBase for TextureBase {
    /// png画像として保存します
    /// # Arguments
    /// * `path` - 保存先

    fn save(&mut self, path: &str) -> bool {
        let ret = unsafe { cbg_TextureBase_Save(self.self_ptr, encode_string(&path).as_ptr()) };
        ret
    }

    /// テクスチャの大きさ(ピクセル)を取得します。
    fn get_size(&mut self) -> crate::math::Vector2<i32> {
        let ret = unsafe { cbg_TextureBase_GetSize(self.self_ptr) };
        ret
    }
}
impl TextureBase {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Arc::new(Mutex::new(TextureBase { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        lazy_static! {
            static ref TEXTUREBASE_CACHE: RwLock<HashMap<RawPtrStorage, sync::Weak<Mutex<TextureBase>>>> =
                RwLock::new(HashMap::new());
        }
        let mut hash_map = TEXTUREBASE_CACHE.write().unwrap();
        let storage = RawPtrStorage(self_ptr);
        if let Some(x) = hash_map.get(&storage) {
            match x.upgrade() {
                Some(o) => {
                    return Some(o);
                }
                None => {
                    hash_map.remove(&storage);
                }
            }
        }
        let o = Self::cbg_create_raw(self_ptr)?;
        hash_map.insert(storage, Arc::downgrade(&o));
        Some(o)
    }
}

impl Drop for TextureBase {
    fn drop(&mut self) {
        unsafe { cbg_TextureBase_Release(self.self_ptr) };
    }
}

/// テクスチャのクラス
#[derive(Debug)]
pub struct Texture2D {
    self_ptr: *mut RawPtr,
}

unsafe impl Send for Texture2D {}
unsafe impl Sync for Texture2D {}

impl HasRawPtr for Texture2D {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl AsTextureBase for Texture2D {
    /// png画像として保存します
    /// # Arguments
    /// * `path` - 保存先

    fn save(&mut self, path: &str) -> bool {
        let ret = unsafe { cbg_TextureBase_Save(self.self_ptr, encode_string(&path).as_ptr()) };
        ret
    }

    /// テクスチャの大きさ(ピクセル)を取得します。
    fn get_size(&mut self) -> crate::math::Vector2<i32> {
        let ret = unsafe { cbg_TextureBase_GetSize(self.self_ptr) };
        ret
    }
}
impl Texture2D {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Arc::new(Mutex::new(Texture2D { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        lazy_static! {
            static ref TEXTURE2D_CACHE: RwLock<HashMap<RawPtrStorage, sync::Weak<Mutex<Texture2D>>>> =
                RwLock::new(HashMap::new());
        }
        let mut hash_map = TEXTURE2D_CACHE.write().unwrap();
        let storage = RawPtrStorage(self_ptr);
        if let Some(x) = hash_map.get(&storage) {
            match x.upgrade() {
                Some(o) => {
                    return Some(o);
                }
                None => {
                    hash_map.remove(&storage);
                }
            }
        }
        let o = Self::cbg_create_raw(self_ptr)?;
        hash_map.insert(storage, Arc::downgrade(&o));
        Some(o)
    }

    /// 指定したファイルからテクスチャを読み込みます。
    /// # Arguments
    /// * `path` - 読み込むファイルのパス

    pub(crate) fn __load(path: &str) -> Option<Arc<Mutex<Texture2D>>> {
        let ret = unsafe { cbg_Texture2D_Load(encode_string(&path).as_ptr()) };
        {
            let ret = Texture2D::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// 再読み込みを行います。

    pub fn reload(&mut self) -> bool {
        let ret = unsafe { cbg_Texture2D_Reload(self.self_ptr) };
        ret
    }

    /// 読み込んだファイルのパスを取得します。
    pub(crate) fn __get_path(&mut self) -> String {
        let ret = unsafe { cbg_Texture2D_GetPath(self.self_ptr) };
        decode_string(ret)
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe { cbg_Texture2D_Release(self.self_ptr) };
    }
}

/// ポストエフェクトやカメラにおける描画先のクラス
#[derive(Debug)]
pub struct RenderTexture {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for RenderTexture {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl AsTextureBase for RenderTexture {
    /// png画像として保存します
    /// # Arguments
    /// * `path` - 保存先

    fn save(&mut self, path: &str) -> bool {
        let ret = unsafe { cbg_TextureBase_Save(self.self_ptr, encode_string(&path).as_ptr()) };
        ret
    }

    /// テクスチャの大きさ(ピクセル)を取得します。
    fn get_size(&mut self) -> crate::math::Vector2<i32> {
        let ret = unsafe { cbg_TextureBase_GetSize(self.self_ptr) };
        ret
    }
}
impl RenderTexture {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(RenderTexture { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static RENDERTEXTURE_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<RenderTexture>>>> = RefCell::new(HashMap::new());
        }
        RENDERTEXTURE_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    pub fn create(mut size: crate::math::Vector2<i32>) -> Option<Rc<RefCell<RenderTexture>>> {
        let ret = unsafe { cbg_RenderTexture_Create(size.clone()) };
        {
            let ret = RenderTexture::try_get_from_cache(ret)?;
            Some(ret)
        }
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        unsafe { cbg_RenderTexture_Release(self.self_ptr) };
    }
}

/// マテリアル
#[derive(Debug)]
pub struct Material {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for Material {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Material {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Material { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static MATERIAL_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Material>>>> = RefCell::new(HashMap::new());
        }
        MATERIAL_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// 新しいインスタンスを生成する

    pub fn new() -> Option<Rc<RefCell<Material>>> {
        Self::cbg_create_raw(unsafe { cbg_Material_Constructor_0() })
    }

    /// 指定した名前を持つVector4<f32>のインスタンスを取得する
    /// # Arguments
    /// * `key` - 検索するVector4<f32>のインスタンスの名前

    pub fn get_vector4f(&mut self, key: &str) -> crate::math::Vector4<f32> {
        let ret = unsafe { cbg_Material_GetVector4F(self.self_ptr, encode_string(&key).as_ptr()) };
        ret
    }

    /// 指定した名前を持つVector4<f32>の値を設定する
    /// # Arguments
    /// * `key` - 検索するVector4<f32>のインスタンスの名前
    /// * `value` - 設定するVector4<f32>のインスタンスの値

    pub fn set_vector4f(&mut self, key: &str, mut value: crate::math::Vector4<f32>) -> () {
        unsafe {
            cbg_Material_SetVector4F(self.self_ptr, encode_string(&key).as_ptr(), value.clone())
        }
    }

    /// 指定した名前を持つMatrix44<f32>のインスタンスを取得する
    /// # Arguments
    /// * `key` - 検索するMatrix44<f32>のインスタンスの名前

    pub fn get_matrix44f(&mut self, key: &str) -> crate::math::Matrix44<f32> {
        let ret = unsafe { cbg_Material_GetMatrix44F(self.self_ptr, encode_string(&key).as_ptr()) };
        ret
    }

    /// 指定した名前を持つMatrix44<f32>の値を設定する
    /// # Arguments
    /// * `key` - 検索するMatrix44<f32>のインスタンスの名前
    /// * `value` - 設定するMatrix44<f32>のインスタンスの値

    pub fn set_matrix44f(&mut self, key: &str, mut value: crate::math::Matrix44<f32>) -> () {
        unsafe {
            cbg_Material_SetMatrix44F(self.self_ptr, encode_string(&key).as_ptr(), value.clone())
        }
    }

    /// 指定した名前を持つTextureBaseのインスタンスを取得する
    /// # Arguments
    /// * `key` - 検索するTextureBaseのインスタンスの名前

    pub fn get_texture(&mut self, key: &str) -> Option<Arc<Mutex<TextureBase>>> {
        let ret = unsafe { cbg_Material_GetTexture(self.self_ptr, encode_string(&key).as_ptr()) };
        {
            let ret = TextureBase::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// 指定した名前を持つTextureBaseの値を設定する
    /// # Arguments
    /// * `key` - 検索するTextureBaseのインスタンスの名前
    /// * `value` - 設定するTextureBaseのインスタンスの値

    pub fn set_texture<T0: AsTextureBase>(&mut self, key: &str, value: &mut T0) -> () {
        unsafe {
            cbg_Material_SetTexture(
                self.self_ptr,
                encode_string(&key).as_ptr(),
                value.self_ptr(),
            )
        }
    }

    /// 指定した種類のシェーダを取得する
    /// # Arguments
    /// * `shader_stage` - 検索するシェーダのタイプ

    pub fn get_shader(&mut self, shader_stage: ShaderStageType) -> Option<Rc<RefCell<Shader>>> {
        let ret = unsafe { cbg_Material_GetShader(self.self_ptr, shader_stage as i32) };
        {
            let ret = Shader::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// シェーダを設定する
    /// # Arguments
    /// * `shader` - 設定するシェーダ

    pub fn set_shader(&mut self, shader: &mut Shader) -> () {
        unsafe { cbg_Material_SetShader(self.self_ptr, shader.self_ptr()) }
    }
}

impl Drop for Material {
    fn drop(&mut self) {
        unsafe { cbg_Material_Release(self.self_ptr) };
    }
}

/// レンダラのクラス
#[derive(Debug)]
pub(crate) struct Renderer {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for Renderer {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Renderer {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Renderer { self_ptr })
    }

    /// インスタンスを取得します。

    pub(crate) fn get_instance() -> Option<Renderer> {
        let ret = unsafe { cbg_Renderer_GetInstance() };
        Renderer::cbg_create_raw(ret)
    }

    /// スプライトを描画します。
    /// # Arguments
    /// * `sprite` - 描画するRenderedSpriteのインスタンス

    pub(crate) fn draw_sprite(&mut self, sprite: &mut RenderedSprite) -> () {
        unsafe { cbg_Renderer_DrawSprite(self.self_ptr, sprite.self_ptr()) }
    }

    /// テキストを描画します。
    /// # Arguments
    /// * `text` - 描画するRenderedTextのインスタンス

    pub(crate) fn draw_text(&mut self, text: &mut RenderedText) -> () {
        unsafe { cbg_Renderer_DrawText(self.self_ptr, text.self_ptr()) }
    }

    /// ポリゴンを描画します。
    /// # Arguments
    /// * `polygon` - 描画するRenderedPolygonのインスタンス

    pub(crate) fn draw_polygon(&mut self, polygon: &mut RenderedPolygon) -> () {
        unsafe { cbg_Renderer_DrawPolygon(self.self_ptr, polygon.self_ptr()) }
    }

    /// コマンドリストを描画します。

    pub fn render(&mut self) -> () {
        unsafe { cbg_Renderer_Render(self.self_ptr) }
    }

    /// 使用するカメラを設定します。
    /// # Arguments
    /// * `command_list` - 描画するカメラ

    pub(crate) fn set_camera(&mut self, command_list: &mut RenderedCamera) -> () {
        unsafe { cbg_Renderer_SetCamera(self.self_ptr, command_list.self_ptr()) }
    }

    /// 使用するカメラの設定をリセットします。

    pub fn reset_camera(&mut self) -> () {
        unsafe { cbg_Renderer_ResetCamera(self.self_ptr) }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe { cbg_Renderer_Release(self.self_ptr) };
    }
}

/// コマンドリストのクラス
#[derive(Debug)]
pub struct CommandList {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for CommandList {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl CommandList {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(CommandList { self_ptr })
    }

    /// ？

    pub fn set_render_target_with_screen(&mut self) -> () {
        unsafe { cbg_CommandList_SetRenderTargetWithScreen(self.self_ptr) }
    }

    pub fn get_screen_texture(&mut self) -> Option<Rc<RefCell<RenderTexture>>> {
        let ret = unsafe { cbg_CommandList_GetScreenTexture(self.self_ptr) };
        {
            let ret = RenderTexture::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    pub fn set_render_target(
        &mut self,
        target: &mut RenderTexture,
        mut render_pass_parameter: crate::structs::RenderPassParameter,
    ) -> () {
        unsafe {
            cbg_CommandList_SetRenderTarget(
                self.self_ptr,
                target.self_ptr(),
                render_pass_parameter.clone(),
            )
        }
    }

    pub fn render_to_render_target(&mut self, material: &mut Material) -> () {
        unsafe { cbg_CommandList_RenderToRenderTarget(self.self_ptr, material.self_ptr()) }
    }
}

impl Drop for CommandList {
    fn drop(&mut self) {
        unsafe { cbg_CommandList_Release(self.self_ptr) };
    }
}

/// 描画されるオブジェクトの基本クラスを表します
#[derive(Debug)]
pub(crate) struct Rendered {
    self_ptr: *mut RawPtr,
    transform: Option<crate::math::Matrix44<f32>>,
}

impl HasRawPtr for Rendered {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

pub trait AsRendered: std::fmt::Debug + HasRawPtr {
    /// 変換行列を取得または設定します。
    fn get_transform(&mut self) -> crate::math::Matrix44<f32>;
    /// 変換行列を取得または設定します。
    fn set_transform(&mut self, value: crate::math::Matrix44<f32>);
    /// BaseObjectのIdを取得します
    fn get_id(&mut self) -> i32;
}
impl AsRendered for Rendered {
    /// 変換行列を取得または設定します。
    fn get_transform(&mut self) -> crate::math::Matrix44<f32> {
        if let Some(value) = &self.transform {
            return value.clone();
        }
        let ret = unsafe { cbg_Rendered_GetTransform(self.self_ptr) };
        ret
    }

    /// 変換行列を取得または設定します。
    fn set_transform(&mut self, mut value: crate::math::Matrix44<f32>) {
        unsafe { cbg_Rendered_SetTransform(self.self_ptr, value.clone()) }
        self.transform = Some(value.clone());
    }

    /// BaseObjectのIdを取得します
    fn get_id(&mut self) -> i32 {
        let ret = unsafe { cbg_Rendered_GetId(self.self_ptr) };
        ret
    }
}
impl Rendered {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rendered {
            self_ptr,
            transform: None,
        })
    }

    /// 変換行列を取得または設定します。
    pub fn set_transform(&mut self, mut value: crate::math::Matrix44<f32>) {
        unsafe { cbg_Rendered_SetTransform(self.self_ptr, value.clone()) }
        self.transform = Some(value.clone());
    }
}

impl Drop for Rendered {
    fn drop(&mut self) {
        unsafe { cbg_Rendered_Release(self.self_ptr) };
    }
}

/// スプライトのクラス
#[derive(Debug)]
pub(crate) struct RenderedSprite {
    self_ptr: *mut RawPtr,
    texture: Option<Arc<Mutex<dyn AsTextureBase>>>,
    src: Option<crate::structs::Rect<f32>>,
    material: Option<Rc<RefCell<Material>>>,
    transform: Option<crate::math::Matrix44<f32>>,
}

impl HasRawPtr for RenderedSprite {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl AsRendered for RenderedSprite {
    /// 変換行列を取得または設定します。
    fn get_transform(&mut self) -> crate::math::Matrix44<f32> {
        if let Some(value) = &self.transform {
            return value.clone();
        }
        let ret = unsafe { cbg_Rendered_GetTransform(self.self_ptr) };
        ret
    }

    /// 変換行列を取得または設定します。
    fn set_transform(&mut self, mut value: crate::math::Matrix44<f32>) {
        unsafe { cbg_Rendered_SetTransform(self.self_ptr, value.clone()) }
        self.transform = Some(value.clone());
    }

    /// BaseObjectのIdを取得します
    fn get_id(&mut self) -> i32 {
        let ret = unsafe { cbg_Rendered_GetId(self.self_ptr) };
        ret
    }
}
impl RenderedSprite {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(RenderedSprite {
            self_ptr,
            texture: None,
            src: None,
            material: None,
            transform: None,
        })
    }

    /// スプライトを作成します。

    pub fn create() -> Option<RenderedSprite> {
        let ret = unsafe { cbg_RenderedSprite_Create() };
        RenderedSprite::cbg_create_raw(ret)
    }

    /// テクスチャを取得または設定します。
    pub fn get_texture(&mut self) -> Option<Arc<Mutex<dyn AsTextureBase>>> {
        if let Some(value) = &self.texture {
            return Some(value.clone());
        }
        let ret = unsafe { cbg_RenderedSprite_GetTexture(self.self_ptr) };
        {
            let ret = TextureBase::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// テクスチャを取得または設定します。
    pub fn set_texture<T: AsTextureBase + 'static>(&mut self, value: Arc<Mutex<T>>) {
        unsafe {
            cbg_RenderedSprite_SetTexture(
                self.self_ptr,
                value
                    .lock()
                    .expect("Failed to get lock of TextureBase")
                    .self_ptr(),
            )
        }
        self.texture = Some(value.clone());
    }

    /// 描画範囲を取得または設定します。
    pub fn get_src(&mut self) -> crate::structs::Rect<f32> {
        if let Some(value) = &self.src {
            return value.clone();
        }
        let ret = unsafe { cbg_RenderedSprite_GetSrc(self.self_ptr) };
        ret
    }

    /// 描画範囲を取得または設定します。
    pub fn set_src(&mut self, mut value: crate::structs::Rect<f32>) {
        unsafe { cbg_RenderedSprite_SetSrc(self.self_ptr, value.clone()) }
        self.src = Some(value.clone());
    }

    /// マテリアルを取得または設定します。
    pub fn get_material(&mut self) -> Option<Rc<RefCell<Material>>> {
        if let Some(value) = &self.material {
            return Some(value.clone());
        }
        let ret = unsafe { cbg_RenderedSprite_GetMaterial(self.self_ptr) };
        {
            let ret = Material::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// マテリアルを取得または設定します。
    pub fn set_material(&mut self, value: Rc<RefCell<Material>>) {
        unsafe { cbg_RenderedSprite_SetMaterial(self.self_ptr, value.borrow_mut().self_ptr()) }
        self.material = Some(value.clone());
    }
}

impl Drop for RenderedSprite {
    fn drop(&mut self) {
        unsafe { cbg_RenderedSprite_Release(self.self_ptr) };
    }
}

/// テキストのクラス
#[derive(Debug)]
pub(crate) struct RenderedText {
    self_ptr: *mut RawPtr,
    material: Option<Rc<RefCell<Material>>>,
    text: Option<String>,
    font: Option<Arc<Mutex<Font>>>,
    weight: Option<f32>,
    color: Option<crate::structs::Color>,
    transform: Option<crate::math::Matrix44<f32>>,
}

impl HasRawPtr for RenderedText {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl AsRendered for RenderedText {
    /// 変換行列を取得または設定します。
    fn get_transform(&mut self) -> crate::math::Matrix44<f32> {
        if let Some(value) = &self.transform {
            return value.clone();
        }
        let ret = unsafe { cbg_Rendered_GetTransform(self.self_ptr) };
        ret
    }

    /// 変換行列を取得または設定します。
    fn set_transform(&mut self, mut value: crate::math::Matrix44<f32>) {
        unsafe { cbg_Rendered_SetTransform(self.self_ptr, value.clone()) }
        self.transform = Some(value.clone());
    }

    /// BaseObjectのIdを取得します
    fn get_id(&mut self) -> i32 {
        let ret = unsafe { cbg_Rendered_GetId(self.self_ptr) };
        ret
    }
}
impl RenderedText {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(RenderedText {
            self_ptr,
            material: None,
            text: None,
            font: None,
            weight: None,
            color: None,
            transform: None,
        })
    }

    /// テキストを作成します。

    pub fn create() -> Option<RenderedText> {
        let ret = unsafe { cbg_RenderedText_Create() };
        RenderedText::cbg_create_raw(ret)
    }

    /// マテリアルを取得または設定します。
    pub fn get_material(&mut self) -> Option<Rc<RefCell<Material>>> {
        if let Some(value) = &self.material {
            return Some(value.clone());
        }
        let ret = unsafe { cbg_RenderedText_GetMaterial(self.self_ptr) };
        {
            let ret = Material::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// マテリアルを取得または設定します。
    pub fn set_material(&mut self, value: Rc<RefCell<Material>>) {
        unsafe { cbg_RenderedText_SetMaterial(self.self_ptr, value.borrow_mut().self_ptr()) }
        self.material = Some(value.clone());
    }

    /// テキストを取得または設定します。
    pub fn get_text(&mut self) -> String {
        if let Some(value) = &self.text {
            return value.clone();
        }
        let ret = unsafe { cbg_RenderedText_GetText(self.self_ptr) };
        decode_string(ret)
    }

    /// テキストを取得または設定します。
    pub fn set_text(&mut self, value: String) {
        unsafe { cbg_RenderedText_SetText(self.self_ptr, encode_string(&value).as_ptr()) }
        self.text = Some(value.clone());
    }

    /// フォントを取得または設定します。
    pub fn get_font(&mut self) -> Option<Arc<Mutex<Font>>> {
        if let Some(value) = &self.font {
            return Some(value.clone());
        }
        let ret = unsafe { cbg_RenderedText_GetFont(self.self_ptr) };
        {
            let ret = Font::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// フォントを取得または設定します。
    pub fn set_font(&mut self, value: Arc<Mutex<Font>>) {
        unsafe {
            cbg_RenderedText_SetFont(
                self.self_ptr,
                value.lock().expect("Failed to get lock of Font").self_ptr(),
            )
        }
        self.font = Some(value.clone());
    }

    /// 文字の太さを取得または設定します。(0 ~ 255)
    pub fn get_weight(&mut self) -> f32 {
        if let Some(value) = &self.weight {
            return value.clone();
        }
        let ret = unsafe { cbg_RenderedText_GetWeight(self.self_ptr) };
        ret
    }

    /// 文字の太さを取得または設定します。(0 ~ 255)
    pub fn set_weight(&mut self, value: f32) {
        unsafe { cbg_RenderedText_SetWeight(self.self_ptr, value) }
        self.weight = Some(value.clone());
    }

    /// 色を取得または設定します。
    pub fn get_color(&mut self) -> crate::structs::Color {
        if let Some(value) = &self.color {
            return value.clone();
        }
        let ret = unsafe { cbg_RenderedText_GetColor(self.self_ptr) };
        ret
    }

    /// 色を取得または設定します。
    pub fn set_color(&mut self, mut value: crate::structs::Color) {
        unsafe { cbg_RenderedText_SetColor(self.self_ptr, value.clone()) }
        self.color = Some(value.clone());
    }
}

impl Drop for RenderedText {
    fn drop(&mut self) {
        unsafe { cbg_RenderedText_Release(self.self_ptr) };
    }
}

/// ポリゴンのクラス
#[derive(Debug)]
pub(crate) struct RenderedPolygon {
    self_ptr: *mut RawPtr,
    vertexes: Option<Rc<RefCell<VertexArray>>>,
    texture: Option<Arc<Mutex<dyn AsTextureBase>>>,
    src: Option<crate::structs::Rect<f32>>,
    material: Option<Rc<RefCell<Material>>>,
    transform: Option<crate::math::Matrix44<f32>>,
}

impl HasRawPtr for RenderedPolygon {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl AsRendered for RenderedPolygon {
    /// 変換行列を取得または設定します。
    fn get_transform(&mut self) -> crate::math::Matrix44<f32> {
        if let Some(value) = &self.transform {
            return value.clone();
        }
        let ret = unsafe { cbg_Rendered_GetTransform(self.self_ptr) };
        ret
    }

    /// 変換行列を取得または設定します。
    fn set_transform(&mut self, mut value: crate::math::Matrix44<f32>) {
        unsafe { cbg_Rendered_SetTransform(self.self_ptr, value.clone()) }
        self.transform = Some(value.clone());
    }

    /// BaseObjectのIdを取得します
    fn get_id(&mut self) -> i32 {
        let ret = unsafe { cbg_Rendered_GetId(self.self_ptr) };
        ret
    }
}
impl RenderedPolygon {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(RenderedPolygon {
            self_ptr,
            vertexes: None,
            texture: None,
            src: None,
            material: None,
            transform: None,
        })
    }

    /// ポリゴンを作成します。

    pub fn create() -> Option<RenderedPolygon> {
        let ret = unsafe { cbg_RenderedPolygon_Create() };
        RenderedPolygon::cbg_create_raw(ret)
    }

    /// 頂点情報

    pub fn set_vertexes_by_vector2f(&mut self, vertexes: &mut Vector2FArray) -> () {
        unsafe { cbg_RenderedPolygon_SetVertexesByVector2F(self.self_ptr, vertexes.self_ptr()) }
    }

    /// 頂点情報を取得または設定します。
    pub fn get_vertexes(&mut self) -> Option<Rc<RefCell<VertexArray>>> {
        if let Some(value) = &self.vertexes {
            return Some(value.clone());
        }
        let ret = unsafe { cbg_RenderedPolygon_GetVertexes(self.self_ptr) };
        {
            let ret = VertexArray::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// 頂点情報を取得または設定します。
    pub fn set_vertexes(&mut self, value: Rc<RefCell<VertexArray>>) {
        unsafe { cbg_RenderedPolygon_SetVertexes(self.self_ptr, value.borrow_mut().self_ptr()) }
        self.vertexes = Some(value.clone());
    }

    /// テクスチャを取得または設定します。
    pub fn get_texture(&mut self) -> Option<Arc<Mutex<dyn AsTextureBase>>> {
        if let Some(value) = &self.texture {
            return Some(value.clone());
        }
        let ret = unsafe { cbg_RenderedPolygon_GetTexture(self.self_ptr) };
        {
            let ret = TextureBase::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// テクスチャを取得または設定します。
    pub fn set_texture<T: AsTextureBase + 'static>(&mut self, value: Arc<Mutex<T>>) {
        unsafe {
            cbg_RenderedPolygon_SetTexture(
                self.self_ptr,
                value
                    .lock()
                    .expect("Failed to get lock of TextureBase")
                    .self_ptr(),
            )
        }
        self.texture = Some(value.clone());
    }

    /// 描画範囲を取得または設定します。
    pub fn get_src(&mut self) -> crate::structs::Rect<f32> {
        if let Some(value) = &self.src {
            return value.clone();
        }
        let ret = unsafe { cbg_RenderedPolygon_GetSrc(self.self_ptr) };
        ret
    }

    /// 描画範囲を取得または設定します。
    pub fn set_src(&mut self, mut value: crate::structs::Rect<f32>) {
        unsafe { cbg_RenderedPolygon_SetSrc(self.self_ptr, value.clone()) }
        self.src = Some(value.clone());
    }

    /// マテリアルを取得または設定します。
    pub fn get_material(&mut self) -> Option<Rc<RefCell<Material>>> {
        if let Some(value) = &self.material {
            return Some(value.clone());
        }
        let ret = unsafe { cbg_RenderedPolygon_GetMaterial(self.self_ptr) };
        {
            let ret = Material::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// マテリアルを取得または設定します。
    pub fn set_material(&mut self, value: Rc<RefCell<Material>>) {
        unsafe { cbg_RenderedPolygon_SetMaterial(self.self_ptr, value.borrow_mut().self_ptr()) }
        self.material = Some(value.clone());
    }
}

impl Drop for RenderedPolygon {
    fn drop(&mut self) {
        unsafe { cbg_RenderedPolygon_Release(self.self_ptr) };
    }
}

/// カメラのクラス
#[derive(Debug)]
pub(crate) struct RenderedCamera {
    self_ptr: *mut RawPtr,
    center_offset: Option<crate::math::Vector2<f32>>,
    target_texture: Option<Rc<RefCell<RenderTexture>>>,
    transform: Option<crate::math::Matrix44<f32>>,
}

impl HasRawPtr for RenderedCamera {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl AsRendered for RenderedCamera {
    /// 変換行列を取得または設定します。
    fn get_transform(&mut self) -> crate::math::Matrix44<f32> {
        if let Some(value) = &self.transform {
            return value.clone();
        }
        let ret = unsafe { cbg_Rendered_GetTransform(self.self_ptr) };
        ret
    }

    /// 変換行列を取得または設定します。
    fn set_transform(&mut self, mut value: crate::math::Matrix44<f32>) {
        unsafe { cbg_Rendered_SetTransform(self.self_ptr, value.clone()) }
        self.transform = Some(value.clone());
    }

    /// BaseObjectのIdを取得します
    fn get_id(&mut self) -> i32 {
        let ret = unsafe { cbg_Rendered_GetId(self.self_ptr) };
        ret
    }
}
impl RenderedCamera {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(RenderedCamera {
            self_ptr,
            center_offset: None,
            target_texture: None,
            transform: None,
        })
    }

    /// RenderedCameraを作成します。

    pub fn create() -> Option<RenderedCamera> {
        let ret = unsafe { cbg_RenderedCamera_Create() };
        RenderedCamera::cbg_create_raw(ret)
    }

    /// CenterOffsetを取得または設定します。
    pub fn get_center_offset(&mut self) -> crate::math::Vector2<f32> {
        if let Some(value) = &self.center_offset {
            return value.clone();
        }
        let ret = unsafe { cbg_RenderedCamera_GetCenterOffset(self.self_ptr) };
        ret
    }

    /// CenterOffsetを取得または設定します。
    pub fn set_center_offset(&mut self, mut value: crate::math::Vector2<f32>) {
        unsafe { cbg_RenderedCamera_SetCenterOffset(self.self_ptr, value.clone()) }
        self.center_offset = Some(value.clone());
    }

    /// TargetTextureを取得または設定します。
    pub fn get_target_texture(&mut self) -> Option<Rc<RefCell<RenderTexture>>> {
        if let Some(value) = &self.target_texture {
            return Some(value.clone());
        }
        let ret = unsafe { cbg_RenderedCamera_GetTargetTexture(self.self_ptr) };
        {
            let ret = RenderTexture::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// TargetTextureを取得または設定します。
    pub fn set_target_texture(&mut self, value: Rc<RefCell<RenderTexture>>) {
        unsafe { cbg_RenderedCamera_SetTargetTexture(self.self_ptr, value.borrow_mut().self_ptr()) }
        self.target_texture = Some(value.clone());
    }
}

impl Drop for RenderedCamera {
    fn drop(&mut self) {
        unsafe { cbg_RenderedCamera_Release(self.self_ptr) };
    }
}

/// 組み込みシェーダの取得を行うクラス
#[derive(Debug)]
pub struct BuiltinShader {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for BuiltinShader {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl BuiltinShader {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(BuiltinShader { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static BUILTINSHADER_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<BuiltinShader>>>> = RefCell::new(HashMap::new());
        }
        BUILTINSHADER_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// シェーダを取得します。
    /// # Arguments
    /// * `type_` - シェーダの種類

    pub(crate) fn __create(&mut self, type_: BuiltinShaderType) -> Option<Rc<RefCell<Shader>>> {
        let ret = unsafe { cbg_BuiltinShader_Create(self.self_ptr, type_ as i32) };
        {
            let ret = Shader::try_get_from_cache(ret)?;
            Some(ret)
        }
    }
}

impl Drop for BuiltinShader {
    fn drop(&mut self) {
        unsafe { cbg_BuiltinShader_Release(self.self_ptr) };
    }
}

/// シェーダ
#[derive(Debug)]
pub struct Shader {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for Shader {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Shader {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Shader { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static SHADER_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Shader>>>> = RefCell::new(HashMap::new());
        }
        SHADER_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// コードをコンパイルしてシェーダを生成する
    /// # Arguments
    /// * `name` - シェーダの名前
    /// * `code` - コンパイルするコード
    /// * `shader_stage` -

    pub fn create(
        name: &str,
        code: &str,
        shader_stage: ShaderStageType,
    ) -> Option<Rc<RefCell<Shader>>> {
        let ret = unsafe {
            cbg_Shader_Create(
                encode_string(&name).as_ptr(),
                encode_string(&code).as_ptr(),
                shader_stage as i32,
            )
        };
        {
            let ret = Shader::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    pub fn get_stage_type(&mut self) -> ShaderStageType {
        let ret = unsafe { cbg_Shader_GetStageType(self.self_ptr) };
        unsafe { std::mem::transmute(ret) }
    }

    /// インスタンス生成に使用したコードを取得します
    pub fn get_code(&mut self) -> String {
        let ret = unsafe { cbg_Shader_GetCode(self.self_ptr) };
        decode_string(ret)
    }

    /// 名前を取得します
    pub fn get_name(&mut self) -> String {
        let ret = unsafe { cbg_Shader_GetName(self.self_ptr) };
        decode_string(ret)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { cbg_Shader_Release(self.self_ptr) };
    }
}

/// 文字情報
#[derive(Debug)]
pub struct Glyph {
    self_ptr: *mut RawPtr,
}

unsafe impl Send for Glyph {}
unsafe impl Sync for Glyph {}

impl HasRawPtr for Glyph {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Glyph {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Arc::new(Mutex::new(Glyph { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        lazy_static! {
            static ref GLYPH_CACHE: RwLock<HashMap<RawPtrStorage, sync::Weak<Mutex<Glyph>>>> =
                RwLock::new(HashMap::new());
        }
        let mut hash_map = GLYPH_CACHE.write().unwrap();
        let storage = RawPtrStorage(self_ptr);
        if let Some(x) = hash_map.get(&storage) {
            match x.upgrade() {
                Some(o) => {
                    return Some(o);
                }
                None => {
                    hash_map.remove(&storage);
                }
            }
        }
        let o = Self::cbg_create_raw(self_ptr)?;
        hash_map.insert(storage, Arc::downgrade(&o));
        Some(o)
    }

    /// 文字テクスチャのサイズを取得する
    pub fn get_texture_size(&mut self) -> crate::math::Vector2<i32> {
        let ret = unsafe { cbg_Glyph_GetTextureSize(self.self_ptr) };
        ret
    }

    /// 文字テクスチャのインデックスを取得する
    pub fn get_texture_index(&mut self) -> i32 {
        let ret = unsafe { cbg_Glyph_GetTextureIndex(self.self_ptr) };
        ret
    }

    /// 文字の座標を取得する
    pub fn get_position(&mut self) -> crate::math::Vector2<i32> {
        let ret = unsafe { cbg_Glyph_GetPosition(self.self_ptr) };
        ret
    }

    /// 文字のサイズを取得する
    pub fn get_size(&mut self) -> crate::math::Vector2<i32> {
        let ret = unsafe { cbg_Glyph_GetSize(self.self_ptr) };
        ret
    }

    /// 文字のオフセットを取得する
    pub fn get_offset(&mut self) -> crate::math::Vector2<i32> {
        let ret = unsafe { cbg_Glyph_GetOffset(self.self_ptr) };
        ret
    }

    /// 文字の幅を取得する
    pub fn get_glyph_width(&mut self) -> i32 {
        let ret = unsafe { cbg_Glyph_GetGlyphWidth(self.self_ptr) };
        ret
    }
}

impl Drop for Glyph {
    fn drop(&mut self) {
        unsafe { cbg_Glyph_Release(self.self_ptr) };
    }
}

/// フォント
#[derive(Debug)]
pub struct Font {
    self_ptr: *mut RawPtr,
}

unsafe impl Send for Font {}
unsafe impl Sync for Font {}

impl HasRawPtr for Font {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Font {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Arc::new(Mutex::new(Font { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        lazy_static! {
            static ref FONT_CACHE: RwLock<HashMap<RawPtrStorage, sync::Weak<Mutex<Font>>>> =
                RwLock::new(HashMap::new());
        }
        let mut hash_map = FONT_CACHE.write().unwrap();
        let storage = RawPtrStorage(self_ptr);
        if let Some(x) = hash_map.get(&storage) {
            match x.upgrade() {
                Some(o) => {
                    return Some(o);
                }
                None => {
                    hash_map.remove(&storage);
                }
            }
        }
        let o = Self::cbg_create_raw(self_ptr)?;
        hash_map.insert(storage, Arc::downgrade(&o));
        Some(o)
    }

    /// 動的にフォントを生成します
    /// # Arguments
    /// * `path` - 読み込むフォントのパス
    /// * `size` - フォントのサイズ

    pub(crate) fn __load_dynamic_font(path: &str, size: i32) -> Option<Arc<Mutex<Font>>> {
        let ret = unsafe { cbg_Font_LoadDynamicFont(encode_string(&path).as_ptr(), size) };
        {
            let ret = Font::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// 静的にフォントを生成します
    /// # Arguments
    /// * `path` - 読み込むフォントのパス

    pub(crate) fn __load_static_font(path: &str) -> Option<Arc<Mutex<Font>>> {
        let ret = unsafe { cbg_Font_LoadStaticFont(encode_string(&path).as_ptr()) };
        {
            let ret = Font::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// a2fフォントを生成します
    /// # Arguments
    /// * `dynamic_font_path` - 読み込むtruetypeフォントのパス
    /// * `static_font_path` - 生成するa2fフォントのパス
    /// * `size` - フォントのサイズ
    /// * `characters` - フォント化させる文字列

    pub fn generate_font_file(
        dynamic_font_path: &str,
        static_font_path: &str,
        size: i32,
        characters: &str,
    ) -> bool {
        let ret = unsafe {
            cbg_Font_GenerateFontFile(
                encode_string(&dynamic_font_path).as_ptr(),
                encode_string(&static_font_path).as_ptr(),
                size,
                encode_string(&characters).as_ptr(),
            )
        };
        ret
    }

    /// 文字情報を取得する
    /// # Arguments
    /// * `character` - 文字

    pub fn get_glyph(&mut self, character: i32) -> Option<Arc<Mutex<Glyph>>> {
        let ret = unsafe { cbg_Font_GetGlyph(self.self_ptr, character) };
        {
            let ret = Glyph::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// 文字列テクスチャを得る
    /// # Arguments
    /// * `index` - インデックス

    pub fn get_font_texture(&mut self, index: i32) -> Option<Arc<Mutex<Texture2D>>> {
        let ret = unsafe { cbg_Font_GetFontTexture(self.self_ptr, index) };
        {
            let ret = Texture2D::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// カーニングを得る
    /// # Arguments
    /// * `c1` - 文字1
    /// * `c2` - 文字2

    pub fn get_kerning(&mut self, c1: i32, c2: i32) -> i32 {
        let ret = unsafe { cbg_Font_GetKerning(self.self_ptr, c1, c2) };
        ret
    }

    /// テキストを描画したときのサイズを取得します
    /// # Arguments
    /// * `text` - テキスト
    /// * `direction` - 文字列の方向
    /// * `is_enable_kerning` - カーニングの有無

    pub fn calc_texture_size(
        &mut self,
        text: &str,
        direction: WritingDirection,
        is_enable_kerning: bool,
    ) -> crate::math::Vector2<i32> {
        let ret = unsafe {
            cbg_Font_CalcTextureSize(
                self.self_ptr,
                encode_string(&text).as_ptr(),
                direction as i32,
                is_enable_kerning,
            )
        };
        ret
    }

    /// テクスチャ追加対応フォントを生成します
    /// # Arguments
    /// * `base_font` - ベースとなるフォント

    pub fn create_image_font(base_font: &mut Font) -> Option<Arc<Mutex<Font>>> {
        let ret = unsafe { cbg_Font_CreateImageFont(base_font.self_ptr()) };
        {
            let ret = Font::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// テクスチャ文字を追加する
    /// # Arguments
    /// * `character` - 文字
    /// * `texture` - テクスチャ

    pub(crate) fn __add_image_glyph(&mut self, character: i32, texture: &mut Texture2D) -> () {
        unsafe { cbg_Font_AddImageGlyph(self.self_ptr, character, texture.self_ptr()) }
    }

    /// テクスチャ文字を取得する
    /// # Arguments
    /// * `character` - 文字

    pub fn get_image_glyph(&mut self, character: i32) -> Option<Arc<Mutex<Texture2D>>> {
        let ret = unsafe { cbg_Font_GetImageGlyph(self.self_ptr, character) };
        {
            let ret = Texture2D::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// フォントのサイズを取得する
    pub fn get_size(&mut self) -> i32 {
        let ret = unsafe { cbg_Font_GetSize(self.self_ptr) };
        ret
    }

    /// フォントのベースラインからトップラインまでの距離を取得する
    pub fn get_ascent(&mut self) -> i32 {
        let ret = unsafe { cbg_Font_GetAscent(self.self_ptr) };
        ret
    }

    /// フォントのベースラインからボトムラインまでの距離を取得する
    pub fn get_descent(&mut self) -> i32 {
        let ret = unsafe { cbg_Font_GetDescent(self.self_ptr) };
        ret
    }

    /// フォントの行間の距離を取得する
    pub fn get_line_gap(&mut self) -> i32 {
        let ret = unsafe { cbg_Font_GetLineGap(self.self_ptr) };
        ret
    }

    /// StaticFontか否か
    pub fn get_is_static_font(&mut self) -> bool {
        let ret = unsafe { cbg_Font_GetIsStaticFont(self.self_ptr) };
        ret
    }

    /// 読み込んだファイルのパスを取得します。
    pub fn get_path(&mut self) -> String {
        let ret = unsafe { cbg_Font_GetPath(self.self_ptr) };
        decode_string(ret)
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { cbg_Font_Release(self.self_ptr) };
    }
}

/// カリングのクラス
#[derive(Debug)]
pub(crate) struct CullingSystem {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for CullingSystem {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl CullingSystem {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(CullingSystem { self_ptr })
    }

    /// インスタンスを取得します。

    pub(crate) fn get_instance() -> Option<CullingSystem> {
        let ret = unsafe { cbg_CullingSystem_GetInstance() };
        CullingSystem::cbg_create_raw(ret)
    }

    /// RenderedのAABBを更新します

    pub(crate) fn update_aabb(&mut self) -> () {
        unsafe { cbg_CullingSystem_UpdateAABB(self.self_ptr) }
    }

    /// 描画されているRenderedの個数を取得する
    pub fn get_drawing_rendered_count(&mut self) -> i32 {
        let ret = unsafe { cbg_CullingSystem_GetDrawingRenderedCount(self.self_ptr) };
        ret
    }

    /// 描画されているRenderedのIdの配列を取得する
    pub fn get_drawing_rendered_ids(&mut self) -> Option<Rc<RefCell<Int32Array>>> {
        let ret = unsafe { cbg_CullingSystem_GetDrawingRenderedIds(self.self_ptr) };
        {
            let ret = Int32Array::try_get_from_cache(ret)?;
            Some(ret)
        }
    }
}

impl Drop for CullingSystem {
    fn drop(&mut self) {
        unsafe { cbg_CullingSystem_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Tool {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for Tool {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Tool {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Tool { self_ptr })
    }

    pub(crate) fn __get_instance() -> Option<Tool> {
        let ret = unsafe { cbg_Tool_GetInstance() };
        Tool::cbg_create_raw(ret)
    }

    /// `End()` を呼び出してください。

    pub fn begin(&mut self, name: &str, flags: ToolWindow) -> bool {
        let ret = unsafe {
            cbg_Tool_Begin(
                self.self_ptr,
                encode_string(&name).as_ptr(),
                flags.bits as i32,
            )
        };
        ret
    }

    ///

    pub fn end(&mut self) -> () {
        unsafe { cbg_Tool_End(self.self_ptr) }
    }

    ///

    pub(crate) fn __new_frame(&mut self) -> () {
        unsafe { cbg_Tool_NewFrame(self.self_ptr) }
    }

    ///

    pub(crate) fn __render(&mut self) -> () {
        unsafe { cbg_Tool_Render(self.self_ptr) }
    }

    ///

    pub fn dummy(&mut self, mut size: crate::math::Vector2<f32>) -> () {
        unsafe { cbg_Tool_Dummy(self.self_ptr, size.clone()) }
    }

    ///

    pub fn text(&mut self, text: &str) -> () {
        unsafe { cbg_Tool_Text(self.self_ptr, encode_string(&text).as_ptr()) }
    }

    ///

    pub fn text_unformatted(&mut self, text: &str) -> () {
        unsafe { cbg_Tool_TextUnformatted(self.self_ptr, encode_string(&text).as_ptr()) }
    }

    ///

    pub fn text_wrapped(&mut self, text: &str) -> () {
        unsafe { cbg_Tool_TextWrapped(self.self_ptr, encode_string(&text).as_ptr()) }
    }

    ///

    pub fn text_colored(&mut self, mut color: crate::math::Vector4<f32>, text: &str) -> () {
        unsafe { cbg_Tool_TextColored(self.self_ptr, color.clone(), encode_string(&text).as_ptr()) }
    }

    ///

    pub fn text_disabled(&mut self, text: &str) -> () {
        unsafe { cbg_Tool_TextDisabled(self.self_ptr, encode_string(&text).as_ptr()) }
    }

    ///

    pub fn bullet_text(&mut self, text: &str) -> () {
        unsafe { cbg_Tool_BulletText(self.self_ptr, encode_string(&text).as_ptr()) }
    }

    ///

    pub fn label_text(&mut self, label: &str, text: &str) -> () {
        unsafe {
            cbg_Tool_LabelText(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                encode_string(&text).as_ptr(),
            )
        }
    }

    ///

    pub fn collapsing_header(&mut self, label: &str, flags: ToolTreeNode) -> bool {
        let ret = unsafe {
            cbg_Tool_CollapsingHeader(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                flags.bits as i32,
            )
        };
        ret
    }

    ///

    pub fn tree_node(&mut self, label: &str) -> bool {
        let ret = unsafe { cbg_Tool_TreeNode(self.self_ptr, encode_string(&label).as_ptr()) };
        ret
    }

    ///

    pub fn tree_node_ex(&mut self, label: &str, flags: ToolTreeNode) -> bool {
        let ret = unsafe {
            cbg_Tool_TreeNodeEx(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                flags.bits as i32,
            )
        };
        ret
    }

    ///

    pub fn tree_pop(&mut self) -> () {
        unsafe { cbg_Tool_TreePop(self.self_ptr) }
    }

    ///

    pub fn set_next_item_open(&mut self, is_open: bool, cond: ToolCond) -> () {
        unsafe { cbg_Tool_SetNextItemOpen(self.self_ptr, is_open, cond as i32) }
    }

    ///

    pub fn button(&mut self, label: &str, mut size: crate::math::Vector2<f32>) -> bool {
        let ret =
            unsafe { cbg_Tool_Button(self.self_ptr, encode_string(&label).as_ptr(), size.clone()) };
        ret
    }

    ///

    pub fn check_box(&mut self, label: &str, is_open: &mut bool) -> bool {
        let ret = unsafe {
            cbg_Tool_CheckBox(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                is_open as *mut bool,
            )
        };
        ret
    }

    ///

    pub fn radio_button(&mut self, label: &str, active: bool) -> bool {
        let ret =
            unsafe { cbg_Tool_RadioButton(self.self_ptr, encode_string(&label).as_ptr(), active) };
        ret
    }

    ///

    pub fn arrow_button(&mut self, label: &str, dir: ToolDir) -> bool {
        let ret = unsafe {
            cbg_Tool_ArrowButton(self.self_ptr, encode_string(&label).as_ptr(), dir as i32)
        };
        ret
    }

    ///

    pub fn invisible_button(&mut self, label: &str, mut size: crate::math::Vector2<f32>) -> bool {
        let ret = unsafe {
            cbg_Tool_InvisibleButton(self.self_ptr, encode_string(&label).as_ptr(), size.clone())
        };
        ret
    }

    ///

    pub fn selectable(&mut self, label: &str, selected: &bool, flags: ToolSelectable) -> bool {
        let ret = unsafe {
            cbg_Tool_Selectable(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                selected as *const bool,
                flags.bits as i32,
            )
        };
        ret
    }

    ///

    pub fn input_int(&mut self, label: &str, value: &i32) -> bool {
        let ret = unsafe {
            cbg_Tool_InputInt(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                value as *const c_int,
            )
        };
        ret
    }

    ///

    pub fn input_float(&mut self, label: &str, value: &f32) -> bool {
        let ret = unsafe {
            cbg_Tool_InputFloat(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                value as *const c_float,
            )
        };
        ret
    }

    ///

    pub fn slider_int(
        &mut self,
        label: &str,
        value: &i32,
        speed: f32,
        value_min: i32,
        value_max: i32,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_SliderInt(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                value as *const c_int,
                speed,
                value_min,
                value_max,
            )
        };
        ret
    }

    ///

    pub fn slider_float(
        &mut self,
        label: &str,
        value: &f32,
        speed: f32,
        value_min: f32,
        value_max: f32,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_SliderFloat(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                value as *const c_float,
                speed,
                value_min,
                value_max,
            )
        };
        ret
    }

    ///

    pub fn slider_angle(&mut self, label: &str, angle: &f32) -> bool {
        let ret = unsafe {
            cbg_Tool_SliderAngle(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                angle as *const c_float,
            )
        };
        ret
    }

    ///

    pub fn vslider_int(
        &mut self,
        label: &str,
        mut size: crate::math::Vector2<f32>,
        value: &i32,
        value_min: i32,
        value_max: i32,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_VSliderInt(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                size.clone(),
                value as *const c_int,
                value_min,
                value_max,
            )
        };
        ret
    }

    ///

    pub fn vslider_float(
        &mut self,
        label: &str,
        mut size: crate::math::Vector2<f32>,
        value: &f32,
        value_min: f32,
        value_max: f32,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_VSliderFloat(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                size.clone(),
                value as *const c_float,
                value_min,
                value_max,
            )
        };
        ret
    }

    ///

    pub fn drag_int(
        &mut self,
        label: &str,
        value: &i32,
        speed: f32,
        value_min: i32,
        value_max: i32,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_DragInt(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                value as *const c_int,
                speed,
                value_min,
                value_max,
            )
        };
        ret
    }

    ///

    pub fn drag_float(
        &mut self,
        label: &str,
        value: &f32,
        speed: f32,
        value_min: f32,
        value_max: f32,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_DragFloat(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                value as *const c_float,
                speed,
                value_min,
                value_max,
            )
        };
        ret
    }

    ///

    pub fn open_popup(&mut self, label: &str) -> () {
        unsafe { cbg_Tool_OpenPopup(self.self_ptr, encode_string(&label).as_ptr()) }
    }

    /// `EndPopup()` を呼び出してください

    pub fn begin_popup(&mut self, label: &str) -> bool {
        let ret = unsafe { cbg_Tool_BeginPopup(self.self_ptr, encode_string(&label).as_ptr()) };
        ret
    }

    /// `EndPopup()` を呼び出してください

    pub fn begin_popup_modal(&mut self, label: &str) -> bool {
        let ret =
            unsafe { cbg_Tool_BeginPopupModal(self.self_ptr, encode_string(&label).as_ptr()) };
        ret
    }

    ///

    pub fn end_popup(&mut self) -> () {
        unsafe { cbg_Tool_EndPopup(self.self_ptr) }
    }

    /// `EndChild()` を呼び出してください

    pub fn begin_child(
        &mut self,
        label: &str,
        mut size: crate::math::Vector2<f32>,
        border: bool,
        flags: ToolWindow,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_BeginChild(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                size.clone(),
                border,
                flags.bits as i32,
            )
        };
        ret
    }

    ///

    pub fn end_child(&mut self) -> () {
        unsafe { cbg_Tool_EndChild(self.self_ptr) }
    }

    /// `EndMenuBar()` を呼び出してください

    pub fn begin_menu_bar(&mut self) -> bool {
        let ret = unsafe { cbg_Tool_BeginMenuBar(self.self_ptr) };
        ret
    }

    ///

    pub fn end_menu_bar(&mut self) -> () {
        unsafe { cbg_Tool_EndMenuBar(self.self_ptr) }
    }

    /// `EndMenu()` を呼び出してください

    pub fn begin_menu(&mut self, label: &str, enabled: bool) -> bool {
        let ret =
            unsafe { cbg_Tool_BeginMenu(self.self_ptr, encode_string(&label).as_ptr(), enabled) };
        ret
    }

    ///

    pub fn end_menu(&mut self) -> () {
        unsafe { cbg_Tool_EndMenu(self.self_ptr) }
    }

    ///

    pub fn menu_item(
        &mut self,
        label: &str,
        shortcut: &str,
        selected: bool,
        enabled: bool,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_MenuItem(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                encode_string(&shortcut).as_ptr(),
                selected,
                enabled,
            )
        };
        ret
    }

    /// `EndTabBar()` を呼び出してください

    pub fn begin_tab_bar(&mut self, label: &str, flags: ToolTabBar) -> bool {
        let ret = unsafe {
            cbg_Tool_BeginTabBar(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                flags.bits as i32,
            )
        };
        ret
    }

    ///

    pub fn end_tab_bar(&mut self) -> () {
        unsafe { cbg_Tool_EndTabBar(self.self_ptr) }
    }

    /// `EndTabItem()` を呼び出してください

    pub fn begin_tab_item(&mut self, label: &str) -> bool {
        let ret = unsafe { cbg_Tool_BeginTabItem(self.self_ptr, encode_string(&label).as_ptr()) };
        ret
    }

    ///

    pub fn end_tab_item(&mut self) -> () {
        unsafe { cbg_Tool_EndTabItem(self.self_ptr) }
    }

    ///

    pub fn indent(&mut self) -> () {
        unsafe { cbg_Tool_Indent(self.self_ptr) }
    }

    ///

    pub fn unindent(&mut self) -> () {
        unsafe { cbg_Tool_Unindent(self.self_ptr) }
    }

    ///

    pub fn separator(&mut self) -> () {
        unsafe { cbg_Tool_Separator(self.self_ptr) }
    }

    ///

    pub fn set_tooltip(&mut self, text: &str) -> () {
        unsafe { cbg_Tool_SetTooltip(self.self_ptr, encode_string(&text).as_ptr()) }
    }

    /// `EndTooltip()` を呼び出してください

    pub fn begin_tooltip(&mut self) -> () {
        unsafe { cbg_Tool_BeginTooltip(self.self_ptr) }
    }

    ///

    pub fn end_tooltip(&mut self) -> () {
        unsafe { cbg_Tool_EndTooltip(self.self_ptr) }
    }

    ///

    pub fn new_line(&mut self) -> () {
        unsafe { cbg_Tool_NewLine(self.self_ptr) }
    }

    ///

    pub fn same_line(&mut self) -> () {
        unsafe { cbg_Tool_SameLine(self.self_ptr) }
    }

    ///

    pub fn push_text_wrap_pos(&mut self, wrap_local_pos_x: f32) -> () {
        unsafe { cbg_Tool_PushTextWrapPos(self.self_ptr, wrap_local_pos_x) }
    }

    ///

    pub fn pop_text_wrap_pos(&mut self) -> () {
        unsafe { cbg_Tool_PopTextWrapPos(self.self_ptr) }
    }

    ///

    pub fn set_next_item_width(&mut self, width: f32) -> () {
        unsafe { cbg_Tool_SetNextItemWidth(self.self_ptr, width) }
    }

    ///

    pub fn push_item_width(&mut self, width: f32) -> () {
        unsafe { cbg_Tool_PushItemWidth(self.self_ptr, width) }
    }

    ///

    pub fn pop_item_width(&mut self) -> () {
        unsafe { cbg_Tool_PopItemWidth(self.self_ptr) }
    }

    ///

    pub fn push_button_repeat(&mut self, repeat: bool) -> () {
        unsafe { cbg_Tool_PushButtonRepeat(self.self_ptr, repeat) }
    }

    ///

    pub fn pop_button_repeat(&mut self) -> () {
        unsafe { cbg_Tool_PopButtonRepeat(self.self_ptr) }
    }

    ///

    pub fn columns(&mut self, count: i32, border: bool) -> () {
        unsafe { cbg_Tool_Columns(self.self_ptr, count, border) }
    }

    ///

    pub fn next_column(&mut self) -> () {
        unsafe { cbg_Tool_NextColumn(self.self_ptr) }
    }

    ///

    pub fn push_id(&mut self, id: i32) -> () {
        unsafe { cbg_Tool_PushID(self.self_ptr, id) }
    }

    ///

    pub fn pop_id(&mut self) -> () {
        unsafe { cbg_Tool_PopID(self.self_ptr) }
    }

    ///

    pub fn is_item_active(&mut self) -> bool {
        let ret = unsafe { cbg_Tool_IsItemActive(self.self_ptr) };
        ret
    }

    ///

    pub fn is_item_hovered(&mut self) -> bool {
        let ret = unsafe { cbg_Tool_IsItemHovered(self.self_ptr) };
        ret
    }

    ///

    pub fn set_scroll_here(&mut self) -> () {
        unsafe { cbg_Tool_SetScrollHere(self.self_ptr) }
    }

    ///

    pub fn set_scroll_here_x(&mut self) -> () {
        unsafe { cbg_Tool_SetScrollHereX(self.self_ptr) }
    }

    ///

    pub fn set_scroll_here_y(&mut self) -> () {
        unsafe { cbg_Tool_SetScrollHereY(self.self_ptr) }
    }

    ///

    pub fn get_text_line_height(&mut self) -> f32 {
        let ret = unsafe { cbg_Tool_GetTextLineHeight(self.self_ptr) };
        ret
    }

    ///

    pub fn get_font_size(&mut self) -> f32 {
        let ret = unsafe { cbg_Tool_GetFontSize(self.self_ptr) };
        ret
    }

    ///

    pub fn get_window_size(&mut self) -> crate::math::Vector2<f32> {
        let ret = unsafe { cbg_Tool_GetWindowSize(self.self_ptr) };
        ret
    }

    ///

    pub fn set_window_size(&mut self, mut size: crate::math::Vector2<f32>) -> () {
        unsafe { cbg_Tool_SetWindowSize(self.self_ptr, size.clone()) }
    }

    ///

    pub fn is_mouse_pos_valid(&mut self) -> bool {
        let ret = unsafe { cbg_Tool_IsMousePosValid(self.self_ptr) };
        ret
    }

    ///

    pub fn is_mouse_dragging(&mut self) -> bool {
        let ret = unsafe { cbg_Tool_IsMouseDragging(self.self_ptr) };
        ret
    }

    ///

    pub fn is_mouse_double_clicked(&mut self, button: i32) -> bool {
        let ret = unsafe { cbg_Tool_IsMouseDoubleClicked(self.self_ptr, button) };
        ret
    }

    ///

    pub fn get_mouse_drag_delta(&mut self, button: i32) -> crate::math::Vector2<f32> {
        let ret = unsafe { cbg_Tool_GetMouseDragDelta(self.self_ptr, button) };
        ret
    }

    ///

    pub fn reset_mouse_drag_delta(&mut self, button: i32) -> () {
        unsafe { cbg_Tool_ResetMouseDragDelta(self.self_ptr, button) }
    }

    ///

    pub fn set_next_window_content_size(&mut self, mut size: crate::math::Vector2<f32>) -> () {
        unsafe { cbg_Tool_SetNextWindowContentSize(self.self_ptr, size.clone()) }
    }

    ///

    pub fn set_next_window_size(&mut self, mut size: crate::math::Vector2<f32>) -> () {
        unsafe { cbg_Tool_SetNextWindowSize(self.self_ptr, size.clone()) }
    }

    ///

    pub fn set_next_window_pos(&mut self, mut pos: crate::math::Vector2<f32>) -> () {
        unsafe { cbg_Tool_SetNextWindowPos(self.self_ptr, pos.clone()) }
    }
}

impl Drop for Tool {
    fn drop(&mut self) {
        unsafe { cbg_Tool_Release(self.self_ptr) };
    }
}

/// 段階的にファイルを読み取るクラス
#[derive(Debug)]
pub struct StreamFile {
    self_ptr: *mut RawPtr,
}

unsafe impl Send for StreamFile {}
unsafe impl Sync for StreamFile {}

impl HasRawPtr for StreamFile {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl StreamFile {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Arc::new(Mutex::new(StreamFile { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        lazy_static! {
            static ref STREAMFILE_CACHE: RwLock<HashMap<RawPtrStorage, sync::Weak<Mutex<StreamFile>>>> =
                RwLock::new(HashMap::new());
        }
        let mut hash_map = STREAMFILE_CACHE.write().unwrap();
        let storage = RawPtrStorage(self_ptr);
        if let Some(x) = hash_map.get(&storage) {
            match x.upgrade() {
                Some(o) => {
                    return Some(o);
                }
                None => {
                    hash_map.remove(&storage);
                }
            }
        }
        let o = Self::cbg_create_raw(self_ptr)?;
        hash_map.insert(storage, Arc::downgrade(&o));
        Some(o)
    }

    /// 指定ファイルを読み込むStreamFileの新しいインスタンスを生成します。
    /// # Arguments
    /// * `path` - 読み込むファイルのパス

    pub(crate) fn __create(path: &str) -> Option<Arc<Mutex<StreamFile>>> {
        let ret = unsafe { cbg_StreamFile_Create(encode_string(&path).as_ptr()) };
        {
            let ret = StreamFile::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// 指定した分ファイルを読み込む
    /// # Arguments
    /// * `size` - この処理で読み込むデータサイズ

    pub fn read(&mut self, size: i32) -> i32 {
        let ret = unsafe { cbg_StreamFile_Read(self.self_ptr, size) };
        ret
    }

    /// 現在読み込んでいるファイルのデータを取得します。

    pub(crate) fn __get_temp_buffer(&mut self) -> Option<Rc<RefCell<Int8Array>>> {
        let ret = unsafe { cbg_StreamFile_GetTempBuffer(self.self_ptr) };
        {
            let ret = Int8Array::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// 再読み込みを行います。

    pub fn reload(&mut self) -> bool {
        let ret = unsafe { cbg_StreamFile_Reload(self.self_ptr) };
        ret
    }

    /// 読み込むファイルのデータサイズを取得します。
    pub fn get_size(&mut self) -> i32 {
        let ret = unsafe { cbg_StreamFile_GetSize(self.self_ptr) };
        ret
    }

    /// 現在読み込んでいるファイル上の位置を取得します。
    pub fn get_current_position(&mut self) -> i32 {
        let ret = unsafe { cbg_StreamFile_GetCurrentPosition(self.self_ptr) };
        ret
    }

    /// 現在読み込んでいるファイルのデータサイズを取得します。
    pub fn get_temp_buffer_size(&mut self) -> i32 {
        let ret = unsafe { cbg_StreamFile_GetTempBufferSize(self.self_ptr) };
        ret
    }

    /// 読み込むファイルがファイルパッケージ内に格納されているかどうかを取得します。
    pub fn get_is_in_package(&mut self) -> bool {
        let ret = unsafe { cbg_StreamFile_GetIsInPackage(self.self_ptr) };
        ret
    }

    /// 読み込んだファイルのパスを取得します。
    pub(crate) fn __get_path(&mut self) -> String {
        let ret = unsafe { cbg_StreamFile_GetPath(self.self_ptr) };
        decode_string(ret)
    }
}

impl Drop for StreamFile {
    fn drop(&mut self) {
        unsafe { cbg_StreamFile_Release(self.self_ptr) };
    }
}

/// 一度でファイルを読み取るクラス
#[derive(Debug)]
pub struct StaticFile {
    self_ptr: *mut RawPtr,
}

unsafe impl Send for StaticFile {}
unsafe impl Sync for StaticFile {}

impl HasRawPtr for StaticFile {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl StaticFile {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Arc::new(Mutex::new(StaticFile { self_ptr })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        lazy_static! {
            static ref STATICFILE_CACHE: RwLock<HashMap<RawPtrStorage, sync::Weak<Mutex<StaticFile>>>> =
                RwLock::new(HashMap::new());
        }
        let mut hash_map = STATICFILE_CACHE.write().unwrap();
        let storage = RawPtrStorage(self_ptr);
        if let Some(x) = hash_map.get(&storage) {
            match x.upgrade() {
                Some(o) => {
                    return Some(o);
                }
                None => {
                    hash_map.remove(&storage);
                }
            }
        }
        let o = Self::cbg_create_raw(self_ptr)?;
        hash_map.insert(storage, Arc::downgrade(&o));
        Some(o)
    }

    /// 指定ファイルを読み込んだStaticFileの新しいインスタンスを生成します。
    /// # Arguments
    /// * `path` - 読み込むファイルのパス

    pub(crate) fn __create(path: &str) -> Option<Arc<Mutex<StaticFile>>> {
        let ret = unsafe { cbg_StaticFile_Create(encode_string(&path).as_ptr()) };
        {
            let ret = StaticFile::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// 読み込んだファイルのデータを取得します。

    pub(crate) fn __get_buffer(&mut self) -> Option<Rc<RefCell<Int8Array>>> {
        let ret = unsafe { cbg_StaticFile_GetBuffer(self.self_ptr) };
        {
            let ret = Int8Array::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// 再読み込みを行います。

    pub fn reload(&mut self) -> bool {
        let ret = unsafe { cbg_StaticFile_Reload(self.self_ptr) };
        ret
    }

    /// 読み込んだファイルのパスを取得します。
    pub fn get_path(&mut self) -> String {
        let ret = unsafe { cbg_StaticFile_GetPath(self.self_ptr) };
        decode_string(ret)
    }

    /// 読み込んだファイルのデータサイズを取得します。
    pub fn get_size(&mut self) -> i32 {
        let ret = unsafe { cbg_StaticFile_GetSize(self.self_ptr) };
        ret
    }

    /// 読み込んだファイルがファイルパッケージ内に格納されているかどうかを取得します。
    pub fn get_is_in_package(&mut self) -> bool {
        let ret = unsafe { cbg_StaticFile_GetIsInPackage(self.self_ptr) };
        ret
    }
}

impl Drop for StaticFile {
    fn drop(&mut self) {
        unsafe { cbg_StaticFile_Release(self.self_ptr) };
    }
}

/// ファイル制御を行うクラス
#[derive(Debug)]
pub struct File {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for File {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl File {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(File { self_ptr })
    }

    /// インスタンスを取得します。

    pub(crate) fn __get_instance() -> Option<File> {
        let ret = unsafe { cbg_File_GetInstance() };
        File::cbg_create_raw(ret)
    }

    /// ファイル読み込み時に自動的に保管されるディレクトリを追加します。
    /// # Arguments
    /// * `path` - 追加するディレクトリ

    pub fn add_root_directory(&mut self, path: &str) -> bool {
        let ret =
            unsafe { cbg_File_AddRootDirectory(self.self_ptr, encode_string(&path).as_ptr()) };
        ret
    }

    /// ファイルパッケージをパスワード有りで読み込む
    /// # Arguments
    /// * `path` - 読み込むファイルパッケージのパス
    /// * `password` - 読み込むファイルパッケージのパスワード

    pub fn add_root_package_with_password(&mut self, path: &str, password: &str) -> bool {
        let ret = unsafe {
            cbg_File_AddRootPackageWithPassword(
                self.self_ptr,
                encode_string(&path).as_ptr(),
                encode_string(&password).as_ptr(),
            )
        };
        ret
    }

    /// ファイルパッケージをパスワード無しで読み込む
    /// # Arguments
    /// * `path` - 読み込むファイルパッケージのパス

    pub fn add_root_package(&mut self, path: &str) -> bool {
        let ret = unsafe { cbg_File_AddRootPackage(self.self_ptr, encode_string(&path).as_ptr()) };
        ret
    }

    /// 追加されたディレクトリやファイルパッケージをすべて削除します。

    pub fn clear_root_directories(&mut self) -> () {
        unsafe { cbg_File_ClearRootDirectories(self.self_ptr) }
    }

    /// 指定したファイルが存在するかどうかを検索します。
    /// # Arguments
    /// * `path` - 存在を確認するファイルのパス

    pub fn exists(&mut self, path: &str) -> bool {
        let ret = unsafe { cbg_File_Exists(self.self_ptr, encode_string(&path).as_ptr()) };
        ret
    }

    /// 指定したディレクトリのファイルをパックする
    /// # Arguments
    /// * `src_path` - パックするファイルのディレクトリ
    /// * `dst_path` - パックされたファイル名

    pub fn pack(&mut self, src_path: &str, dst_path: &str) -> bool {
        let ret = unsafe {
            cbg_File_Pack(
                self.self_ptr,
                encode_string(&src_path).as_ptr(),
                encode_string(&dst_path).as_ptr(),
            )
        };
        ret
    }

    /// 指定したディレクトリのファイルをパスワード付きでパックする
    /// # Arguments
    /// * `src_path` - パックするファイルのディレクトリ
    /// * `dst_path` - パックされたファイル名
    /// * `password` - かけるパスワード

    pub fn pack_with_password(&mut self, src_path: &str, dst_path: &str, password: &str) -> bool {
        let ret = unsafe {
            cbg_File_PackWithPassword(
                self.self_ptr,
                encode_string(&src_path).as_ptr(),
                encode_string(&dst_path).as_ptr(),
                encode_string(&password).as_ptr(),
            )
        };
        ret
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { cbg_File_Release(self.self_ptr) };
    }
}

/// 音源のクラス
#[derive(Debug)]
pub struct Sound {
    self_ptr: *mut RawPtr,
    loop_starting_point: Option<f32>,
    loop_end_point: Option<f32>,
    is_looping_mode: Option<bool>,
}

unsafe impl Send for Sound {}
unsafe impl Sync for Sound {}

impl HasRawPtr for Sound {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Sound {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Arc::new(Mutex::new(Sound {
            self_ptr,
            loop_starting_point: None,
            loop_end_point: None,
            is_looping_mode: None,
        })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        lazy_static! {
            static ref SOUND_CACHE: RwLock<HashMap<RawPtrStorage, sync::Weak<Mutex<Sound>>>> =
                RwLock::new(HashMap::new());
        }
        let mut hash_map = SOUND_CACHE.write().unwrap();
        let storage = RawPtrStorage(self_ptr);
        if let Some(x) = hash_map.get(&storage) {
            match x.upgrade() {
                Some(o) => {
                    return Some(o);
                }
                None => {
                    hash_map.remove(&storage);
                }
            }
        }
        let o = Self::cbg_create_raw(self_ptr)?;
        hash_map.insert(storage, Arc::downgrade(&o));
        Some(o)
    }

    /// 音声ファイルを読み込みます。
    /// # Arguments
    /// * `path` - 読み込む音声ファイルのパス
    /// * `is_decompressed` - 音を再生する前にデータを全て解凍するか?

    pub(crate) fn __load(path: &str, is_decompressed: bool) -> Option<Arc<Mutex<Sound>>> {
        let ret = unsafe { cbg_Sound_Load(encode_string(&path).as_ptr(), is_decompressed) };
        {
            let ret = Sound::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// ループ開始地点(秒)を取得または設定します。
    pub fn get_loop_starting_point(&mut self) -> f32 {
        if let Some(value) = &self.loop_starting_point {
            return value.clone();
        }
        let ret = unsafe { cbg_Sound_GetLoopStartingPoint(self.self_ptr) };
        ret
    }

    /// ループ開始地点(秒)を取得または設定します。
    pub fn set_loop_starting_point(&mut self, value: f32) {
        unsafe { cbg_Sound_SetLoopStartingPoint(self.self_ptr, value) }
        self.loop_starting_point = Some(value.clone());
    }

    /// ループ終了地点(秒)を取得または設定します。
    pub fn get_loop_end_point(&mut self) -> f32 {
        if let Some(value) = &self.loop_end_point {
            return value.clone();
        }
        let ret = unsafe { cbg_Sound_GetLoopEndPoint(self.self_ptr) };
        ret
    }

    /// ループ終了地点(秒)を取得または設定します。
    pub fn set_loop_end_point(&mut self, value: f32) {
        unsafe { cbg_Sound_SetLoopEndPoint(self.self_ptr, value) }
        self.loop_end_point = Some(value.clone());
    }

    /// ループするかどうかを取得または設定します。
    pub fn get_is_looping_mode(&mut self) -> bool {
        if let Some(value) = &self.is_looping_mode {
            return value.clone();
        }
        let ret = unsafe { cbg_Sound_GetIsLoopingMode(self.self_ptr) };
        ret
    }

    /// ループするかどうかを取得または設定します。
    pub fn set_is_looping_mode(&mut self, value: bool) {
        unsafe { cbg_Sound_SetIsLoopingMode(self.self_ptr, value) }
        self.is_looping_mode = Some(value.clone());
    }

    /// 音源の長さ(秒)を取得します。
    pub fn get_length(&mut self) -> f32 {
        let ret = unsafe { cbg_Sound_GetLength(self.self_ptr) };
        ret
    }

    /// 読み込んだファイルのパスを取得します。
    pub(crate) fn __get_path(&mut self) -> String {
        let ret = unsafe { cbg_Sound_GetPath(self.self_ptr) };
        decode_string(ret)
    }

    /// 音源を解凍するかどうかを取得する
    pub(crate) fn __get_is_decompressed(&mut self) -> bool {
        let ret = unsafe { cbg_Sound_GetIsDecompressed(self.self_ptr) };
        ret
    }
}

impl Drop for Sound {
    fn drop(&mut self) {
        unsafe { cbg_Sound_Release(self.self_ptr) };
    }
}

/// 音源を操作するクラス
#[derive(Debug)]
pub struct SoundMixer {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for SoundMixer {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl SoundMixer {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(SoundMixer { self_ptr })
    }

    pub(crate) fn __get_instance() -> Option<SoundMixer> {
        let ret = unsafe { cbg_SoundMixer_GetInstance() };
        SoundMixer::cbg_create_raw(ret)
    }

    /// 音を再生します。
    /// # Arguments
    /// * `sound` - 音源データ

    pub fn play(&mut self, sound: &mut Sound) -> i32 {
        let ret = unsafe { cbg_SoundMixer_Play(self.self_ptr, sound.self_ptr()) };
        ret
    }

    /// 指定した音が再生中であるかを取得します。
    /// # Arguments
    /// * `id` - 音のID

    pub fn get_is_playing(&mut self, id: i32) -> bool {
        let ret = unsafe { cbg_SoundMixer_GetIsPlaying(self.self_ptr, id) };
        ret
    }

    /// 再生中の音を全て停止します。

    pub fn stop_all(&mut self) -> () {
        unsafe { cbg_SoundMixer_StopAll(self.self_ptr) }
    }

    /// 指定した音の再生を停止します。
    /// # Arguments
    /// * `id` - 音のID

    pub fn stop(&mut self, id: i32) -> () {
        unsafe { cbg_SoundMixer_Stop(self.self_ptr, id) }
    }

    /// 指定した音の再生を一時停止します。
    /// # Arguments
    /// * `id` - 音のID

    pub fn pause(&mut self, id: i32) -> () {
        unsafe { cbg_SoundMixer_Pause(self.self_ptr, id) }
    }

    /// 指定した音の再生を再開します。
    /// # Arguments
    /// * `id` - 音のID

    pub fn resume(&mut self, id: i32) -> () {
        unsafe { cbg_SoundMixer_Resume(self.self_ptr, id) }
    }

    /// 指定した音の音量を変更します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `volume` - 音量(0.0〜1.0)

    pub fn set_volume(&mut self, id: i32, volume: f32) -> () {
        unsafe { cbg_SoundMixer_SetVolume(self.self_ptr, id, volume) }
    }

    /// 指定した音をフェードインさせる
    /// # Arguments
    /// * `id` -
    /// * `second` - フェードインに使用する時間(秒)

    pub fn fade_in(&mut self, id: i32, second: f32) -> () {
        unsafe { cbg_SoundMixer_FadeIn(self.self_ptr, id, second) }
    }

    /// 指定した音をフェードアウトさせる
    /// # Arguments
    /// * `id` - 音のID
    /// * `second` - フェードアウトに使用する時間(秒)

    pub fn fade_out(&mut self, id: i32, second: f32) -> () {
        unsafe { cbg_SoundMixer_FadeOut(self.self_ptr, id, second) }
    }

    /// 指定した音の音量を一定時間かけて変更します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `second` - フェードに使用する時間(秒)
    /// * `targeted_volume` - 変更後の音量(0.0〜1.0)

    pub fn fade(&mut self, id: i32, second: f32, targeted_volume: f32) -> () {
        unsafe { cbg_SoundMixer_Fade(self.self_ptr, id, second, targeted_volume) }
    }

    /// 再生速度を変更するかを取得します。
    /// # Arguments
    /// * `id` - 音のID

    pub fn get_is_playback_speed_enabled(&mut self, id: i32) -> bool {
        let ret = unsafe { cbg_SoundMixer_GetIsPlaybackSpeedEnabled(self.self_ptr, id) };
        ret
    }

    /// 再生速度を変更するかを設定します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `is_playback_speed_enabled` - 再生速度を変更するか?

    pub fn set_is_playback_speed_enabled(
        &mut self,
        id: i32,
        is_playback_speed_enabled: bool,
    ) -> () {
        unsafe {
            cbg_SoundMixer_SetIsPlaybackSpeedEnabled(self.self_ptr, id, is_playback_speed_enabled)
        }
    }

    /// 再生速度を取得します。
    /// # Arguments
    /// * `id` - 音のID

    pub fn get_playback_speed(&mut self, id: i32) -> f32 {
        let ret = unsafe { cbg_SoundMixer_GetPlaybackSpeed(self.self_ptr, id) };
        ret
    }

    /// 再生速度を設定します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `playback_speed` - 変更後の再生速度

    pub fn set_playback_speed(&mut self, id: i32, playback_speed: f32) -> () {
        unsafe { cbg_SoundMixer_SetPlaybackSpeed(self.self_ptr, id, playback_speed) }
    }

    /// パン位置を取得します。
    /// # Arguments
    /// * `id` - 音のID

    pub fn get_panning_position(&mut self, id: i32) -> f32 {
        let ret = unsafe { cbg_SoundMixer_GetPanningPosition(self.self_ptr, id) };
        ret
    }

    /// パン位置を設定します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `panning_position` - パン位置 : 0.0で中央, -1.0で左, 1.0で右

    pub fn set_panning_position(&mut self, id: i32, panning_position: f32) -> () {
        unsafe { cbg_SoundMixer_SetPanningPosition(self.self_ptr, id, panning_position) }
    }

    /// 指定した音の再生位置を取得します。
    /// # Arguments
    /// * `id` - 音のID

    pub fn get_playback_position(&mut self, id: i32) -> f32 {
        let ret = unsafe { cbg_SoundMixer_GetPlaybackPosition(self.self_ptr, id) };
        ret
    }

    /// 指定した音の再生位置を変更します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `position` - 再生位置(秒)

    pub fn set_playback_position(&mut self, id: i32, position: f32) -> () {
        unsafe { cbg_SoundMixer_SetPlaybackPosition(self.self_ptr, id, position) }
    }

    /// 再生中の音のスペクトル情報を取得します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `spectrums` - 音のスペクトル情報を格納するための配列
    /// * `window` - フーリエ変換に用いる窓関数

    pub(crate) fn __get_spectrum_data(
        &mut self,
        id: i32,
        spectrums: &mut FloatArray,
        window: FFTWindow,
    ) -> () {
        unsafe {
            cbg_SoundMixer_GetSpectrumData(self.self_ptr, id, spectrums.self_ptr(), window as i32)
        }
    }
}

impl Drop for SoundMixer {
    fn drop(&mut self) {
        unsafe { cbg_SoundMixer_Release(self.self_ptr) };
    }
}

/// ログを出力するクラス
#[derive(Debug)]
pub struct Log {
    self_ptr: *mut RawPtr,
}

impl HasRawPtr for Log {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Log {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Log { self_ptr })
    }

    /// インスタンスを取得します。

    pub(crate) fn __get_instance() -> Option<Log> {
        let ret = unsafe { cbg_Log_GetInstance() };
        Log::cbg_create_raw(ret)
    }

    /// ログを出力します。

    pub fn write(&mut self, category: LogCategory, level: LogLevel, message: &str) -> () {
        unsafe {
            cbg_Log_Write(
                self.self_ptr,
                category as i32,
                level as i32,
                encode_string(&message).as_ptr(),
            )
        }
    }

    /// LogLevel.Traceでログを出力します。

    pub fn trace(&mut self, category: LogCategory, message: &str) -> () {
        unsafe {
            cbg_Log_Trace(
                self.self_ptr,
                category as i32,
                encode_string(&message).as_ptr(),
            )
        }
    }

    /// LogLevel.Debugでログを出力します。

    pub fn debug(&mut self, category: LogCategory, message: &str) -> () {
        unsafe {
            cbg_Log_Debug(
                self.self_ptr,
                category as i32,
                encode_string(&message).as_ptr(),
            )
        }
    }

    /// LogLevel.Infoでログを出力します。

    pub fn info(&mut self, category: LogCategory, message: &str) -> () {
        unsafe {
            cbg_Log_Info(
                self.self_ptr,
                category as i32,
                encode_string(&message).as_ptr(),
            )
        }
    }

    /// LogLevel.Warningでログを出力します。

    pub fn warn(&mut self, category: LogCategory, message: &str) -> () {
        unsafe {
            cbg_Log_Warn(
                self.self_ptr,
                category as i32,
                encode_string(&message).as_ptr(),
            )
        }
    }

    /// LogLevel.Errorでログを出力します。

    pub fn error(&mut self, category: LogCategory, message: &str) -> () {
        unsafe {
            cbg_Log_Error(
                self.self_ptr,
                category as i32,
                encode_string(&message).as_ptr(),
            )
        }
    }

    /// LogLevel.Criticalでログを出力します。

    pub fn critical(&mut self, category: LogCategory, message: &str) -> () {
        unsafe {
            cbg_Log_Critical(
                self.self_ptr,
                category as i32,
                encode_string(&message).as_ptr(),
            )
        }
    }

    /// ログレベルを設定します。

    pub fn set_level(&mut self, category: LogCategory, level: LogLevel) -> () {
        unsafe { cbg_Log_SetLevel(self.self_ptr, category as i32, level as i32) }
    }
}

impl Drop for Log {
    fn drop(&mut self) {
        unsafe { cbg_Log_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub(crate) struct Window {
    self_ptr: *mut RawPtr,
    title: Option<String>,
}

impl HasRawPtr for Window {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl Window {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Self> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Window {
            self_ptr,
            title: None,
        })
    }

    /// インスタンスを取得します。

    pub(crate) fn get_instance() -> Option<Window> {
        let ret = unsafe { cbg_Window_GetInstance() };
        Window::cbg_create_raw(ret)
    }

    /// ウィンドウに表示するタイトルを取得または設定します
    pub fn get_title(&mut self) -> String {
        if let Some(value) = &self.title {
            return value.clone();
        }
        let ret = unsafe { cbg_Window_GetTitle(self.self_ptr) };
        decode_string(ret)
    }

    /// ウィンドウに表示するタイトルを取得または設定します
    pub fn set_title(&mut self, value: String) {
        unsafe { cbg_Window_SetTitle(self.self_ptr, encode_string(&value).as_ptr()) }
        self.title = Some(value.clone());
    }

    /// ウィンドウサイズを取得します
    pub fn get_size(&mut self) -> crate::math::Vector2<i32> {
        let ret = unsafe { cbg_Window_GetSize(self.self_ptr) };
        ret
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { cbg_Window_Release(self.self_ptr) };
    }
}

/// コライダの抽象基本クラスです
#[derive(Debug)]
pub struct Collider {
    self_ptr: *mut RawPtr,
    position: Option<crate::math::Vector2<f32>>,
    rotation: Option<f32>,
}

impl HasRawPtr for Collider {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

pub trait AsCollider: std::fmt::Debug + HasRawPtr {
    /// 指定したコライダとの衝突判定を行います。

    fn get_is_collided_with<T0: AsCollider>(&mut self, collider: &mut T0) -> bool;
    /// コライダの位置情報を取得または設定します。
    fn get_position(&mut self) -> crate::math::Vector2<f32>;
    /// コライダの位置情報を取得または設定します。
    fn set_position(&mut self, value: crate::math::Vector2<f32>);
    /// コライダの回転情報を取得または設定します。
    fn get_rotation(&mut self) -> f32;
    /// コライダの回転情報を取得または設定します。
    fn set_rotation(&mut self, value: f32);
}
impl AsCollider for Collider {
    /// 指定したコライダとの衝突判定を行います。

    fn get_is_collided_with<T0: AsCollider>(&mut self, collider: &mut T0) -> bool {
        let ret = unsafe { cbg_Collider_GetIsCollidedWith(self.self_ptr, collider.self_ptr()) };
        ret
    }

    /// コライダの位置情報を取得または設定します。
    fn get_position(&mut self) -> crate::math::Vector2<f32> {
        if let Some(value) = &self.position {
            return value.clone();
        }
        let ret = unsafe { cbg_Collider_GetPosition(self.self_ptr) };
        ret
    }

    /// コライダの位置情報を取得または設定します。
    fn set_position(&mut self, mut value: crate::math::Vector2<f32>) {
        unsafe { cbg_Collider_SetPosition(self.self_ptr, value.clone()) }
        self.position = Some(value.clone());
    }

    /// コライダの回転情報を取得または設定します。
    fn get_rotation(&mut self) -> f32 {
        if let Some(value) = &self.rotation {
            return value.clone();
        }
        let ret = unsafe { cbg_Collider_GetRotation(self.self_ptr) };
        ret
    }

    /// コライダの回転情報を取得または設定します。
    fn set_rotation(&mut self, value: f32) {
        unsafe { cbg_Collider_SetRotation(self.self_ptr, value) }
        self.rotation = Some(value.clone());
    }
}
impl Collider {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Collider {
            self_ptr,
            position: None,
            rotation: None,
        })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static COLLIDER_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Collider>>>> = RefCell::new(HashMap::new());
        }
        COLLIDER_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    pub(crate) fn new() -> Option<Rc<RefCell<Collider>>> {
        Self::cbg_create_raw(unsafe { cbg_Collider_Constructor_0() })
    }

    /// コライダの位置情報を取得または設定します。
    pub fn set_position(&mut self, mut value: crate::math::Vector2<f32>) {
        unsafe { cbg_Collider_SetPosition(self.self_ptr, value.clone()) }
        self.position = Some(value.clone());
    }

    /// コライダの回転情報を取得または設定します。
    pub fn set_rotation(&mut self, value: f32) {
        unsafe { cbg_Collider_SetRotation(self.self_ptr, value) }
        self.rotation = Some(value.clone());
    }
}

impl Drop for Collider {
    fn drop(&mut self) {
        unsafe { cbg_Collider_Release(self.self_ptr) };
    }
}

/// 円形コライダのクラス
#[derive(Debug)]
pub struct CircleCollider {
    self_ptr: *mut RawPtr,
    radius: Option<f32>,
    position: Option<crate::math::Vector2<f32>>,
    rotation: Option<f32>,
}

impl HasRawPtr for CircleCollider {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl AsCollider for CircleCollider {
    /// 指定したコライダとの衝突判定を行います。

    fn get_is_collided_with<T0: AsCollider>(&mut self, collider: &mut T0) -> bool {
        let ret = unsafe { cbg_Collider_GetIsCollidedWith(self.self_ptr, collider.self_ptr()) };
        ret
    }

    /// コライダの位置情報を取得または設定します。
    fn get_position(&mut self) -> crate::math::Vector2<f32> {
        if let Some(value) = &self.position {
            return value.clone();
        }
        let ret = unsafe { cbg_Collider_GetPosition(self.self_ptr) };
        ret
    }

    /// コライダの位置情報を取得または設定します。
    fn set_position(&mut self, mut value: crate::math::Vector2<f32>) {
        unsafe { cbg_Collider_SetPosition(self.self_ptr, value.clone()) }
        self.position = Some(value.clone());
    }

    /// コライダの回転情報を取得または設定します。
    fn get_rotation(&mut self) -> f32 {
        if let Some(value) = &self.rotation {
            return value.clone();
        }
        let ret = unsafe { cbg_Collider_GetRotation(self.self_ptr) };
        ret
    }

    /// コライダの回転情報を取得または設定します。
    fn set_rotation(&mut self, value: f32) {
        unsafe { cbg_Collider_SetRotation(self.self_ptr, value) }
        self.rotation = Some(value.clone());
    }
}
impl CircleCollider {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(CircleCollider {
            self_ptr,
            radius: None,
            position: None,
            rotation: None,
        })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static CIRCLECOLLIDER_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<CircleCollider>>>> = RefCell::new(HashMap::new());
        }
        CIRCLECOLLIDER_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// 円形コライダの半径を取得または設定します。
    pub fn get_radius(&mut self) -> f32 {
        if let Some(value) = &self.radius {
            return value.clone();
        }
        let ret = unsafe { cbg_CircleCollider_GetRadius(self.self_ptr) };
        ret
    }

    /// 円形コライダの半径を取得または設定します。
    pub fn set_radius(&mut self, value: f32) {
        unsafe { cbg_CircleCollider_SetRadius(self.self_ptr, value) }
        self.radius = Some(value.clone());
    }
}

impl Drop for CircleCollider {
    fn drop(&mut self) {
        unsafe { cbg_CircleCollider_Release(self.self_ptr) };
    }
}

/// 矩形コライダのクラス
#[derive(Debug)]
pub struct RectangleCollider {
    self_ptr: *mut RawPtr,
    size: Option<crate::math::Vector2<f32>>,
    center_position: Option<crate::math::Vector2<f32>>,
    position: Option<crate::math::Vector2<f32>>,
    rotation: Option<f32>,
}

impl HasRawPtr for RectangleCollider {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl AsCollider for RectangleCollider {
    /// 指定したコライダとの衝突判定を行います。

    fn get_is_collided_with<T0: AsCollider>(&mut self, collider: &mut T0) -> bool {
        let ret = unsafe { cbg_Collider_GetIsCollidedWith(self.self_ptr, collider.self_ptr()) };
        ret
    }

    /// コライダの位置情報を取得または設定します。
    fn get_position(&mut self) -> crate::math::Vector2<f32> {
        if let Some(value) = &self.position {
            return value.clone();
        }
        let ret = unsafe { cbg_Collider_GetPosition(self.self_ptr) };
        ret
    }

    /// コライダの位置情報を取得または設定します。
    fn set_position(&mut self, mut value: crate::math::Vector2<f32>) {
        unsafe { cbg_Collider_SetPosition(self.self_ptr, value.clone()) }
        self.position = Some(value.clone());
    }

    /// コライダの回転情報を取得または設定します。
    fn get_rotation(&mut self) -> f32 {
        if let Some(value) = &self.rotation {
            return value.clone();
        }
        let ret = unsafe { cbg_Collider_GetRotation(self.self_ptr) };
        ret
    }

    /// コライダの回転情報を取得または設定します。
    fn set_rotation(&mut self, value: f32) {
        unsafe { cbg_Collider_SetRotation(self.self_ptr, value) }
        self.rotation = Some(value.clone());
    }
}
impl RectangleCollider {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(RectangleCollider {
            self_ptr,
            size: None,
            center_position: None,
            position: None,
            rotation: None,
        })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static RECTANGLECOLLIDER_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<RectangleCollider>>>> = RefCell::new(HashMap::new());
        }
        RECTANGLECOLLIDER_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// 矩形コライダの幅・高さを取得または設定します。
    pub fn get_size(&mut self) -> crate::math::Vector2<f32> {
        if let Some(value) = &self.size {
            return value.clone();
        }
        let ret = unsafe { cbg_RectangleCollider_GetSize(self.self_ptr) };
        ret
    }

    /// 矩形コライダの幅・高さを取得または設定します。
    pub fn set_size(&mut self, mut value: crate::math::Vector2<f32>) {
        unsafe { cbg_RectangleCollider_SetSize(self.self_ptr, value.clone()) }
        self.size = Some(value.clone());
    }

    /// 矩形コライダの中心の位置を取得または設定します。
    pub fn get_center_position(&mut self) -> crate::math::Vector2<f32> {
        if let Some(value) = &self.center_position {
            return value.clone();
        }
        let ret = unsafe { cbg_RectangleCollider_GetCenterPosition(self.self_ptr) };
        ret
    }

    /// 矩形コライダの中心の位置を取得または設定します。
    pub fn set_center_position(&mut self, mut value: crate::math::Vector2<f32>) {
        unsafe { cbg_RectangleCollider_SetCenterPosition(self.self_ptr, value.clone()) }
        self.center_position = Some(value.clone());
    }
}

impl Drop for RectangleCollider {
    fn drop(&mut self) {
        unsafe { cbg_RectangleCollider_Release(self.self_ptr) };
    }
}

/// 多角形コライダのクラス
#[derive(Debug)]
pub struct PolygonCollider {
    self_ptr: *mut RawPtr,
    vertexes: Option<Rc<RefCell<Vector2FArray>>>,
    position: Option<crate::math::Vector2<f32>>,
    rotation: Option<f32>,
}

impl HasRawPtr for PolygonCollider {
    fn self_ptr(&mut self) -> *mut RawPtr {
        self.self_ptr.clone()
    }
}

impl AsCollider for PolygonCollider {
    /// 指定したコライダとの衝突判定を行います。

    fn get_is_collided_with<T0: AsCollider>(&mut self, collider: &mut T0) -> bool {
        let ret = unsafe { cbg_Collider_GetIsCollidedWith(self.self_ptr, collider.self_ptr()) };
        ret
    }

    /// コライダの位置情報を取得または設定します。
    fn get_position(&mut self) -> crate::math::Vector2<f32> {
        if let Some(value) = &self.position {
            return value.clone();
        }
        let ret = unsafe { cbg_Collider_GetPosition(self.self_ptr) };
        ret
    }

    /// コライダの位置情報を取得または設定します。
    fn set_position(&mut self, mut value: crate::math::Vector2<f32>) {
        unsafe { cbg_Collider_SetPosition(self.self_ptr, value.clone()) }
        self.position = Some(value.clone());
    }

    /// コライダの回転情報を取得または設定します。
    fn get_rotation(&mut self) -> f32 {
        if let Some(value) = &self.rotation {
            return value.clone();
        }
        let ret = unsafe { cbg_Collider_GetRotation(self.self_ptr) };
        ret
    }

    /// コライダの回転情報を取得または設定します。
    fn set_rotation(&mut self, value: f32) {
        unsafe { cbg_Collider_SetRotation(self.self_ptr, value) }
        self.rotation = Some(value.clone());
    }
}
impl PolygonCollider {
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(PolygonCollider {
            self_ptr,
            vertexes: None,
            position: None,
            rotation: None,
        })))
    }

    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static POLYGONCOLLIDER_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<PolygonCollider>>>> = RefCell::new(HashMap::new());
        }
        POLYGONCOLLIDER_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// 多角形コライダの頂点の座標を取得または設定します
    pub(crate) fn __get_vertexes(&mut self) -> Option<Rc<RefCell<Vector2FArray>>> {
        if let Some(value) = &self.vertexes {
            return Some(value.clone());
        }
        let ret = unsafe { cbg_PolygonCollider_GetVertexes(self.self_ptr) };
        {
            let ret = Vector2FArray::try_get_from_cache(ret)?;
            Some(ret)
        }
    }

    /// 多角形コライダの頂点の座標を取得または設定します
    pub(crate) fn __set_vertexes(&mut self, value: Rc<RefCell<Vector2FArray>>) {
        unsafe { cbg_PolygonCollider_SetVertexes(self.self_ptr, value.borrow_mut().self_ptr()) }
        self.vertexes = Some(value.clone());
    }
}

impl Drop for PolygonCollider {
    fn drop(&mut self) {
        unsafe { cbg_PolygonCollider_Release(self.self_ptr) };
    }
}
