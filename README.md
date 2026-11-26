# bevy-sandbox

[Bevy](https://bevy.org/)を試す場所です。今は、回る立方体と文字を出すだけのhello worldです。

## 動かし方

[rustup](https://rustup.rs/)を入れて、このフォルダーで次を打ちます。Rustの版は`rust-toolchain.toml`で決めてあり、rustupが自動で入れます。

```bash
cargo run
```

最初のbuildは、Bevyのcompileに数分かかります。2回目からは速くなります。

OSごとに、先に要るもの:

- Windows：Visual Studio Build Toolsの「C++によるデスクトップ開発」。rustupのinstallerが案内します
- Linux（Ubuntu）：`sudo apt install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 libwayland-dev libxkbcommon-dev`
- macOS：Xcode Command Line Tools（`xcode-select --install`）

何も入っていない機械からの手順は、[環境構築のrunbook](docs/setup-runbook_ja.md)にあります。
