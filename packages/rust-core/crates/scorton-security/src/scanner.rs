use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::{Result, Context};
use tokio::net::TcpStream;
use tokio::time::timeout;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerConfig {
    pub timeout: Duration,
    pub max_concurrent: usize,
    pub rate_limit: Option<Duration>,
    pub retry_count: u32,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_concurrent: 100,
            rate_limit: Some(Duration::from_millis(100)),
            retry_count: 3,
        }
    }
}

pub struct ScannerOrchestrator {
    config: ScannerConfig,
}

impl ScannerOrchestrator {
    pub fn new(config: ScannerConfig) -> Self {
        Self { config }
    }

    pub async fn run_comprehensive_scan(&self, target: &str) -> Result<HashMap<String, serde_json::Value>> {
        let mut results = HashMap::new();
        
        // Port scan
        if let Ok(port_results) = self.scan_common_ports(target).await {
            results.insert("port_scan".to_string(), serde_json::to_value(port_results)?);
        }

        // SSL scan
        if let Ok(ssl_result) = self.scan_ssl(target).await {
            results.insert("ssl_scan".to_string(), serde_json::to_value(ssl_result)?);
        }

        // DNS enumeration
        if let Ok(dns_results) = self.enumerate_dns(target).await {
            results.insert("dns_enum".to_string(), serde_json::to_value(dns_results)?);
        }

        // Security headers
        if let Ok(headers) = self.check_security_headers(target).await {
            results.insert("security_headers".to_string(), serde_json::to_value(headers)?);
        }

        Ok(results)
    }

    async fn scan_common_ports(&self, target: &str) -> Result<Vec<crate::PortScanResult>> {
        let common_ports = vec![
            21, 22, 23, 25, 53, 80, 110, 135, 139, 143, 443, 993, 995, 1723, 3389, 5432, 3306, 6379, 27017
        ];
        
        let scanner = crate::SecurityScanner::new(self.config.timeout, self.config.max_concurrent);
        scanner.port_scan(target, &common_ports).await
    }

    async fn scan_ssl(&self, target: &str) -> Result<crate::SSLCertificate> {
        let scanner = crate::SecurityScanner::new(self.config.timeout, self.config.max_concurrent);
        scanner.ssl_scan(target, 443).await
    }

    async fn enumerate_dns(&self, target: &str) -> Result<Vec<crate::DNSRecord>> {
        let scanner = crate::SecurityScanner::new(self.config.timeout, self.config.max_concurrent);
        scanner.dns_enum(target).await
    }

    async fn check_security_headers(&self, target: &str) -> Result<crate::SecurityHeaders> {
        let scanner = crate::SecurityScanner::new(self.config.timeout, self.config.max_concurrent);
        scanner.check_headers(target).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scanner_orchestrator_creation() {
        let config = ScannerConfig::default();
        let orchestrator = ScannerOrchestrator::new(config);
        assert_eq!(orchestrator.config.max_concurrent, 100);
    }
}
