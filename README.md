# rust-encode-mp4-with-ffmpeg-wrapper

Example of encoding mp4 with ffmpeg wrapper

https://qiita.com/tasshi/items/de36d9add14f24317f47

## Sample

| input                                                                 | output                         |
| --------------------------------------------------------------------- | ------------------------------ |
| <image src="rustacean-orig-noshadow.png" alt="input" width="256px" /> | ![output](./output/output.gif) |

## Directory

```
.
├── output
│   ├── output.gif
│   └── output.mp4 # sample output movie
├── src
│   ├── bin
│   │   ├── encode_frames_to_mp4.rs # non-iterative version
│   │   └── main.rs # iterative version.
│   ├── error.rs
│   ├── lib.rs
│   └── mp4.rs # mp4 encoder module
├── target
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
└── rustacean-orig-noshadow.png # sample input image
```
