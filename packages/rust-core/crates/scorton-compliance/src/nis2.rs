use serde::{Deserialize, Serialize};
use std::time::Duration;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NIS2Assessment {
    pub risk_level: RiskLevel,
    pub incident_handling: IncidentMetrics,
    pub business_continuity: BCPStatus,
    pub supply_chain_security: SupplyChainScore,
    pub compliance_status: ComplianceStatus,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentMetrics {
    pub detection_time: Duration,
    pub response_time: Duration,
    pub resolution_time: Duration,
    pub reporting_time: Duration,
    pub incidents_last_year: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BCPStatus {
    pub plan_exists: bool,
    pub last_tested: Option<chrono::DateTime<chrono::Utc>>,
    pub recovery_time_objective: Duration,
    pub recovery_point_objective: Duration,
    pub backup_frequency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainScore {
    pub vendor_assessment: f64,
    pub security_requirements: f64,
    pub monitoring_capability: f64,
    pub overall_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    PartiallyCompliant,
    NonCompliant,
    Unknown,
}

pub async fn assess_nis2_compliance(target: &str) -> Result<NIS2Assessment> {
    let risk_level = assess_nis2_risk_level(target).await?;
    let incident_handling = assess_incident_handling(target).await?;
    let business_continuity = assess_business_continuity(target).await?;
    let supply_chain_security = assess_supply_chain_security(target).await?;
    
    let compliance_status = determine_nis2_compliance_status(
        &risk_level,
        &incident_handling,
        &business_continuity,
        &supply_chain_security,
    );

    let recommendations = generate_nis2_recommendations(&compliance_status);

    Ok(NIS2Assessment {
        risk_level,
        incident_handling,
        business_continuity,
        supply_chain_security,
        compliance_status,
        recommendations,
    })
}

async fn assess_nis2_risk_level(_target: &str) -> Result<RiskLevel> {
    // Placeholder implementation
    // In real implementation, this would assess actual risk factors
    Ok(RiskLevel::Medium)
}

async fn assess_incident_handling(_target: &str) -> Result<IncidentMetrics> {
    // Placeholder implementation
    // In real implementation, this would assess actual incident handling capabilities
    Ok(IncidentMetrics {
        detection_time: Duration::from_secs(15 * 60), // 15 minutes
        response_time: Duration::from_secs(1 * 3600), // 1 hour
        resolution_time: Duration::from_secs(4 * 3600), // 4 hours
        reporting_time: Duration::from_secs(2 * 3600), // 2 hours
        incidents_last_year: 3,
    })
}

async fn assess_business_continuity(_target: &str) -> Result<BCPStatus> {
    // Placeholder implementation
    // In real implementation, this would assess actual BCP status
    Ok(BCPStatus {
        plan_exists: true,
        last_tested: Some(chrono::Utc::now() - chrono::Duration::days(30)),
        recovery_time_objective: Duration::from_secs(4 * 3600), // 4 hours
        recovery_point_objective: Duration::from_secs(1 * 3600), // 1 hour
        backup_frequency: "Daily".to_string(),
    })
}

async fn assess_supply_chain_security(_target: &str) -> Result<SupplyChainScore> {
    // Placeholder implementation
    // In real implementation, this would assess actual supply chain security
    Ok(SupplyChainScore {
        vendor_assessment: 0.8,
        security_requirements: 0.75,
        monitoring_capability: 0.7,
        overall_score: 0.75,
    })
}

fn determine_nis2_compliance_status(
    _risk_level: &RiskLevel,
    incident_handling: &IncidentMetrics,
    business_continuity: &BCPStatus,
    supply_chain_security: &SupplyChainScore,
) -> ComplianceStatus {
    let max_reporting_time = Duration::from_secs(24 * 3600); // 24 hours
    let min_supply_chain_score = 0.7;
    
    let incident_ok = incident_handling.reporting_time <= max_reporting_time;
    let bcp_ok = business_continuity.plan_exists && business_continuity.last_tested.is_some();
    let supply_chain_ok = supply_chain_security.overall_score >= min_supply_chain_score;

    if incident_ok && bcp_ok && supply_chain_ok {
        ComplianceStatus::Compliant
    } else if incident_ok || bcp_ok || supply_chain_ok {
        ComplianceStatus::PartiallyCompliant
    } else {
        ComplianceStatus::NonCompliant
    }
}

fn generate_nis2_recommendations(status: &ComplianceStatus) -> Vec<String> {
    match status {
        ComplianceStatus::Compliant => vec!["Maintain current compliance posture".to_string()],
        ComplianceStatus::PartiallyCompliant => vec![
            "Improve incident reporting procedures".to_string(),
            "Enhance business continuity planning".to_string(),
            "Strengthen supply chain security".to_string(),
        ],
        ComplianceStatus::NonCompliant => vec![
            "Implement comprehensive risk management system".to_string(),
            "Establish incident handling procedures".to_string(),
            "Develop business continuity plans".to_string(),
            "Conduct supply chain security assessments".to_string(),
        ],
        ComplianceStatus::Unknown => vec!["Conduct comprehensive compliance assessment".to_string()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_nis2_assessment() {
        let result = assess_nis2_compliance("example.com").await;
        assert!(result.is_ok());
    }
}
