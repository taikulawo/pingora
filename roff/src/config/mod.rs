use std::{collections::HashMap, sync::Arc};

use rustls::ServerConfig;
use tokio::sync::RwLock;
pub mod constant;
#[derive(Clone)]
struct ConfigCenter {
    host_config: Arc<RwLock<HashMap<String, SharedHostConfig>>>,
}

type SharedConfigCenter = Arc<ConfigCenter>;

pub struct HostConfig {}

pub type SharedHostConfig = Arc<HostConfig>;
impl ConfigCenter {
    pub async fn get_host_config_for(&self, host: &str) -> Option<SharedHostConfig> {
        let _guard = self.host_config.read().await;
        _guard.get(host).cloned()
    }
    pub async fn get_rustls_config_for(&self, host: &str) -> Option<ServerConfig> {
        self.get_host_config_for(host).await.map(|f|{
            todo!();
        })
    }
}
