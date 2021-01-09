use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("None error")]
    Optional,
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Image error")]
    Image(#[from] image::ImageError),
}
