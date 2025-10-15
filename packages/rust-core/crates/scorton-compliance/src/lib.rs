use serde::{Deserialize, Serialize};
use std::time::Duration;
use anyhow::Result;

pub mod dora;
pub mod nis2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub target: String,
    pub framework: ComplianceFramework,
    pub assessment_date: chrono::DateTime<chrono::Utc>,
    pub overall_score: f64,
    pub compliance_status: ComplianceStatus,
    pub findings: Vec<Finding>,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceFramework {
    DORA,
    NIS2,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    PartiallyCompliant,
    NonCompliant,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub category: Category,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub effort: Effort,
    pub impact: Impact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Category {
    Technical,
    Organizational,
    Process,
    Legal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effort {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Impact {
    Low,
    Medium,
    High,
}

pub struct ComplianceEngine {
    config: ComplianceConfig,
}

#[derive(Debug, Clone)]
pub struct ComplianceConfig {
    pub dora_enabled: bool,
    pub nis2_enabled: bool,
    pub assessment_depth: AssessmentDepth,
    pub reporting_format: ReportingFormat,
}

#[derive(Debug, Clone)]
pub enum AssessmentDepth {
    Quick,
    Standard,
    Deep,
}

#[derive(Debug, Clone)]
pub enum ReportingFormat {
    Json,
    Markdown,
    Pdf,
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            dora_enabled: true,
            nis2_enabled: true,
            assessment_depth: AssessmentDepth::Standard,
            reporting_format: ReportingFormat::Json,
        }
    }
}

impl ComplianceEngine {
    pub fn new(config: ComplianceConfig) -> Self {
        Self { config }
    }

    pub async fn run_compliance_assessment(&self, target: &str) -> Result<ComplianceReport> {
        let mut findings = Vec::new();
        let mut recommendations = Vec::new();
        let mut overall_score = 0.0;
        let mut score_count = 0;

        // Run DORA assessment if enabled
        if self.config.dora_enabled {
            let dora_result = dora::assess_dora_compliance(target).await?;
            overall_score += dora_result.ict_risk_score;
            score_count += 1;
            
            findings.extend(self.convert_dora_findings(&dora_result));
            recommendations.extend(self.convert_dora_recommendations(&dora_result));
        }

        // Run NIS2 assessment if enabled
        if self.config.nis2_enabled {
            let nis2_result = nis2::assess_nis2_compliance(target).await?;
            overall_score += nis2_result.supply_chain_security.overall_score;
            score_count += 1;
            
            findings.extend(self.convert_nis2_findings(&nis2_result));
            recommendations.extend(self.convert_nis2_recommendations(&nis2_result));
        }

        let final_score = if score_count > 0 { overall_score / score_count as f64 } else { 0.0 };
        let compliance_status = self.determine_overall_status(final_score);

        Ok(ComplianceReport {
            target: target.to_string(),
            framework: if self.config.dora_enabled && self.config.nis2_enabled {
                ComplianceFramework::Both
            } else if self.config.dora_enabled {
                ComplianceFramework::DORA
            } else {
                ComplianceFramework::NIS2
            },
            assessment_date: chrono::Utc::now(),
            overall_score: final_score,
            compliance_status,
            findings,
            recommendations,
        })
    }

    fn convert_dora_findings(&self, dora_result: &dora::DORAAssessment) -> Vec<Finding> {
        let mut findings = Vec::new();
        
        if dora_result.ict_risk_score < 0.7 {
            findings.push(Finding {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Low ICT Risk Score".to_string(),
                description: format!("ICT risk score of {} is below recommended threshold", dora_result.ict_risk_score),
                severity: Severity::High,
                category: Category::Technical,
                evidence: vec![format!("ICT Risk Score: {}", dora_result.ict_risk_score)],
            });
        }

        if dora_result.resilience_score < 0.8 {
            findings.push(Finding {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Low Resilience Score".to_string(),
                description: format!("Resilience score of {} indicates operational weaknesses", dora_result.resilience_score),
                severity: Severity::Medium,
                category: Category::Organizational,
                evidence: vec![format!("Resilience Score: {}", dora_result.resilience_score)],
            });
        }

        findings
    }

    fn convert_nis2_findings(&self, nis2_result: &nis2::NIS2Assessment) -> Vec<Finding> {
        let mut findings = Vec::new();
        
        if nis2_result.incident_handling.reporting_time > Duration::from_secs(24 * 3600) {
            findings.push(Finding {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Slow Incident Reporting".to_string(),
                description: "Incident reporting time exceeds NIS2 requirements".to_string(),
                severity: Severity::High,
                category: Category::Process,
                evidence: vec![format!("Reporting Time: {:?}", nis2_result.incident_handling.reporting_time)],
            });
        }

        if !nis2_result.business_continuity.plan_exists {
            findings.push(Finding {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Missing Business Continuity Plan".to_string(),
                description: "No business continuity plan found".to_string(),
                severity: Severity::Critical,
                category: Category::Organizational,
                evidence: vec!["No BCP found".to_string()],
            });
        }

        findings
    }

    fn convert_dora_recommendations(&self, dora_result: &dora::DORAAssessment) -> Vec<Recommendation> {
        dora_result.recommendations.iter().map(|rec| Recommendation {
            id: uuid::Uuid::new_v4().to_string(),
            title: rec.clone(),
            description: format!("DORA compliance recommendation: {}", rec),
            priority: Priority::High,
            effort: Effort::Medium,
            impact: Impact::High,
        }).collect()
    }

    fn convert_nis2_recommendations(&self, nis2_result: &nis2::NIS2Assessment) -> Vec<Recommendation> {
        nis2_result.recommendations.iter().map(|rec| Recommendation {
            id: uuid::Uuid::new_v4().to_string(),
            title: rec.clone(),
            description: format!("NIS2 compliance recommendation: {}", rec),
            priority: Priority::High,
            effort: Effort::Medium,
            impact: Impact::High,
        }).collect()
    }

    fn determine_overall_status(&self, score: f64) -> ComplianceStatus {
        if score >= 0.8 {
            ComplianceStatus::Compliant
        } else if score >= 0.6 {
            ComplianceStatus::PartiallyCompliant
        } else {
            ComplianceStatus::NonCompliant
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compliance_engine() {
        let config = ComplianceConfig::default();
        let engine = ComplianceEngine::new(config);
        let result = engine.run_compliance_assessment("example.com").await;
        assert!(result.is_ok());
    }
}
