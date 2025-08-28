use anyhow::{Error, Result, bail};
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};

// 默认位置, 可以被环境变量覆盖
pub const DEF_APP_YAML_PATRH: &'static str = "app.yaml";

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    // pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig, // 形成嵌套结构
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // 几个选项: 优化先exists, 再打开
        let ret: Result<AppConfig, serde_yaml::Error> = match (File::open("chat.yaml"), File::open("/etc/config/chat.yaml"), env::var("CHAT_CONFIG")) {
            (Ok(reader), _, _) => serde_yaml::from_reader::<_, AppConfig>(reader),
            (_, Ok(reader), _) => serde_yaml::from_reader::<_, AppConfig>(reader),
            (_, _, Ok(path)) => serde_yaml::from_reader::<_, AppConfig>(File::open(path)?),
            _ => bail!("Config file not found!"),
        };

        let _config: AppConfig = Self {
            server: ServerConfig { port: 6688 },
        };
        Ok(ret?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
