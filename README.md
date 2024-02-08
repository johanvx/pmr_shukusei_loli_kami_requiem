# PMR Shukusei Loli Kami Requiem

**CAUTION:** The method used in this repository consumes a huge amount of disk
space, so make sure you have enough space for it. Check [ascii-art-converter]
for an improved method.

[ascii-art-converter]: https://github.com/johanvx/ascii-art-converter

This is an ASCII Art version of
[粛聖!! ロリ神レクイエム☆ / P丸様。（9さい）【歌ってみた】],
and the output of [play.rs](src/bin/play.rs) is recorded and published as [a
video on Bilibili](https://www.bilibili.com/video/BV1Sz4y1w7Un).

[粛聖!! ロリ神レクイエム☆ / P丸様。（9さい）【歌ってみた】]: https://www.youtube.com/watch?v=sY22ntY9TnQ

## Table of contents

- [Getting necessary data](#getting-necessary-data)
- [How to use](#how-to-use)
- [Why 32 fps in `play.rs`](#why-32-fps-in-play-rs)

## Getting necessary data

Download the thumbnail `thumbnail.webp`:

```bash
yt-dlp --skip-download --write-thumbnail -o thumbnail 'https://www.youtube.com/watch?v=sY22ntY9TnQ'
```

Download the video `pmr.webm`:

```bash
yt-dlp -o pmr 'https://www.youtube.com/watch?v=sY22ntY9TnQ'
```

Extract frames from the video and save them as `frames/pmr-%04d.png`:

```bash
mkdir -p frames
ffmpeg -i pmr.webm -f image2 -vf fps=29.97 frames/pmr-%04d.png
```

**HINT**: use `ffprobe <input_file>` to query the frame rate of the video and
the width/height of the video/thumbnail.

## Usage

Print the rendered thumbnail:

```bash
cargo run --release --bin thumbnail
```

Render the extracted frames and save the output to files:

```bash
mkdir -p output
cargo run --release --bin render
```

Play the 'video' by reading the rendered output:

```bash
cargo run --release --quiet --bin play
```

## Why 32 fps in `play.rs`

It's really difficult (at least for me) to keep the frame rate of the resulting
video at 30 fps. I tried setting `frame_rate` to 30, but the resulting video
was longer than the original video.

Setting `frame_rate` to 32 doesn't mean that the resulting video is a 32-fps
video. In fact, the actual frame rate of the resulting video is very close to
30 fps. With that, the only thing that I need to do is to slightly decelerate
the resulting video in Final Cut Pro.
