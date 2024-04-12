# Jantar's Catgirl Downloader
## What is this?
I made this project to learn a bit of Rust, Slint, and HTTP requests all at the same time.
Also I got inspired by [this catgirl downloader](https://github.com/NyarchLinux/CatgirlDownloader) but didn't like that is was a FlatPak and GTK and was overall unsatisfied how it was realized.
My app gets its images from the same [source](https://nekos.moe).

## Running (linux)
> [!IMPORTANT]
> I am testing this program only on linux at the newest version of cargo and lts kernel at the moment. Windows testing **may** be done in the future.
### Compiling from source
You can compile this project yourself using `cargo build --release` while in root of this project. You need to have **cargo** and **internet connection**.

There is approximately 500 packages to be built so it is understandable you may not want to compile this project yourself. I will provide executables in the future.

## Planned features
- You will be able to view info on the catgirl you are currently looking at
- There will be NSFW switch (on-off-surprise)