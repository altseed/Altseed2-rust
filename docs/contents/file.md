# ファイル機能について

## File
[File](../../core/struct.File.html)はファイルを読み込む先のディレクトリ・パッケージの指定やディレクトリのパッケージ化を行います。
[Engine::file](../../engine/struct.Engine.html#method.file)を利用してください。

### ファイルを読み込む時のルートを指定する
[File::add_root_directory](../../core/struct.File.html#method.add_root_directory)を利用します。
- 指定なし: カレントディレクトリから相対的にファイルを読み込みます。
- 指定あり: ルートディレクトリから相対的にファイルを読み込みます。

### パッケージをルートに指定する
[File::add_root_package](../../core/struct.File.html#method.add_root_package)を利用します。
複数のファイルを1つにまとめたパッケージをルートにして、ファイルを読み込みます。

複数のルートディレクトリやパッケージを指定できます。
ファイルを読み込む時に複数のファイルが見つかった時、同じ名前のファイルは後から追加されたルートディレクトリやパッケージから読み込まれます。

絶対パスを指定した場合、ルートの追加順序に関係なく絶対パスで指定された先を読み込みます。

### パッケージを作成する

[pack](../../core/struct.File.html#method.pack)を利用します。
パッキングするディレクトリと出力先のパスを指定します。

```ignore
engine.file().borrow_mut().pack("./hoge", "./hoge.pack");
```

### パスワード付きでパッケージを作成する

[pack_with_password](../../core/struct.File.html#method.pack_with_password)を利用します。
パスワード付きのパッケージにすることで、パッケージ化されたリソースデータをユーザから抽出されるのを防ぐことができます。

### パスワード付きのパッケージをルートに指定する
[add_root_package_with_password](../../core/struct.File.html#method.add_root_package_with_password)を使用します。

**TIPS**:  
複数のパッケージを読み込んだ時の優先順位を利用することでアップデートパッチを容易に実装できます。

例えば、

1. 製品の最初で、**パッケージX**に格納されているファイルAがあります。
2. ファイルAの不具合が発覚し、差し替えることになりました。
3. 更新した新しいファイルAを**パッケージY**に格納します。
4. プログラム側では**パッケージX**, **パッケージY**の順に追加します。
5. 同じ名前のファイルAを読み込むときに**パッケージX**からでなく、後から追加した**パッケージY**から読み込まれます。
6. 更新したファイルのみを別のパッケージにまとめることで、 実際に読み込む際のファイルのパス指定を変更することなくファイルを更新できます。

## StaticFile
[StaticFile](../../core/struct.StaticFile.html)はファイルを一括で全て読み込むクラスです。

[Loader::load_static_file](../../engine/struct.Loader.html#method.load_static_file)を利用して読み込むことができます。

<!-- 読み込んだファイルの内容は、[StaticFile::buffer](../../core/struct.StaticFile.html#buffer)で、Byte配列として得られます。 TODO -->

**TIPS**:  
一度読み込まれると内部でキャッシュされます。
よって、同じパスでファイルを読み込んだ場合、キャッシュから読み込まれるため、読み込み時間が小さくなります。

## StreamFile

[StreamFile](../../core/struct.StreamFile.html)はファイルを部分的に読み込むクラスです。

[Loader::load_stream_file](../../engine/struct.Loader.html#method.load_stream_file)を利用して読み込むことができます。

### 指定したサイズだけ読み込む
[StreamFile::read](../../core/struct.StreamFile.html#method.read)を利用します。

<!-- 読み込んだ内容は、[TempBuffer](xref:Altseed.StreamFile.TempBuffer)で、Byte配列として得られます。 TODO -->

### 読み込み済みのデータのサイズを取得する
- [StreamFile::get_temp_buffer_size](../../core/struct.StreamFile.html#method.get_temp_buffer_size)を利用します。

<!-- ## サンプル TODO

### StaticFileによるファイル読み込み


### StreamFileによるファイル読み込み -->

