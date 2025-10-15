use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DORAAssessment {
    pub ict_risk_score: f64,
    pub incident_response_time: Duration,
    pub third_party_risks: Vec<ThirdPartyRisk>,
    pub resilience_score: f64,
    pub compliance_status: ComplianceStatus,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThirdPartyRisk {
    pub vendor_name: String,
    pub risk_level: RiskLevel,
    pub assessment_date: chrono::DateTime<chrono::Utc>,
    pub criticality: Criticality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Criticality {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    PartiallyCompliant,
    NonCompliant,
    Unknown,
}

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

pub struct ComplianceAssessor {
    config: ComplianceConfig,
}

#[derive(Debug, Clone)]
pub struct ComplianceConfig {
    pub dora_thresholds: DORAThresholds,
    pub nis2_thresholds: NIS2Thresholds,
}

#[derive(Debug, Clone)]
pub struct DORAThresholds {
    pub max_incident_response_time: Duration,
    pub min_resilience_score: f64,
    pub max_third_party_risk: RiskLevel,
}

#[derive(Debug, Clone)]
pub struct NIS2Thresholds {
    pub max_incident_reporting_time: Duration,
    pub min_bcp_score: f64,
    pub min_supply_chain_score: f64,
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            dora_thresholds: DORAThresholds {
                max_incident_response_time: Duration::from_secs(4 * 3600), // 4 hours
                min_resilience_score: 0.7,
                max_third_party_risk: RiskLevel::Medium,
            },
            nis2_thresholds: NIS2Thresholds {
                max_incident_reporting_time: Duration::from_secs(24 * 3600), // 24 hours
                min_bcp_score: 0.8,
                min_supply_chain_score: 0.7,
            },
        }
    }
}

impl ComplianceAssessor {
    pub fn new(config: ComplianceConfig) -> Self {
        Self { config }
    }

    pub async fn assess_dora_compliance(&self, target: &str) -> Result<DORAAssessment> {
        // Simulate DORA assessment - in real implementation, this would
        // analyze actual infrastructure, policies, and procedures
        let ict_risk_score = self.calculate_ict_risk_score(target).await?;
        let incident_response_time = self.measure_incident_response_time(target).await?;
        let third_party_risks = self.assess_third_party_risks(target).await?;
        let resilience_score = self.calculate_resilience_score(target).await?;
        
        let compliance_status = self.determine_dora_compliance_status(
            &ict_risk_score,
            &incident_response_time,
            &third_party_risks,
            &resilience_score,
        );

        let recommendations = self.generate_dora_recommendations(&compliance_status);

        Ok(DORAAssessment {
            ict_risk_score,
            incident_response_time,
            third_party_risks,
            resilience_score,
            compliance_status,
            recommendations,
        })
    }

    pub async fn assess_nis2_compliance(&self, target: &str) -> Result<NIS2Assessment> {
        let risk_level = self.assess_nis2_risk_level(target).await?;
        let incident_handling = self.assess_incident_handling(target).await?;
        let business_continuity = self.assess_business_continuity(target).await?;
        let supply_chain_security = self.assess_supply_chain_security(target).await?;
        
        let compliance_status = self.determine_nis2_compliance_status(
            &risk_level,
            &incident_handling,
            &business_continuity,
            &supply_chain_security,
        );

        let recommendations = self.generate_nis2_recommendations(&compliance_status);

        Ok(NIS2Assessment {
            risk_level,
            incident_handling,
            business_continuity,
            supply_chain_security,
            compliance_status,
            recommendations,
        })
    }

    async fn calculate_ict_risk_score(&self, _target: &str) -> Result<f64> {
        // Placeholder implementation
        // In real implementation, this would analyze:
        // - Infrastructure security
        // - Network security
        // - Application security
        // - Data protection measures
        Ok(0.75)
    }

    async fn measure_incident_response_time(&self, _target: &str) -> Result<Duration> {
        // Placeholder implementation
        // In real implementation, this would measure actual response times
        Ok(Duration::from_secs(2 * 3600)) // 2 hours
    }

    async fn assess_third_party_risks(&self, _target: &str) -> Result<Vec<ThirdPartyRisk>> {
        // Placeholder implementation
        // In real implementation, this would assess actual third-party vendors
        Ok(vec![
            ThirdPartyRisk {
                vendor_name: "Cloud Provider".to_string(),
                risk_level: RiskLevel::Medium,
                assessment_date: chrono::Utc::now(),
                criticality: Criticality::High,
            },
        ])
    }

    async fn calculate_resilience_score(&self, _target: &str) -> Result<f64> {
        // Placeholder implementation
        // In real implementation, this would calculate actual resilience metrics
        Ok(0.85)
    }

    async fn assess_nis2_risk_level(&self, _target: &str) -> Result<RiskLevel> {
        // Placeholder implementation
        Ok(RiskLevel::Medium)
    }

    async fn assess_incident_handling(&self, _target: &str) -> Result<IncidentMetrics> {
        // Placeholder implementation
        Ok(IncidentMetrics {
            detection_time: Duration::from_secs(15 * 60), // 15 minutes
            response_time: Duration::from_secs(1 * 3600), // 1 hour
            resolution_time: Duration::from_secs(4 * 3600), // 4 hours
            reporting_time: Duration::from_secs(2 * 3600), // 2 hours
            incidents_last_year: 3,
        })
    }

    async fn assess_business_continuity(&self, _target: &str) -> Result<BCPStatus> {
        // Placeholder implementation
        Ok(BCPStatus {
            plan_exists: true,
            last_tested: Some(chrono::Utc::now() - chrono::Duration::days(30)),
            recovery_time_objective: Duration::from_secs(4 * 3600), // 4 hours
            recovery_point_objective: Duration::from_secs(1 * 3600), // 1 hour
            backup_frequency: "Daily".to_string(),
        })
    }

    async fn assess_supply_chain_security(&self, _target: &str) -> Result<SupplyChainScore> {
        // Placeholder implementation
        Ok(SupplyChainScore {
            vendor_assessment: 0.8,
            security_requirements: 0.75,
            monitoring_capability: 0.7,
            overall_score: 0.75,
        })
    }

    fn determine_dora_compliance_status(
        &self,
        ict_risk_score: &f64,
        incident_response_time: &Duration,
        third_party_risks: &[ThirdPartyRisk],
        resilience_score: &f64,
    ) -> ComplianceStatus {
        let response_time_ok = *incident_response_time <= self.config.dora_thresholds.max_incident_response_time;
        let resilience_ok = *resilience_score >= self.config.dora_thresholds.min_resilience_score;
        let third_party_ok = !third_party_risks.iter().any(|risk| {
            matches!(risk.risk_level, RiskLevel::High | RiskLevel::Critical)
        });

        if response_time_ok && resilience_ok && third_party_ok {
            ComplianceStatus::Compliant
        } else if response_time_ok || resilience_ok || third_party_ok {
            ComplianceStatus::PartiallyCompliant
        } else {
            ComplianceStatus::NonCompliant
        }
    }

    fn determine_nis2_compliance_status(
        &self,
        _risk_level: &RiskLevel,
        incident_handling: &IncidentMetrics,
        business_continuity: &BCPStatus,
        supply_chain_security: &SupplyChainScore,
    ) -> ComplianceStatus {
        let incident_ok = incident_handling.reporting_time <= self.config.nis2_thresholds.max_incident_reporting_time;
        let bcp_ok = business_continuity.plan_exists && business_continuity.last_tested.is_some();
        let supply_chain_ok = supply_chain_security.overall_score >= self.config.nis2_thresholds.min_supply_chain_score;

        if incident_ok && bcp_ok && supply_chain_ok {
            ComplianceStatus::Compliant
        } else if incident_ok || bcp_ok || supply_chain_ok {
            ComplianceStatus::PartiallyCompliant
        } else {
            ComplianceStatus::NonCompliant
        }
    }

    fn generate_dora_recommendations(&self, status: &ComplianceStatus) -> Vec<String> {
        match status {
            ComplianceStatus::Compliant => vec!["Maintain current compliance posture".to_string()],
            ComplianceStatus::PartiallyCompliant => vec![
                "Improve incident response procedures".to_string(),
                "Enhance third-party risk management".to_string(),
                "Strengthen operational resilience".to_string(),
            ],
            ComplianceStatus::NonCompliant => vec![
                "Implement comprehensive ICT risk management framework".to_string(),
                "Establish incident response team and procedures".to_string(),
                "Conduct third-party risk assessments".to_string(),
                "Develop business continuity plans".to_string(),
            ],
            ComplianceStatus::Unknown => vec!["Conduct comprehensive compliance assessment".to_string()],
        }
    }

    fn generate_nis2_recommendations(&self, status: &ComplianceStatus) -> Vec<String> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dora_assessment() {
        let config = ComplianceConfig::default();
        let assessor = ComplianceAssessor::new(config);
        let result = assessor.assess_dora_compliance("example.com").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_nis2_assessment() {
        let config = ComplianceConfig::default();
        let assessor = ComplianceAssessor::new(config);
        let result = assessor.assess_nis2_compliance("example.com").await;
        assert!(result.is_ok());
    }
}
