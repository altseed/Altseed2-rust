use crate::auto_generated_core_binding::SoundMixer as CoreMixer;
use crate::auto_generated_core_binding::{FFTWindow, FloatArray, Sound};
use crate::error::*;

/// 再生する[Sound](struct.Sound.html)のIDを表します。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundID {
    value: i32,
}

impl SoundID {
    fn new(id: i32) -> Option<Self> {
        if id < 0 {
            None
        } else {
            Some(SoundID { value: id })
        }
    }
}

/// [Sound](struct.Sound.html)の操作を行います。
pub struct SoundMixer {
    instance: CoreMixer,
}

impl SoundMixer {
    pub(crate) fn new() -> Option<Self> {
        Some(SoundMixer {
            instance: CoreMixer::__get_instance()?,
        })
    }

    /// 音を再生します。
    pub fn play(&mut self, sound: &mut Sound) -> AltseedResult<SoundID> {
        SoundID::new(self.instance.play(sound))
            .ok_or(AltseedError::FailedToPlaySound(sound.__get_path()))
    }

    /// 指定した音が再生中であるかを取得します。
    pub fn is_playing(&mut self, id: SoundID) -> bool {
        self.instance.get_is_playing(id.value)
    }

    /// 再生中の音を全て停止します。
    pub fn stop_all(&mut self) {
        self.instance.stop_all()
    }

    /// 指定した音の再生を停止します。
    pub fn stop(&mut self, id: SoundID) {
        self.instance.stop(id.value)
    }

    /// 指定した音の再生を一時停止します。
    pub fn pause(&mut self, id: SoundID) {
        self.instance.pause(id.value)
    }

    /// 指定した音の再生を再開します。
    pub fn resume(&mut self, id: SoundID) {
        self.instance.resume(id.value)
    }

    /// 指定した音の音量を変更します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `volume` - 音量(0.0〜1.0)
    pub fn set_volume(&mut self, id: SoundID, volume: f32) {
        self.instance.set_volume(id.value, volume)
    }

    /// 指定した音をフェードインさせる
    /// # Arguments
    /// * `id` - 音のID
    /// * `second` - フェードインに使用する時間(秒)
    pub fn fade_in(&mut self, id: SoundID, second: f32) {
        self.instance.fade_in(id.value, second)
    }

    /// 指定した音をフェードアウトさせる
    /// # Arguments
    /// * `id` - 音のID
    /// * `second` - フェードアウトに使用する時間(秒)
    pub fn fade_out(&mut self, id: SoundID, second: f32) {
        self.instance.fade_out(id.value, second)
    }

    /// 指定した音の音量を一定時間かけて変更します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `second` - フェードに使用する時間(秒)
    /// * `target_volume` - 変更後の音量(0.0〜1.0)
    pub fn fade(&mut self, id: SoundID, second: f32, target_volume: f32) {
        self.instance.fade(id.value, second, target_volume)
    }

    /// 再生速度を変更するかを取得します。
    pub fn get_is_playback_speed_enabled(&mut self, id: SoundID) -> bool {
        self.instance.get_is_playback_speed_enabled(id.value)
    }

    /// 再生速度を変更するかを設定します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `is_playback_speed_enabled` - 再生速度を変更するか?
    pub fn set_is_playback_speed_enabled(&mut self, id: SoundID, enabled: bool) {
        self.instance
            .set_is_playback_speed_enabled(id.value, enabled)
    }

    /// 再生速度を取得します。
    /// # Arguments
    /// * `id` - 音のID
    pub fn get_playback_speed(&mut self, id: SoundID) -> f32 {
        self.instance.get_playback_speed(id.value)
    }

    /// 再生速度を設定します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `playback_speed` - 変更後の再生速度
    pub fn set_playback_speed(&mut self, id: SoundID, playback_speed: f32) {
        self.instance.set_playback_speed(id.value, playback_speed)
    }

    /// パン位置を取得します。
    /// # Arguments
    /// * `id` - 音のID
    pub fn get_panning_pos(&mut self, id: SoundID) -> f32 {
        self.instance.get_panning_position(id.value)
    }

    /// パン位置を設定します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `panning_position` - パン位置 : 0.0で中央, -1.0で左, 1.0で右
    pub fn set_panning_pos(&mut self, id: SoundID, pos: f32) {
        self.instance.set_panning_position(id.value, pos)
    }

    /// 指定した音の再生位置を取得します。
    /// # Arguments
    /// * `id` - 音のID
    pub fn get_playback_pos(&mut self, id: SoundID) -> f32 {
        self.instance.get_playback_position(id.value)
    }

    /// 指定した音の再生位置を変更します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `position` - 再生位置(秒)
    pub fn set_playback_pos(&mut self, id: SoundID, pos: f32) {
        self.instance.set_playback_position(id.value, pos)
    }

    pub fn spectrum(&mut self, id: SoundID, window: FFTWindow) -> Vec<f32> {
        let vec = FloatArray::create(8192).unwrap();
        let mut v = vec.borrow_mut();
        self.instance.__get_spectrum_data(id.value, &mut v, window);
        v.to_vec()
    }
}
