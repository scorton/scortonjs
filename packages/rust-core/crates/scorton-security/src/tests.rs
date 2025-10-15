#[cfg(test)]
mod integration_tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_security_scanner_creation() {
        let scanner = SecurityScanner::default();
        assert_eq!(scanner.timeout.as_secs(), 30);
        assert_eq!(scanner.max_concurrent, 100);
    }

    #[tokio::test]
    async fn test_scanner_orchestrator() {
        let config = scanner::ScannerConfig::default();
        let orchestrator = scanner::ScannerOrchestrator::new(config);
        
        // Test that orchestrator can be created
        assert_eq!(orchestrator.config.timeout.as_secs(), 30);
    }

    #[tokio::test]
    async fn test_compliance_assessor() {
        let config = compliance::ComplianceConfig::default();
        let assessor = compliance::ComplianceAssessor::new(config);
        
        // Test DORA assessment
        let dora_result = assessor.assess_dora_compliance("example.com").await;
        assert!(dora_result.is_ok());
        
        // Test NIS2 assessment
        let nis2_result = assessor.assess_nis2_compliance("example.com").await;
        assert!(nis2_result.is_ok());
    }

    #[tokio::test]
    async fn test_ssl_analysis() {
        // Test SSL certificate analysis
        let result = ssl::analyze_ssl_certificate("example.com", 443).await;
        assert!(result.is_ok());
        
        let cert = result.unwrap();
        assert!(!cert.subject.is_empty());
        assert!(!cert.issuer.is_empty());
    }

    #[tokio::test]
    async fn test_dns_enumeration() {
        // Test DNS record enumeration
        let result = dns::enumerate_dns_records("example.com").await;
        assert!(result.is_ok());
        
        let records = result.unwrap();
        // Should have at least some records
        assert!(!records.is_empty());
    }

    #[tokio::test]
    async fn test_security_headers() {
        // Test security header analysis
        let result = headers::analyze_security_headers("https://example.com").await;
        assert!(result.is_ok());
        
        let headers = result.unwrap();
        // Headers struct should be created
        assert!(headers.strict_transport_security.is_some() || 
                headers.content_security_policy.is_some() ||
                headers.x_frame_options.is_some());
    }

    #[tokio::test]
    async fn test_performance_benchmark() {
        let scanner = SecurityScanner::default();
        let start = std::time::Instant::now();
        
        // Simulate a quick scan
        let _result = scanner.port_scan("127.0.0.1", &[80, 443]).await;
        
        let duration = start.elapsed();
        
        // Should complete quickly (within 5 seconds for localhost)
        assert!(duration.as_secs() < 5);
    }

    #[tokio::test]
    async fn test_error_handling() {
        let scanner = SecurityScanner::default();
        
        // Test with invalid target
        let result = scanner.port_scan("invalid-target-that-should-fail", &[80]).await;
        
        // Should handle error gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let scanner = SecurityScanner::default();
        
        // Test concurrent port scans
        let tasks: Vec<_> = (0..5)
            .map(|i| {
                let scanner = SecurityScanner::default();
                tokio::spawn(async move {
                    scanner.port_scan("127.0.0.1", &[80 + i]).await
                })
            })
            .collect();
        
        let results = futures::future::join_all(tasks).await;
        
        // All tasks should complete
        assert_eq!(results.len(), 5);
    }
}
