use crate::entity::{PolicyBuilder, PolicyResp};
use crate::error::OssError;
use crate::metadata::ObjectMetadata;
use crate::oss::{OSSInfo, API, OSS};
use crate::request::{RequestBuilder, RequestType};
use crate::util::read_file;
use crate::{debug, util};
use hmac::Hmac;
use reqwest::header::CONTENT_LENGTH;
use sha1::digest::Mac;

impl OSS {
    pub async fn get_object<S: AsRef<str>>(&self, key: S, build: RequestBuilder) -> Result<Vec<u8>, OssError> {
        let key = self.format_key(key);
        let (url, headers) = self.build_request(key.as_str(), build).map_err(|e| OssError::Err(format!("build request error: {}", e)))?;
        debug!("oss logget object url: {} headers: {:?}", url, headers);
        let client = reqwest::Client::new();
        let response = client.get(url).headers(headers).send().await?;
        return if response.status().is_success() {
            let result = response.bytes().await?;
            Ok(result.to_vec())
        } else {
            let status = response.status();
            let result = response.text().await?;
            debug!("oss log: get object status: {} error: {}", status, result);
            Err(OssError::Err(format!("get object status: {} error: {}", status, result)))
        };
    }

    pub fn get_upload_object_policy(&self, build: PolicyBuilder) -> Result<PolicyResp, OssError> {
        let date = chrono::Local::now().naive_local() + chrono::Duration::seconds(build.expire);
        let date_str = date.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
        let mut json_data = r#"
		{
			"expiration": "{time}",
			"conditions": [
				{"bucket": "{bucket}"},
				["content-length-range", 1, {size}],
				["eq", "$success_action_status", "{success_action_status}"],
				["starts-with", "$key", "{prefix}"],
				["in", "$content-type", ["{content_type}"]]
			]
		}
		"#
        .to_string();
		let success_action_status = 200;
		json_data = json_data.replacen("{time}", &date_str, 1);
		json_data = json_data.replacen("{bucket}", &self.bucket(), 1);
        // limit 1GB bytes
        json_data = json_data.replacen("{size}", &build.max_upload_size.to_string(), 1);
        // success status
        json_data = json_data.replacen(
            "{success_action_status}",
            success_action_status.to_string().as_str(),
            1
        );
        json_data = json_data.replacen("{prefix}", &build.upload_dir, 1); // 只允许上传到哪个目录上
        // text file
        json_data = json_data.replacen("{content_type}", &build.content_type, 1);
        // 只允许上传哪个类型的文件
        debug!("oss log: policy json: {}", json_data);
        let base64_policy = util::base64_encode(json_data.as_bytes());
        let mut hasher: Hmac<sha1::Sha1> = Hmac::new_from_slice(self.key_secret().as_bytes())
            .map_err(|_| OssError::Err("Hmac new from slice error".to_string()))?;
        hasher.update(base64_policy.as_bytes());
        let signature = util::base64_encode(&hasher.finalize().into_bytes());
        Ok(PolicyResp {
            access_id: self.key_id().to_string(),
            host: format!("https://{}.{}", self.bucket(), self.endpoint()),
            policy: base64_policy,
            signature,
            success_action_status
        })
    }

    /// 上传文件（本地文件）
    pub async fn put_object_from_file<S: AsRef<str>>(
        &self,
        key: S,
        file_path: S,
        build: RequestBuilder,
    ) -> Result<(), OssError> {
        let buffer = read_file(file_path)?;
        let mut build = build.clone();
        build.method = RequestType::Put;
        let key = self.format_key(key);
        let (url, mut headers) = self
            .build_request(key.as_str(), build)
            .map_err(|e| OssError::Err(format!("build request error: {}", e)))?;
        if buffer.len() == 0 {
            headers.append(CONTENT_LENGTH, "0".parse().unwrap());
        }
        debug!("oss log: put object from file: {} headers: {:?}", url, headers);
        let client = reqwest::Client::new();
        let response = client.put(url).headers(headers).body(buffer).send().await?;
        return if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let result = response.text().await?;
            debug!("oss log: get object staus: {} error: {}", status, result);
            Err(OssError::Err(format!(
                "get object status: {} error: {}",
                status, result
            )))
        };
    }

    /// 上传文件（内存）
    pub async fn put_object_from_buffer<S: AsRef<str>>(
        &self,
        key: S,
        buffer: &[u8],
        build: RequestBuilder,
    ) -> Result<(), OssError> {
        let mut build = build.clone();
        build.method = RequestType::Put;
        let key = self.format_key(key);
        let (url, headers) = self
            .build_request(key.as_str(), build)
            .map_err(|e| OssError::Err(format!("build request error: {}", e)))?;
        debug!("oss log: put object from file: {} headers: {:?}", url, headers);
        let client = reqwest::Client::new();
        let response = client
            .put(url)
            .headers(headers)
            .body(buffer.to_owned())
            .send()
            .await?;
        return if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let result = response.text().await?;
            debug!("oss log: get object status: {} error: {}", status, result);
            Err(OssError::Err(format!(
                "get object status: {} error: {}",
                status, result
            )))
        }
    }

    /// 删除文件
    pub async fn delete_object<S: AsRef<str>>(
        &self,
        key: S,
        build: RequestBuilder,
    ) -> Result<(), OssError> {
        let mut build = build.clone();
        build.method = RequestType::Delete;
        let key = self.format_key(key);
        let (url, headers) = self
            .build_request(key.as_str(), build)
            .map_err(|e| OssError::Err(format!("build request error: {}", e)))?;
        debug!("oss log: delete object from file: {} headers: {:?}", url, headers);
        let client = reqwest::Client::new();
        let response = client.delete(url).headers(headers).send().await?;
        return if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let result = response.text().await?;
            debug!("oss log: delete object status: {} error: {}", status, result);
            Err(OssError::Err(format!(
                "delete object  status: {} error: {}",
                status, result
            )))
        };
    }

    pub async fn get_object_metadata<S: AsRef<str>>(&self, key: S, build: RequestBuilder) -> Result<ObjectMetadata, OssError> {
        let mut build = build.clone();
        build.method = RequestType::Head;
        let key = self.format_key(key);
        let (url, headers) = self.build_request(key.as_str(), build)
            .map_err(|e| OssError::Err(format!("build request error: {}", e)))?;
        debug!("put object from file: {} headers: {:?}", url, headers);
        let client = reqwest::Client::new();
        let response = client.head(url)
            .headers(headers)
            .send()
            .await?;
        return if response.status().is_success() {
            let metadata = ObjectMetadata::new(response.headers());
            Ok(metadata)
        } else {
            let status = response.status();
            let result = response.text().await?;
            debug!("get object status: {} error: {}", status, result);
            Err(OssError::Err(format!("get object status: {} error: {}", status, result)))
        };
    }
}
