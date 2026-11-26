# bevy-sandboxの環境構築

何も入っていない機械から、`cargo run`で回る立方体を出すまでの手順です。段ごとに「確かめ」を置きました。通らなかったら、そこで止めて「うまくいかないとき」を見てください。

確かめた範囲（2026-11-26）：Ubuntu 24.04（Intel UHD Graphics 620）で、「4. 取ってきて動かす」のbuildが通り、窓が開きました。その機械には、1〜3がもう入っていました。Windowsの手順は、まだ誰も通していません。

## Windows（10／11）

### 1. C++のbuild tools

Windows版のRust（MSVC版）は、MicrosoftのリンカーとWindows SDKを使います。

1. Microsoftの「Visual Studioのダウンロード」のページから、「Build Tools for Visual Studio」のinstallerを落として実行します
2. 「ワークロード」で「C++によるデスクトップ開発」に印を付けて、インストールします。MSVCとWindows SDKが入ります

別の道：入れずに次の「2. Rust」へ進むと、rustup-initが足りないことに気づき、Visual Studio Communityのinstallerで入れる選択肢を出します。

ライセンスは「個人、学術、オープンソースは無料」と案内されます。会社の端末では条件を確かめてください。

確かめ：「Visual Studio Installer」を開くと、Build Toolsの「変更」の中で「C++によるデスクトップ開発」に印が付いている。

### 2. Rust（rustup）

1. <https://rustup.rs/> から`rustup-init.exe`（x64）を落として実行します。wingetが使えるなら、`winget install --id Rustlang.Rustup -e`でも入ります
2. 聞かれたら、既定（`1) Proceed with standard installation`）を選びます

確かめ：新しいターミナル（PowerShell）を開いて、次の2つが版を表示する。

```powershell
rustup --version
cargo --version
```

### 3. Git

```powershell
winget install --id Git.Git -e
```

確かめ：新しいターミナルで`git --version`が版を表示する。

### 4. 取ってきて動かす

```powershell
git clone https://github.com/Naohiro2g/bevy-sandbox.git
cd bevy-sandbox
cargo run
```

- 最初に、rustupが`rust-toolchain.toml`に書いた版（1.100.0）を入れます。手順2で入った版より新しくても古くても、このフォルダーの中ではこの版が使われます
- 最初のbuildは、Bevyのcompileに数分かかります。`target`フォルダーは数GBになります。2回目からは速くなります

確かめ：窓が開き、緑の立方体が回り、左上に「Hello, Bevy!」が出る。窓を閉じると終わる。

### 5. VS Code

1. VS Codeを入れます（`winget install --id Microsoft.VisualStudioCode -e`）
2. 拡張の「rust-analyzer」を入れます。補完、型の表示、エラーの表示をします
3. デバッガーを使うときは、拡張の「C/C++」（Microsoft）か「CodeLLDB」を入れます
4. 「フォルダーを開く」で`bevy-sandbox`を開きます

確かめ：`src/main.rs`で`Transform`などにマウスを乗せると、型の説明が出る。

## Linux（Ubuntu）

### 1. compilerと、Bevyが使うlibrary

```bash
sudo apt install build-essential curl git pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 libwayland-dev libxkbcommon-dev
```

GPUによっては、Vulkanのdriver（`mesa-vulkan-drivers`など）も要ります（Bevyの[Linuxの依存の一覧](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)）。

### 2. Rust（rustup）

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

既定（`1`）を選び、終わったら新しいターミナルを開きます。

確かめ：`rustup --version`と`cargo --version`が版を表示する。

### 3. 取ってきて動かす

Windowsの「4. 取ってきて動かす」と同じです（`git clone`、`cd bevy-sandbox`、`cargo run`）。VS Codeも同じです。

## macOS

1. `xcode-select --install`でXcode Command Line Toolsを入れます
2. Linuxの「2. Rust（rustup）」と同じcommandでrustupを入れます
3. Windowsの「4. 取ってきて動かす」と同じです

## うまくいかないとき

- Windowsで`linker 'link.exe' not found`：「1. C++のbuild tools」のワークロードが入っていません。Visual Studio Installerで足して、ターミナルを開き直します
- `rustc 1.97.1 or newer`のような版のエラー：`bevy-sandbox`のフォルダーの中で打っているかを見ます。`rustup show`の最後に`overridden by '…rust-toolchain.toml'`と出れば、このフォルダーの版が使われています
- Linuxで`alsa`や`libudev`が見つからない：「1. compilerと、Bevyが使うlibrary」を入れます
- Linux（Wayland）で`WARN sctk_adwaita::buttons: Ignoring unknown button type`が出る：窓の枠のボタンの設定を読めなかったという知らせで、動きには関係ありません
- 窓が開かない、GPUのエラーが出る：GPUのdriverを新しくします。LinuxではVulkanのdriverを入れます
- buildをもっと速くしたい：Bevyの[setupのページ](https://bevy.org/learn/quick-start/getting-started/setup/)に、`bevy/dynamic_linking`や速いリンカーの使い方があります

## 参考

- Bevyのsetupのページ：<https://bevy.org/learn/quick-start/getting-started/setup/>
- rustup：<https://rustup.rs/>
