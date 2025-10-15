use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::collections::HashMap;

pub async fn analyze_security_headers(url: &str) -> Result<crate::SecurityHeaders> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .context("Failed to create HTTP client")?;

    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to send HTTP request")?;

    let headers = response.headers();
    
    Ok(crate::SecurityHeaders {
        strict_transport_security: headers
            .get("strict-transport-security")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string()),
        content_security_policy: headers
            .get("content-security-policy")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string()),
        x_frame_options: headers
            .get("x-frame-options")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string()),
        x_content_type_options: headers
            .get("x-content-type-options")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string()),
        x_xss_protection: headers
            .get("x-xss-protection")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string()),
        referrer_policy: headers
            .get("referrer-policy")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string()),
        permissions_policy: headers
            .get("permissions-policy")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string()),
    })
}

pub async fn check_security_header_vulnerabilities(url: &str) -> Result<Vec<HeaderVulnerability>> {
    let mut vulnerabilities = Vec::new();
    let headers = analyze_security_headers(url).await?;
    
    // Check for missing HSTS
    if headers.strict_transport_security.is_none() {
        vulnerabilities.push(HeaderVulnerability {
            header: "Strict-Transport-Security".to_string(),
            severity: VulnerabilitySeverity::High,
            description: "Missing HSTS header".to_string(),
            recommendation: "Add Strict-Transport-Security header".to_string(),
        });
    }
    
    // Check for missing CSP
    if headers.content_security_policy.is_none() {
        vulnerabilities.push(HeaderVulnerability {
            header: "Content-Security-Policy".to_string(),
            severity: VulnerabilitySeverity::Medium,
            description: "Missing CSP header".to_string(),
            recommendation: "Add Content-Security-Policy header".to_string(),
        });
    }
    
    // Check for missing X-Frame-Options
    if headers.x_frame_options.is_none() {
        vulnerabilities.push(HeaderVulnerability {
            header: "X-Frame-Options".to_string(),
            severity: VulnerabilitySeverity::Medium,
            description: "Missing X-Frame-Options header".to_string(),
            recommendation: "Add X-Frame-Options header".to_string(),
        });
    }
    
    // Check for missing X-Content-Type-Options
    if headers.x_content_type_options.is_none() {
        vulnerabilities.push(HeaderVulnerability {
            header: "X-Content-Type-Options".to_string(),
            severity: VulnerabilitySeverity::Low,
            description: "Missing X-Content-Type-Options header".to_string(),
            recommendation: "Add X-Content-Type-Options: nosniff".to_string(),
        });
    }
    
    Ok(vulnerabilities)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderVulnerability {
    pub header: String,
    pub severity: VulnerabilitySeverity,
    pub description: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VulnerabilitySeverity {
    Low,
    Medium,
    High,
    Critical,
}

pub fn calculate_security_header_score(headers: &crate::SecurityHeaders) -> f64 {
    let mut score = 0.0;
    let total_checks = 7.0;
    
    if headers.strict_transport_security.is_some() { score += 1.0; }
    if headers.content_security_policy.is_some() { score += 1.0; }
    if headers.x_frame_options.is_some() { score += 1.0; }
    if headers.x_content_type_options.is_some() { score += 1.0; }
    if headers.x_xss_protection.is_some() { score += 1.0; }
    if headers.referrer_policy.is_some() { score += 1.0; }
    if headers.permissions_policy.is_some() { score += 1.0; }
    
    score / total_checks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_headers_analysis() {
        // This test would require a real HTTP endpoint
        // For now, just test that the function doesn't panic
        let _result = analyze_security_headers("https://example.com").await;
    }

    #[test]
    fn test_security_header_score() {
        let headers = crate::SecurityHeaders {
            strict_transport_security: Some("max-age=31536000".to_string()),
            content_security_policy: Some("default-src 'self'".to_string()),
            x_frame_options: Some("DENY".to_string()),
            x_content_type_options: Some("nosniff".to_string()),
            x_xss_protection: Some("1; mode=block".to_string()),
            referrer_policy: Some("strict-origin-when-cross-origin".to_string()),
            permissions_policy: Some("geolocation=()".to_string()),
        };
        
        let score = calculate_security_header_score(&headers);
        assert_eq!(score, 1.0);
    }
}
