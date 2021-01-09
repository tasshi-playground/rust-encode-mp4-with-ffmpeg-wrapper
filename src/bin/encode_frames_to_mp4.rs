use image::{DynamicImage, GenericImage, GenericImageView};
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let path = "./rustacean-orig-noshadow.png";
    let ferris = image::open(path).expect("failed to open the file");
    // オリジナルは大きいので縮小する
    let ferris = ferris.resize_exact(256, 256, image::imageops::FilterType::Lanczos3);

    let (width, height) = ferris.dimensions(); // 256 x 256

    let frames: Vec<DynamicImage> = (0..256)
        .map(|i| {
            let ferris = ferris.clone();
            let mut output = ferris.clone();
            for (x, y, pixel) in ferris.pixels() {
                output.put_pixel((x + i) % width, y, pixel);
            }
            output
        })
        .collect();
    encode_frames_to_mp4("/output/output.mp4", &frames);
}

fn encode_frames_to_mp4(name: &str, frames: &[DynamicImage]) {
    // ffmpegを実行するためにコマンドを組み立てる
    let command = |width, height, output| {
        format!(
        "ffmpeg -f rawvideo -pix_fmt rgba -s {width}x{height} -i - -pix_fmt yuv420p -vcodec libx264 -movflags faststart {output:?}",
        width=width, height=height, output=output
    )
    };

    let (width, height) = frames.first().unwrap().dimensions();

    // ffmpegを実行
    let mut ffmpeg = Command::new("/bin/sh")
        .args(&["-c", &command(width, height, name)])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    {
        // 標準入力にフレームのピクセルを流し込む
        let stdin = ffmpeg.stdin.as_mut().unwrap();
        for frame in frames {
            stdin.write_all(&frame.raw_pixels()).unwrap();
        }
        // ここでスコープを抜けるのでstdinがdropされる
    }

    // ffmpeg側の終了を待つ
    ffmpeg.wait().unwrap();
}
