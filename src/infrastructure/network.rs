use anyhow::Result;
use std::time::Duration;

/// 网络配置
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub proxy: Option<ProxyConfig>,
    pub timeout: TimeoutConfig,
    pub retry: RetryConfig,
}

/// 代理配置
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub http: Option<String>,
    pub https: Option<String>,
    pub no_proxy: Vec<String>,
}

/// 超时配置
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    pub connect: Duration,
    pub read: Duration,
    pub write: Duration,
}

/// 重试配置
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_factor: f64,
}

/// HTTP响应
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: std::collections::HashMap<String, String>,
    pub body: String,
}

/// 网络客户端
pub struct NetworkClient {
    config: NetworkConfig,
    client: reqwest::Client,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            proxy: None,
            timeout: TimeoutConfig::default(),
            retry: RetryConfig::default(),
        }
    }
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            connect: Duration::from_secs(30),
            read: Duration::from_secs(300),
            write: Duration::from_secs(300),
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
            backoff_factor: 2.0,
        }
    }
}

impl NetworkClient {
    /// 创建新的网络客户端
    pub fn new(config: NetworkConfig) -> Result<Self> {
        // TODO: 实现网络客户端初始化
        // 1. 配置HTTP客户端
        // 2. 设置代理
        // 3. 配置超时
        // 4. 设置用户代理

        let mut client_builder = reqwest::Client::builder()
            .timeout(config.timeout.read)
            .connect_timeout(config.timeout.connect);

        // 配置代理
        if let Some(ref proxy_config) = config.proxy {
            if let Some(ref http_proxy) = proxy_config.http {
                if let Ok(proxy) = reqwest::Proxy::http(http_proxy) {
                    client_builder = client_builder.proxy(proxy);
                }
            }
            if let Some(ref https_proxy) = proxy_config.https {
                if let Ok(proxy) = reqwest::Proxy::https(https_proxy) {
                    client_builder = client_builder.proxy(proxy);
                }
            }
        }

        let client = client_builder.build()?;

        Ok(Self { config, client })
    }

    /// 使用默认配置创建
    pub fn with_default() -> Result<Self> {
        Self::new(NetworkConfig::default())
    }

    /// 发送GET请求
    pub async fn get(&self, url: &str) -> Result<HttpResponse> {
        tracing::debug!("发送GET请求: {}", url);

        let response = self.client.get(url).send().await?;

        Ok(self.convert_response(response).await?)
    }

    /// 发送POST请求
    pub async fn post(&self, url: &str, body: &str) -> Result<HttpResponse> {
        tracing::debug!("发送POST请求: {}", url);

        let response = self.client.post(url).body(body.to_string()).send().await?;

        Ok(self.convert_response(response).await?)
    }

    /// 发送带重试的GET请求
    pub async fn get_with_retry(&self, url: &str) -> Result<HttpResponse> {
        tracing::debug!("发送带重试的GET请求: {}", url);

        let mut attempts = 0;
        let mut delay = self.config.retry.initial_delay;

        loop {
            attempts += 1;

            match self.get(url).await {
                Ok(response) => return Ok(response),
                Err(e) if attempts >= self.config.retry.max_attempts => {
                    return Err(anyhow::anyhow!("请求失败，已达到最大重试次数: {}", e));
                }
                Err(e) => {
                    tracing::warn!("请求失败，将重试 (attempts: {}): {}", attempts, e);
                    tokio::time::sleep(delay).await;
                    delay = std::cmp::min(
                        Duration::from_millis(
                            (delay.as_millis() as f64 * self.config.retry.backoff_factor) as u64,
                        ),
                        self.config.retry.max_delay,
                    );
                }
            }
        }
    }

    /// 下载文件
    pub async fn download_file(&self, url: &str, file_path: &std::path::Path) -> Result<()> {
        tracing::info!("下载文件: {} -> {:?}", url, file_path);

        let response = self.client.get(url).send().await?;
        let bytes = response.bytes().await?;

        tokio::fs::write(file_path, &bytes).await?;

        Ok(())
    }

    /// 上传文件
    pub async fn upload_file(
        &self,
        url: &str,
        file_path: &std::path::Path,
        field_name: &str,
    ) -> Result<HttpResponse> {
        tracing::info!("上传文件: {:?} -> {}", file_path, url);

        let file_content = tokio::fs::read(file_path).await?;
        let file_name = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("file");

        let part = reqwest::multipart::Part::bytes(file_content).file_name(file_name.to_string());
        let form = reqwest::multipart::Form::new().part(field_name.to_string(), part);

        let response = self.client.post(url).multipart(form).send().await?;

        Ok(self.convert_response(response).await?)
    }

    /// 发送Webhook通知
    pub async fn send_webhook(
        &self,
        url: &str,
        payload: &str,
        headers: Option<&std::collections::HashMap<String, String>>,
    ) -> Result<HttpResponse> {
        tracing::debug!("发送Webhook: {}", url);

        let mut request = self.client.post(url).body(payload.to_string());

        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        let response = request.send().await?;

        Ok(self.convert_response(response).await?)
    }

    /// 检查URL连通性
    pub async fn check_connectivity(&self, url: &str) -> Result<bool> {
        tracing::debug!("检查URL连通性: {}", url);

        match self.client.head(url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// 获取URL内容大小
    pub async fn get_content_length(&self, url: &str) -> Result<Option<u64>> {
        tracing::debug!("获取内容大小: {}", url);

        let response = self.client.head(url).send().await?;

        let content_length = response
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|value| value.to_str().ok())
            .and_then(|s| s.parse().ok());

        Ok(content_length)
    }

    /// 转换响应对象
    async fn convert_response(&self, response: reqwest::Response) -> Result<HttpResponse> {
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();
        let body = response.text().await?;

        Ok(HttpResponse {
            status,
            headers,
            body,
        })
    }

    /// 测试网络连接
    pub async fn test_connection(&self) -> Result<NetworkTestResult> {
        tracing::debug!("测试网络连接");

        let mut results = Vec::new();

        // 测试常见的公共服务
        let test_urls = vec![
            "https://httpbin.org/get",
            "https://www.google.com",
            "https://github.com",
        ];

        for url in test_urls {
            let start = std::time::Instant::now();
            let result = self.check_connectivity(url).await;
            let duration = start.elapsed();

            results.push(ConnectionTest {
                url: url.to_string(),
                success: result.unwrap_or(false),
                duration,
            });
        }

        let success_count = results.iter().filter(|r| r.success).count();

        Ok(NetworkTestResult {
            overall_success: success_count > 0,
            tests: results,
        })
    }
}

/// 网络测试结果
#[derive(Debug, Clone)]
pub struct NetworkTestResult {
    pub overall_success: bool,
    pub tests: Vec<ConnectionTest>,
}

/// 连接测试
#[derive(Debug, Clone)]
pub struct ConnectionTest {
    pub url: String,
    pub success: bool,
    pub duration: Duration,
}
