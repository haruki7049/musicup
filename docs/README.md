# How to use musicup

This documentation is written simply for how to use musicup.

## Installation

``` shell
cargo install --git https://github.com/haruki7049/musicup

# Or you can use Nix flakes

nix build github:haruki7049/musicup#musicup
```

## How to setup

1. Execute `musicup`. Then, it will generate the configuration file. On Linux `/home/{username}/.config/musicup/musicup.toml`, on macOS `/Users/{username}/Library/Application\ Support/rs.musicup/musicup.toml`. You should stop the musicup when it was generated.
2. `music.toml`の`music_dir`に書かれたパスに音楽データを配置する。
2. Locate your music data to the path written on `music_dir` in `music.toml`.
3. Execute `musicup` again. The IP default value is `127.0.0.1:3000`
4. Access the referenced IP address, and then browser should download the zip file zipped your music file..
