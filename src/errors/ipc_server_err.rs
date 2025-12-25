#[derive(Debug)]
pub enum IpcServerErr {
    OpenSocketErr(String),
    IoErr(std::io::Error),
}
impl From<std::io::Error> for IpcServerErr {
    fn from(value: std::io::Error) -> Self {
        IpcServerErr::IoErr(value)
    }
}
