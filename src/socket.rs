use crate::errors::IpcServerErr;
#[cfg(not(target_os = "windows"))]
use tokio::net::{UnixListener, UnixStream};
//https://docs.rs/tokio/latest/tokio/net/struct.UnixStream.html

#[cfg(not(target_os = "windows"))]
pub struct IpcServer {
    wrapped_server: tokio::net::UnixListener,
}
#[cfg(not(target_os = "windows"))]
impl IpcServer {
    pub async fn new() -> Result<Self, IpcServerErr> {
        if !std::fs::exists("/run/tp-daemon").map_err(|e| IpcServerErr::IoErr(e))? {
            std::fs::create_dir("/run/tp-daemon").map_err(|e| IpcServerErr::IoErr(e))?;
        }
        let sock = UnixListener::bind(format!("/run/tp-daemon/tp.socket"))
            .map_err(|e| IpcServerErr::OpenSocketErr(e.to_string()))?;
        Ok(Self {
            wrapped_server: sock,
        })
    }
    pub async fn next_connection(
        &mut self,
    ) -> Result<(UnixStream, tokio::net::unix::SocketAddr), IpcServerErr> {
        Ok(self
            .wrapped_server
            .accept()
            .await
            .map_err(|e| IpcServerErr::OpenSocketErr(e.to_string()))?)
    }
}

#[cfg(target_os = "windows")]
use tokio::net::windows::NamedPipe;
//https://docs.rs/tokio/latest/tokio/net/windows/named_pipe/index.html

#[cfg(target_os = "windows")]
pub struct IpcServer {
    wrapped: Socket,
}

#[cfg(target_os = "windows")]
impl IpcServer {
    pub async fn new() -> Result<Self, IpcServerErr> {
        //Need to write this on win fuck
    }
}
