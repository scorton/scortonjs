use napi_derive::napi;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[napi]
pub struct ScanResult {
    pub target: String,
    pub tool: String,
    pub status: String,
    pub data: String, // JSON string
    pub duration_ms: f64,
    pub timestamp: String,
}

#[napi]
pub struct CyberScore {
    pub technical: f64,
    pub behavioral: f64,
    pub organizational: f64,
    pub overall: f64,
}

#[napi]
pub struct DORAResult {
    pub ict_risk_score: f64,
    pub incident_response_time_hours: f64,
    pub resilience_score: f64,
    pub compliance_status: String,
    pub recommendations: Vec<String>,
}

#[napi]
pub struct NIS2Result {
    pub risk_level: String,
    pub incident_reporting_time_hours: f64,
    pub business_continuity_score: f64,
    pub supply_chain_score: f64,
    pub compliance_status: String,
    pub recommendations: Vec<String>,
}

#[napi]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
    pub jwt_secret: String,
}

#[napi]
impl ScanResult {
    #[napi(constructor)]
    pub fn new(target: String, tool: String, status: String, data: String, duration_ms: f64, timestamp: String) -> Self {
        Self {
            target,
            tool,
            status,
            data,
            duration_ms,
            timestamp,
        }
    }
}

#[napi]
impl CyberScore {
    #[napi(constructor)]
    pub fn new(technical: f64, behavioral: f64, organizational: f64, overall: f64) -> Self {
        Self {
            technical,
            behavioral,
            organizational,
            overall,
        }
    }
}

#[napi]
impl DORAResult {
    #[napi(constructor)]
    pub fn new(
        ict_risk_score: f64,
        incident_response_time_hours: f64,
        resilience_score: f64,
        compliance_status: String,
        recommendations: Vec<String>,
    ) -> Self {
        Self {
            ict_risk_score,
            incident_response_time_hours,
            resilience_score,
            compliance_status,
            recommendations,
        }
    }
}

#[napi]
impl NIS2Result {
    #[napi(constructor)]
    pub fn new(
        risk_level: String,
        incident_reporting_time_hours: f64,
        business_continuity_score: f64,
        supply_chain_score: f64,
        compliance_status: String,
        recommendations: Vec<String>,
    ) -> Self {
        Self {
            risk_level,
            incident_reporting_time_hours,
            business_continuity_score,
            supply_chain_score,
            compliance_status,
            recommendations,
        }
    }
}

#[napi]
impl ServerConfig {
    #[napi(constructor)]
    pub fn new(host: String, port: u32, jwt_secret: String) -> Self {
        Self {
            host,
            port,
            jwt_secret,
        }
    }
}

#[napi]
pub async fn run_security_scan(tool: String, target: String) -> napi::Result<ScanResult> {
    let start_time = std::time::Instant::now();
    
    let scanner = scorton_security::SecurityScanner::default();
    
    let result = match tool.as_str() {
        "port_scan" => {
            let ports = vec![21, 22, 23, 25, 53, 80, 110, 135, 139, 143, 443, 993, 995, 1723, 3389];
            match scanner.port_scan(&target, &ports).await {
                Ok(results) => {
                    let data = serde_json::to_string(&results).map_err(|e| napi::Error::from_reason(e.to_string()))?;
                    ScanResult::new(
                        target,
                        tool,
                        "success".to_string(),
                        data,
                        start_time.elapsed().as_millis() as f64,
                        chrono::Utc::now().to_rfc3339(),
                    )
                }
                Err(e) => ScanResult::new(
                    target,
                    tool,
                    "error".to_string(),
                    format!("{{\"error\": \"{}\"}}", e),
                    start_time.elapsed().as_millis() as f64,
                    chrono::Utc::now().to_rfc3339(),
                ),
            }
        }
        "ssl_scan" => {
            match scanner.ssl_scan(&target, 443).await {
                Ok(cert) => {
                    let data = serde_json::to_string(&cert).map_err(|e| napi::Error::from_reason(e.to_string()))?;
                    ScanResult::new(
                        target,
                        tool,
                        "success".to_string(),
                        data,
                        start_time.elapsed().as_millis() as f64,
                        chrono::Utc::now().to_rfc3339(),
                    )
                }
                Err(e) => ScanResult::new(
                    target,
                    tool,
                    "error".to_string(),
                    format!("{{\"error\": \"{}\"}}", e),
                    start_time.elapsed().as_millis() as f64,
                    chrono::Utc::now().to_rfc3339(),
                ),
            }
        }
        "dns_enum" => {
            match scanner.dns_enum(&target).await {
                Ok(records) => {
                    let data = serde_json::to_string(&records).map_err(|e| napi::Error::from_reason(e.to_string()))?;
                    ScanResult::new(
                        target,
                        tool,
                        "success".to_string(),
                        data,
                        start_time.elapsed().as_millis() as f64,
                        chrono::Utc::now().to_rfc3339(),
                    )
                }
                Err(e) => ScanResult::new(
                    target,
                    tool,
                    "error".to_string(),
                    format!("{{\"error\": \"{}\"}}", e),
                    start_time.elapsed().as_millis() as f64,
                    chrono::Utc::now().to_rfc3339(),
                ),
            }
        }
        "headers_check" => {
            match scanner.check_headers(&target).await {
                Ok(headers) => {
                    let data = serde_json::to_string(&headers).map_err(|e| napi::Error::from_reason(e.to_string()))?;
                    ScanResult::new(
                        target,
                        tool,
                        "success".to_string(),
                        data,
                        start_time.elapsed().as_millis() as f64,
                        chrono::Utc::now().to_rfc3339(),
                    )
                }
                Err(e) => ScanResult::new(
                    target,
                    tool,
                    "error".to_string(),
                    format!("{{\"error\": \"{}\"}}", e),
                    start_time.elapsed().as_millis() as f64,
                    chrono::Utc::now().to_rfc3339(),
                ),
            }
        }
        _ => ScanResult::new(
            target,
            tool.clone(),
            "error".to_string(),
            format!("{{\"error\": \"Unknown tool: {}\"}}", tool),
            start_time.elapsed().as_millis() as f64,
            chrono::Utc::now().to_rfc3339(),
        ),
    };
    
    Ok(result)
}

#[napi]
pub async fn calculate_cyber_score(target: String) -> napi::Result<CyberScore> {
    // Placeholder implementation for cyber score calculation
    // In a real implementation, this would analyze various security factors
    let technical = 0.75;
    let behavioral = 0.65;
    let organizational = 0.80;
    let overall = (technical + behavioral + organizational) / 3.0;
    
    Ok(CyberScore::new(technical, behavioral, organizational, overall))
}

#[napi]
pub async fn assess_dora_compliance(target: String) -> napi::Result<DORAResult> {
    match scorton_compliance::dora::assess_dora_compliance(&target).await {
        Ok(assessment) => Ok(DORAResult::new(
            assessment.ict_risk_score,
            assessment.incident_response_time.as_secs_f64() / 3600.0, // Convert to hours
            assessment.resilience_score,
            format!("{:?}", assessment.compliance_status),
            assessment.recommendations,
        )),
        Err(e) => Err(napi::Error::from_reason(e.to_string())),
    }
}

#[napi]
pub async fn assess_nis2_compliance(target: String) -> napi::Result<NIS2Result> {
    match scorton_compliance::nis2::assess_nis2_compliance(&target).await {
        Ok(assessment) => Ok(NIS2Result::new(
            format!("{:?}", assessment.risk_level),
            assessment.incident_handling.reporting_time.as_secs_f64() / 3600.0, // Convert to hours
            assessment.supply_chain_security.overall_score,
            assessment.supply_chain_security.overall_score,
            format!("{:?}", assessment.compliance_status),
            assessment.recommendations,
        )),
        Err(e) => Err(napi::Error::from_reason(e.to_string())),
    }
}

#[napi]
pub async fn run_comprehensive_scan(target: String) -> napi::Result<ScanResult> {
    let start_time = std::time::Instant::now();
    
    let orchestrator = scorton_security::scanner::ScannerOrchestrator::new(
        scorton_security::scanner::ScannerConfig::default()
    );
    
    match orchestrator.run_comprehensive_scan(&target).await {
        Ok(results) => {
            let data = serde_json::to_string(&results).map_err(|e| napi::Error::from_reason(e.to_string()))?;
            Ok(ScanResult::new(
                target,
                "comprehensive".to_string(),
                "success".to_string(),
                data,
                start_time.elapsed().as_millis() as f64,
                chrono::Utc::now().to_rfc3339(),
            ))
        }
        Err(e) => Ok(ScanResult::new(
            target,
            "comprehensive".to_string(),
            "error".to_string(),
            format!("{{\"error\": \"{}\"}}", e),
            start_time.elapsed().as_millis() as f64,
            chrono::Utc::now().to_rfc3339(),
        )),
    }
}

#[napi]
pub fn start_rust_server(config: String) -> napi::Result<()> {
    // This is a placeholder implementation
    // In a real implementation, this would start the Actix-web server
    println!("Starting Rust server with config: {}", config);
    Ok(())
}

#[napi]
pub fn get_rust_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[napi]
pub fn get_supported_tools() -> Vec<String> {
    vec![
        "port_scan".to_string(),
        "ssl_scan".to_string(),
        "dns_enum".to_string(),
        "headers_check".to_string(),
        "comprehensive".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_security_scan() {
        let result = run_security_scan("port_scan".to_string(), "127.0.0.1".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_calculate_cyber_score() {
        let result = calculate_cyber_score("example.com".to_string()).await;
        assert!(result.is_ok());
    }
}
