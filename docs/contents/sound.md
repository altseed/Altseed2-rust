# サウンド機能について

```ignore
let bgm: Arc<Mutex<Sound>> = engine.loader().load_sound("./bgm.ogg", false)?;
let bgm_id: SoundID = engine.sound().borrow_mut().play(&mut bgm.lock())?;

let se: Arc<Mutex<Sound>> = engine.loader().load_sound("./se.ogg", true)?;
let se_id: SoundID = engine.sound().borrow_mut().play(&mut se.lock())?;
```

## Sound

[Sound](../../core/struct.Sound.html)はサウンドファイルを読み込んで生成される音源データを表します。

[Loader::load_sound](../../engine/struct.Loader.html#method.load_sound)を利用して`.ogg`や`.wav`といった音源ファイルを読み込みます。

`is_decompressed`引数で、事前に音のデータを解凍するかどうかを指定できます。

OGG形式などでは音のデータが圧縮されており、再生前に解凍する必要があります。
読み込み時に音のデータを解凍しなかった場合、再生しながらリアルタイムに解凍することになります。

#### IMPORTANT
音のデータをを事前に解凍するか、再生中にリアルタイムに解凍するかによって、消費するメモリやCPUへの負荷が異なります。

事前に解凍する場合、読み込み時間は長くなりメモリも消費しますが再生時のCPUへの負荷は小さくなります。

一方、再生中にリアルタイムに解凍する場合は、読み込み時間もメモリも小さくなりますが、再生時に若干CPUの処理を必要とします。

基本的に、**再生時間が長いBGMは圧縮したまま、音が短くたくさん鳴らすSEは事前に解凍**して読み込みます。

### ループ再生
- [Sound::set_is_looping_mode](../../core/struct.Sound.html#method.set_is_looping_mode)

を利用します。

### ループ範囲を指定する
- [Sound::set_loop_starting_point](../../core/struct.Sound.html#method.set_loop_starting_point)
- [Sound::set_loop_end_point](../../core/struct.Sound.html#method.set_loop_end_point)

を利用します。
`loop_starting_point`から`loop_end_point`の範囲でループ再生されます。

<!-- ![loop](loop.png) -->

## SoundMixer
[SoundMixer](../../core/struct.SoundMixer.html)は音の再生・停止・などの機能を提供します。
[Engine::sound](../../engine/struct.Engine.html#method.sound)を利用してください。

できること：
- 音の停止・一時停止・再開
- 音量の変更・フェードイン/アウト
- 再生速度・パン位置・再生位置の取得・変更
- スペクトル情報の取得

### 音源ファイルを再生する
[SoundMixer::play](../../core/struct.SoundMixer.html#method.play)を利用します。
返り値として、**音の再生**に対応するIDが返されます。

[SoundMixer](../../core/struct.SoundMixer.html)の各メソッドにこのIDを渡すことで、再生されている音に対して操作をすることができます。  

**TIPS**:  
Altseedでは再生されている音に対する操作をIDを通して行います。
音のデータそのものとは別に、音の再生情報に関するデータを管理するためです。

**TIPS**:  
BGMなどを除いて、再生中に停止などの操作を行うことは稀です。
再生の度に再生情報を含むインスタンスを生成するオーバーヘッドを減らすため、都度IDを用いて操作する仕組みになっています。

<!-- ## サンプル TODO

### 効果音の再生

[!code-csharp[Main](../../Src/Samples/Sound/SE.cs)]

### BGMの再生

[!code-csharp[Main](../../Src/Samples/Sound/BGM.cs)]

### BGMのループ再生

[!code-csharp[Main](../../Src/Samples/Sound/LoopingBGM.cs)] -->