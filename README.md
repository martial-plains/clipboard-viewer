<div align="center">
	<h1>Clipboard Viewer</h1>
	<p>
		<b>Inspect the system clipboards</b>
	</p>
	<br>
	<br>
	<br>
</div>

[![dependency status](https://deps.rs/repo/github/a-isaiahharvey/rust-macios/status.svg)](https://deps.rs/repo/github/a-isaiahharvey/rust-macios)
[![CI](https://github.com/a-isaiahharvey/clipboard-viewer/actions/workflows/rust.yml/badge.svg)](https://github.com/a-isaiahharvey/clipboard-viewer/actions/workflows/rust.yml)

## Disclaimer

**This app currently only works on macOS**

## About

This is a developer utility that allows you to inspect the various system clipboards.

Note that this is not a clipboard manager. If you're not an app developer, you probably don't want this app.

## Screenshot

![](docs/images/Screenshot%202022-10-28%20at%2007.54.55.png)


## Building

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

### Linux

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel fontconfig-devel`

