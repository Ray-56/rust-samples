use std::collections::HashMap;
use reqwest::header::DATE;
use crate::auth::AuthAPI;
use crate::debug;
use crate::oss::{API, OSS, OSSInfo};
use crate::request::{RequestBuilder, RequestType};

pub trait UrlApi: OSSInfo + API {
  /// 获取签名下载 URL
  /// 
  /// # 使用例子
  /// 
  /// ```
  /// 
  /// ```
  fn sign_download_url<S: AsRef<str>>(&self, key: S, build: &RequestBuilder) -> String;

  /// 获取签名上传 URL
  /// 
  /// # 使用例子
  /// 
  /// ```
  /// 
  /// ``` 
  fn sign_upload_url<S: AsRef<str>>(&self, key: S, build: &RequestBuilder) -> String;
  fn sign_url<S: AsRef<str>>(&self, key: S, build: &RequestBuilder) -> String;
}

impl UrlApi for OSS {
  fn sign_download_url<S: AsRef<str>>(&self, key: S, build: &RequestBuilder) -> String {
      let sign = self.sign_url(key.as_ref(), build);
      if let Some(cdn) = &build.cdn {
        let download_url = format!("{}{}", cdn, sign);
        debug!("download_url: {}", download_url);
        download_url
      } else {
        let schema = if build.https {
          "https://"
        } else {
          "http://"
        };
        let download_url = format!("{}{}.{}{}", schema, self.bucket(), self.endpoint(), sign);
        debug!("download_url: {}", download_url);
        download_url
      }
  }

  fn sign_upload_url<S: AsRef<str>>(&self, key: S, build: &RequestBuilder) -> String {
      let mut build = build.clone();
      build.method = RequestType::Put;
      let sign = self.sign_url(key.as_ref(), &build);
      if let Some(cdn) = &build.cdn {
        let download_url = format!("{}{}", cdn, sign);
        debug!("upload_url: {}", download_url);
        download_url
      } else {
        let schema = if build.https {
          "https://"
        } else {
          "http://"
        };
        let download_url = format!("{}{}.{}{}", schema, self.bucket(), self.endpoint(), sign);
        debug!("upload_url: {}", download_url);
        download_url
      }
  }

  fn sign_url<S: AsRef<str>>(&self, key: S, build: &RequestBuilder) -> String {
      let mut build = build.clone();
      let key = self.format_key(key);
      let expiration = chrono::Local::now() + chrono::Duration::seconds(build.expire);
      build.headers.insert(DATE.to_string(), expiration.timestamp().to_string());
      let signature = self.sign(
        key.as_str(), 
        &build
      );
      debug!("signature: {}", signature);
      let mut query_parameters = HashMap::new();
      query_parameters.insert("Expires".to_string(), expiration.timestamp().to_string());
      query_parameters.insert("OSSAccessKeyId".to_string(), self.key_id().to_string());
      query_parameters.insert("Signature".to_string(), urlencoding::encode(&signature).into_owned());
      build.parameters.iter().for_each(|(k, v)| {
        query_parameters.insert(k.to_string(), urlencoding::encode(v).into_owned());
      });

      let mut params = query_parameters
        .into_iter()
        .filter(|(k, _)| k != "x-oss-ac-source-ip")
        .collect::<Vec<_>>();

      params.sort_by(|a, b| a.0.cmp(&b.0));

      format!(
        "{}?{}",
        self.key_urlencode(key),
        params.into_iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<String>>().join("&")
      )
  }
}