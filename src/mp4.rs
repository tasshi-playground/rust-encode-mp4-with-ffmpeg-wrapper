use crate::error::Error;

use image::{DynamicImage, GenericImageView};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// MP4 encoder.
pub struct Encoder<P: AsRef<Path>> {
    p: P,
    ffmpeg: std::process::Child,
    width: u32,
    height: u32,
    framerate: u32,
}

impl<P: AsRef<Path>> Encoder<P> {
    /// Creates a new MP4 encoder.
    pub fn new(path: P, width: u32, height: u32, framerate: u32) -> Result<Encoder<P>, Error> {
        let name = path.as_ref();

        // ffmpegを実行するためにコマンドを組み立てる
        let command = |width, height, framerate, output| {
            format!(
            "ffmpeg -framerate {framerate} -f rawvideo -pix_fmt rgba -s {width}x{height} -i - -pix_fmt yuv420p -vcodec libx264 -movflags faststart {output:?}",
            width=width, height=height, framerate=framerate, output=output
        )
        };

        // ffmpegを実行
        let ffmpeg = Command::new("/bin/sh")
            .args(&["-c", &command(width, height, framerate, name)])
            .stdin(Stdio::piped())
            .spawn()?;

        // 返り値のEncoder構造体は、実行中のffmpegプロセスのハンドラなどを含む
        Ok(Encoder {
            p: path,
            ffmpeg: ffmpeg,
            width: width,
            height: height,
            framerate: framerate,
        })
    }

    /// Encodes a frame.
    pub fn encode(&mut self, frame: &DynamicImage) -> Result<(), Error> {
        let (width, height) = frame.dimensions();

        // 入力画像のサイズがEncoderに登録されたサイズと異なる場合はエラーを返す
        if (width, height) != (self.width, self.height) {
            Err(Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid image size",
            )))
        } else {
            let stdin = match self.ffmpeg.stdin.as_mut() {
                Some(stdin) => Ok(stdin),
                None => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "cannot start ffmpeg",
                )),
            }?;

            // 標準入力にフレームのピクセルを流し込む
            stdin.write_all(&frame.raw_pixels())?;
            Ok(())
        }
    }

    /// Creates a current MP4 encoder.
    pub fn close(&mut self) -> Result<(), Error> {
        // ここで明示的にstdinをdropする
        drop(&self.ffmpeg.stdin);
        // ffmpeg側の終了を待つ
        self.ffmpeg.wait()?;
        Ok(())
    }
}
