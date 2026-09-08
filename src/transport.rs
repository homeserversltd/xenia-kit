//! The socket is authoritative; `listen` is only a courtesy for discovery.
use crate::manifest::Result;
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
};
use tokio::net::TcpListener;

pub struct Transport {
    pub listener: TcpListener,
    pub address: SocketAddr,
    pub courtesy: Option<PathBuf>,
}
impl Transport {
    pub async fn bind(id: &str, declared: Option<&str>) -> Result<Self> {
        Self::bind_under(id, declared, Path::new("/var/lib/xenia")).await
    }
    pub async fn bind_under(id: &str, declared: Option<&str>, root: &Path) -> Result<Self> {
        // A filesystem boundary, not a second manifest schema.
        if id.is_empty()
            || id == "."
            || id == ".."
            || id.bytes().any(|b| !b.is_ascii_alphanumeric() && b != b'-')
        {
            return Err("unsafe cartridge path component".into());
        }
        let endpoint = declared.unwrap_or("127.0.0.1:0");
        let endpoint = endpoint.strip_prefix("http://").unwrap_or(endpoint);
        let address: SocketAddr = endpoint.parse()?;
        if !address.ip().is_loopback() {
            return Err("transport must bind a literal loopback address".into());
        }
        let listener = TcpListener::bind(address).await?;
        let bound = listener.local_addr()?;
        let courtesy = if address.port() == 0 {
            tokio::fs::create_dir_all(root).await?;
            let root = tokio::fs::canonicalize(root).await?;
            let dir = root.join(id);
            match tokio::fs::create_dir(&dir).await {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(e) => return Err(e.into()),
            }
            let metadata = tokio::fs::symlink_metadata(&dir).await?;
            if !metadata.is_dir() || metadata.file_type().is_symlink() {
                return Err("courtesy directory must not be a symlink".into());
            }
            let path = dir.join("listen");
            let temporary = dir.join(format!(".listen-{}", uuid::Uuid::new_v4()));
            use std::io::Write;
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)?;
            file.write_all(format!("http://{bound}\n").as_bytes())?;
            file.sync_all()?;
            if let Err(e) = tokio::fs::rename(&temporary, &path).await {
                let _ = tokio::fs::remove_file(&temporary).await;
                return Err(e.into());
            }
            Some(path)
        } else {
            None
        };
        Ok(Self {
            listener,
            address: bound,
            courtesy,
        })
    }
}
