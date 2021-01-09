use ffmpeg_wrapper::mp4;
use image::{DynamicImage, GenericImage, GenericImageView};

fn main() {
    let path = "./rustacean-orig-noshadow.png";
    let ferris = image::open(path).expect("failed to open the file");
    // オリジナルは大きいので縮小する
    let ferris = ferris.resize_exact(256, 256, image::imageops::FilterType::Lanczos3);

    // 256 x 256
    let (width, height) = ferris.dimensions();
    let framerate = 64;

    // エンコーダの開始
    let mut mp4_encoder = mp4::Encoder::new("output/output.mp4", width, height, framerate).unwrap();

    // 作業用の画像
    let mut shifted = DynamicImage::new_rgba8(width, height);

    for i in 0..256 {
        // ferris君をずらしながらshiftedにコピー
        for (x, y, pixel) in ferris.pixels() {
            shifted.put_pixel((x + i) % width, y, pixel);
        }
        // shiftedをエンコード
        mp4_encoder.encode(&shifted).unwrap();
    }

    // エンコーダを終了
    mp4_encoder.close().unwrap();
}
