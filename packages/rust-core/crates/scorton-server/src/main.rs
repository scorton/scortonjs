use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanRequest {
    pub tool: String,
    pub target: String,
    pub options: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequest {
    pub framework: String,
    pub target: String,
    pub options: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreRequest {
    pub target: String,
    pub options: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

// Simple handlers without complex middleware
async fn scan_handler(req: web::Json<ScanRequest>) -> Result<HttpResponse> {
    let scanner = scorton_security::SecurityScanner::default();

    match req.tool.as_str() {
        "port_scan" => {
            let ports = vec![22, 80, 443, 8080, 8443];
            match scanner.port_scan(&req.target, &ports).await {
                Ok(results) => {
                    let response = ApiResponse::success(json!({
                        "tool": req.tool,
                        "target": req.target,
                        "results": results,
                        "status": "completed"
                    }));
                    Ok(HttpResponse::Ok().json(response))
                }
                Err(e) => {
                    let response = ApiResponse::<()>::error(format!("Port scan failed: {}", e));
                    Ok(HttpResponse::InternalServerError().json(response))
                }
            }
        }
        "ssl_scan" => match scanner.ssl_scan(&req.target, 443).await {
            Ok(cert) => {
                let response = ApiResponse::success(json!({
                    "tool": req.tool,
                    "target": req.target,
                    "certificate": cert,
                    "status": "completed"
                }));
                Ok(HttpResponse::Ok().json(response))
            }
            Err(e) => {
                let response = ApiResponse::<()>::error(format!("SSL scan failed: {}", e));
                Ok(HttpResponse::InternalServerError().json(response))
            }
        },
        "dns_enum" => match scanner.dns_enum(&req.target).await {
            Ok(records) => {
                let response = ApiResponse::success(json!({
                    "tool": req.tool,
                    "target": req.target,
                    "records": records,
                    "status": "completed"
                }));
                Ok(HttpResponse::Ok().json(response))
            }
            Err(e) => {
                let response = ApiResponse::<()>::error(format!("DNS enumeration failed: {}", e));
                Ok(HttpResponse::InternalServerError().json(response))
            }
        },
        "headers_check" => {
            let url = if req.target.starts_with("http") {
                req.target.clone()
            } else {
                format!("https://{}", req.target)
            };

            match scanner.check_headers(&url).await {
                Ok(headers) => {
                    let response = ApiResponse::success(json!({
                        "tool": req.tool,
                        "target": req.target,
                        "headers": headers,
                        "status": "completed"
                    }));
                    Ok(HttpResponse::Ok().json(response))
                }
                Err(e) => {
                    let response = ApiResponse::<()>::error(format!("Headers check failed: {}", e));
                    Ok(HttpResponse::InternalServerError().json(response))
                }
            }
        }
        _ => {
            let response = ApiResponse::<()>::error(format!("Unknown tool: {}", req.tool));
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

async fn compliance_handler(req: web::Json<ComplianceRequest>) -> Result<HttpResponse> {
    let (dora_enabled, nis2_enabled) = match req.framework.as_str() {
        "dora" => (true, false),
        "nis2" => (false, true),
        "both" => (true, true),
        _ => {
            let response =
                ApiResponse::<()>::error(format!("Unknown framework: {}", req.framework));
            return Ok(HttpResponse::BadRequest().json(response));
        }
    };

    let config = scorton_compliance::ComplianceConfig {
        dora_enabled,
        nis2_enabled,
        ..Default::default()
    };
    let engine = scorton_compliance::ComplianceEngine::new(config);

    match engine.run_compliance_assessment(&req.target).await {
        Ok(result) => {
            let response = ApiResponse::success(json!({
                "framework": req.framework,
                "target": req.target,
                "assessment": result,
                "status": "completed"
            }));
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<()>::error(format!("Compliance assessment failed: {}", e));
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

async fn score_handler(req: web::Json<ScoreRequest>) -> Result<HttpResponse> {
    // Placeholder implementation for cyber score calculation
    let scores = calculate_cyber_score(&req.target).await;

    let score_data = json!({
        "target": req.target,
        "technical": scores.get("technical").copied().unwrap_or(0.0),
        "behavioral": scores.get("behavioral").copied().unwrap_or(0.0),
        "organizational": scores.get("organizational").copied().unwrap_or(0.0),
        "overall": scores.get("overall").copied().unwrap_or(0.0),
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let response = ApiResponse::success(score_data);
    Ok(HttpResponse::Ok().json(response))
}

async fn calculate_cyber_score(_target: &str) -> HashMap<String, f64> {
    let technical = 0.75;
    let behavioral = 0.65;
    let organizational = 0.80;
    let overall = (technical + behavioral + organizational) / 3.0;

    let mut scores = HashMap::new();
    scores.insert("technical".to_string(), technical);
    scores.insert("behavioral".to_string(), behavioral);
    scores.insert("organizational".to_string(), organizational);
    scores.insert("overall".to_string(), overall);
    scores
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting ScortonJS Rust API Server on 127.0.0.1:8001");

    HttpServer::new(|| {
        App::new().wrap(Logger::default()).service(
            web::scope("/api")
                .route("/scan", web::post().to(scan_handler))
                .route("/compliance", web::post().to(compliance_handler))
                .route("/score", web::post().to(score_handler)),
        )
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await
}
