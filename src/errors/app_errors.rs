#[derive(Debug)]
#[allow(unused)]
pub enum AppError {
    AdHocErr(String),
    IoError(String),
}
impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::IoError(value.to_string())
    }
}
