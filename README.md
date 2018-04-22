# hello264-rust

Jamming on [The World's Smallest h264 Encoder](https://cardinalpeak.com/blog/worlds-smallest-h-264-encoder/)

`cargo build` and 

`cargo run path/to/input/video path/to/output/video`

Make another raw data stream to test: 

`ffmpeg -f lavfi -i mandelbrot=size=100x100:rate=25 -s sqcif -pix_fmt yuv420p -t 20 test.yuv`

Mux data stream into video wrapper:

`ffmpeg -f h264 -i foo.264 -vcodec copy test.mkv`