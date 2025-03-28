use crate::request::Seconds;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResp {
    pub access_id: String,
    pub host: String,
    pub policy: String,
    pub signature: String,
    pub success_action_status: u8,
}

unsafe impl Send for PolicyResp {}

unsafe impl Sync for PolicyResp {}

/// Policy 构建器
#[derive(Debug, Clone)]
pub struct PolicyBuilder {
    pub expire: Seconds,
    pub upload_dir: String,
    pub content_type: String,
    pub max_upload_size: u64,
}

unsafe impl Send for PolicyBuilder {}

unsafe impl Sync for PolicyBuilder {}

impl Default for PolicyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PolicyBuilder {
    pub fn new() -> Self {
        Self {
            expire: 60,
            upload_dir: "".to_string(),
            content_type: "text/plain".to_string(),
            max_upload_size: 100 * 1024 * 1024,
        }
    }
    pub fn with_expire(mut self, expire: Seconds) -> Self {
        self.expire = expire;
        self
    }
    pub fn with_upload_dir<S: AsRef<str>>(mut self, upload_dir: S) -> Self {
        self.upload_dir = upload_dir.as_ref().to_string();
        self
    }
    pub fn with_content_type<S: AsRef<str>>(mut self, content_type: S) -> Self {
        self.content_type = content_type.as_ref().to_string();
        self
    }
    pub fn with_max_upload_size(mut self, max_upload_size: u64) -> Self {
        self.max_upload_size = max_upload_size;
        self
    }
}
