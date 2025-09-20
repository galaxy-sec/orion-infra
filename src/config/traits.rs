use log::warn;
use orion_conf::error::OrionConfResult;

/// 配置标准操作接口
pub trait ConfigLifecycle {
    /// 尝试加载配置文件（安全方法）
    fn try_load(path: &str) -> Option<Self>
    where
        Self: Sized,
    {
        if std::path::Path::new(path).exists() {
            match Self::load(path) {
                Ok(conf) => Some(conf),
                Err(e) => {
                    warn!("load conf error: {e}");
                    None
                }
            }
        } else {
            None
        }
    }

    /// 强制加载配置文件
    fn load(path: &str) -> OrionConfResult<Self>
    where
        Self: Sized;

    /// 初始化配置文件
    fn init(&self, path: &str) -> OrionConfResult<()>
    where
        Self: Sized;

    /// 安全清理配置文件
    fn safe_clean(path: &str) -> OrionConfResult<()>;
    fn save(&self, path: &str) -> OrionConfResult<()>;
}
