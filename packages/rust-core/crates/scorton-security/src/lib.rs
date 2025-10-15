use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use url::Url;
use anyhow::{Result, Context};

pub mod scanner;
pub mod compliance;
pub mod ssl;
pub mod dns;
pub mod headers;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub target: String,
    pub tool: String,
    pub status: ScanStatus,
    pub data: serde_json::Value,
    pub duration_ms: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanStatus {
    Success,
    Failed(String),
    Timeout,
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortScanResult {
    pub port: u16,
    pub state: PortState,
    pub service: Option<String>,
    pub version: Option<String>,
    pub banner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortState {
    Open,
    Closed,
    Filtered,
    OpenFiltered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSLCertificate {
    pub subject: String,
    pub issuer: String,
    pub valid_from: chrono::DateTime<chrono::Utc>,
    pub valid_until: chrono::DateTime<chrono::Utc>,
    pub signature_algorithm: String,
    pub key_size: u32,
    pub serial_number: String,
    pub san: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSRecord {
    pub record_type: String,
    pub name: String,
    pub value: String,
    pub ttl: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHeaders {
    pub strict_transport_security: Option<String>,
    pub content_security_policy: Option<String>,
    pub x_frame_options: Option<String>,
    pub x_content_type_options: Option<String>,
    pub x_xss_protection: Option<String>,
    pub referrer_policy: Option<String>,
    pub permissions_policy: Option<String>,
}

pub struct SecurityScanner {
    timeout: Duration,
    max_concurrent: usize,
}

impl Default for SecurityScanner {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_concurrent: 100,
        }
    }
}

impl SecurityScanner {
    pub fn new(timeout: Duration, max_concurrent: usize) -> Self {
        Self {
            timeout,
            max_concurrent,
        }
    }

    pub async fn port_scan(&self, target: &str, ports: &[u16]) -> Result<Vec<PortScanResult>> {
        let start_time = std::time::Instant::now();
        let mut results = Vec::new();
        
        // Parse target to IP address
        let ip: IpAddr = if let Ok(parsed_ip) = target.parse() {
            parsed_ip
        } else {
            // Try DNS resolution if not an IP
            let target = target.to_string();
            let result = tokio::task::spawn_blocking(move || {
                std::net::ToSocketAddrs::to_socket_addrs(&format!("{}:80", target))
                    .map(|mut addrs| addrs.next().map(|addr| addr.ip()))
                    .unwrap_or(None)
            }).await.map_err(|_| anyhow::anyhow!("DNS resolution task failed"))?;
            
            match result {
                Some(ip) => ip,
                None => return Err(anyhow::anyhow!("No IP address found for target")),
            }
        };

        // Create semaphore for concurrency control
        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(self.max_concurrent));
        
        // Scan ports concurrently
        let mut tasks = Vec::new();
        let ports = ports.to_vec(); // Convert to owned Vec
        for port in ports {
            let semaphore = semaphore.clone();
            let ip = ip;
            let timeout = self.timeout;
            
            tasks.push(tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                Self::scan_port(ip, port, timeout).await
            }));
        }

        // Collect results
        for task in tasks {
            if let Ok(result) = task.await {
                results.push(result);
            }
        }

        Ok(results)
    }

    async fn scan_port(ip: IpAddr, port: u16, timeout_duration: Duration) -> PortScanResult {
        let socket_addr = SocketAddr::new(ip, port);
        
        match timeout(timeout_duration, TcpStream::connect(&socket_addr)).await {
            Ok(Ok(_)) => PortScanResult {
                port,
                state: PortState::Open,
                service: Self::detect_service(port),
                version: None,
                banner: None,
            },
            Ok(Err(_)) => PortScanResult {
                port,
                state: PortState::Closed,
                service: None,
                version: None,
                banner: None,
            },
            Err(_) => PortScanResult {
                port,
                state: PortState::Filtered,
                service: None,
                version: None,
                banner: None,
            },
        }
    }

    fn detect_service(port: u16) -> Option<String> {
        match port {
            22 => Some("ssh".to_string()),
            23 => Some("telnet".to_string()),
            25 => Some("smtp".to_string()),
            53 => Some("dns".to_string()),
            80 => Some("http".to_string()),
            110 => Some("pop3".to_string()),
            143 => Some("imap".to_string()),
            443 => Some("https".to_string()),
            993 => Some("imaps".to_string()),
            995 => Some("pop3s".to_string()),
            3389 => Some("rdp".to_string()),
            5432 => Some("postgresql".to_string()),
            3306 => Some("mysql".to_string()),
            6379 => Some("redis".to_string()),
            27017 => Some("mongodb".to_string()),
            _ => None,
        }
    }

    pub async fn ssl_scan(&self, target: &str, port: u16) -> Result<SSLCertificate> {
        ssl::analyze_ssl_certificate(target, port).await
    }

    pub async fn dns_enum(&self, domain: &str) -> Result<Vec<DNSRecord>> {
        dns::enumerate_dns_records(domain).await
    }

    pub async fn check_headers(&self, url: &str) -> Result<SecurityHeaders> {
        headers::analyze_security_headers(url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_port_scan_localhost() {
        let scanner = SecurityScanner::default();
        let ports = vec![80, 443, 22];
        
        // This test will only work if we have a local server running
        // For now, just test that the function doesn't panic
        let _result = scanner.port_scan("127.0.0.1", &ports).await;
    }

    #[tokio::test]
    async fn test_service_detection() {
        assert_eq!(SecurityScanner::detect_service(80), Some("http".to_string()));
        assert_eq!(SecurityScanner::detect_service(443), Some("https".to_string()));
        assert_eq!(SecurityScanner::detect_service(9999), None);
    }
}
