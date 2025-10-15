import { runRustDoraCompliance, runRustNis2Compliance } from '../providers/rustExecutor.js';
import { loadConfig } from '../core/config.js';
import { writeJson, writeMarkdown } from '../core/output.js';

export async function runCompliance(framework: string, target: string, opts: { api?: string; token?: string }) {
  const cfg = loadConfig();
  
  try {
    if (framework === 'dora') {
      const result = await runRustDoraCompliance(target);
      writeJson('compliance-dora', target, result);
      
      const md = generateDoraMarkdown(target, result);
      writeMarkdown('compliance-dora', target, md);
      
    } else if (framework === 'nis2') {
      const result = await runRustNis2Compliance(target);
      writeJson('compliance-nis2', target, result);
      
      const md = generateNis2Markdown(target, result);
      writeMarkdown('compliance-nis2', target, md);
      
    } else if (framework === 'both') {
      const [doraResult, nis2Result] = await Promise.all([
        runRustDoraCompliance(target),
        runRustNis2Compliance(target)
      ]);
      
      const combinedResult = {
        target,
        dora: doraResult,
        nis2: nis2Result,
        timestamp: new Date().toISOString()
      };
      
      writeJson('compliance-both', target, combinedResult);
      
      const md = generateCombinedMarkdown(target, doraResult, nis2Result);
      writeMarkdown('compliance-both', target, md);
      
    } else {
      throw new Error(`Unknown compliance framework: ${framework}. Supported: dora, nis2, both`);
    }
  } catch (error) {
    console.error(`Compliance assessment failed: ${error}`);
    writeJson('compliance-error', target, { error: error instanceof Error ? error.message : String(error), framework, target });
  }
}

function generateDoraMarkdown(target: string, result: any): string {
  return `# DORA Compliance Assessment

## Target
${target}

## Assessment Results

### ICT Risk Score
${(result.ict_risk_score * 100).toFixed(1)}%

### Incident Response Time
${result.incident_response_time_hours.toFixed(1)} hours

### Resilience Score
${(result.resilience_score * 100).toFixed(1)}%

### Compliance Status
${result.compliance_status}

## Recommendations

${result.recommendations.map((rec: string, index: number) => `${index + 1}. ${rec}`).join('\n')}

---
*Assessment completed at ${new Date().toISOString()}*
`;
}

function generateNis2Markdown(target: string, result: any): string {
  return `# NIS2 Compliance Assessment

## Target
${target}

## Assessment Results

### Risk Level
${result.risk_level}

### Incident Reporting Time
${result.incident_reporting_time_hours.toFixed(1)} hours

### Business Continuity Score
${(result.business_continuity_score * 100).toFixed(1)}%

### Supply Chain Security Score
${(result.supply_chain_score * 100).toFixed(1)}%

### Compliance Status
${result.compliance_status}

## Recommendations

${result.recommendations.map((rec: string, index: number) => `${index + 1}. ${rec}`).join('\n')}

---
*Assessment completed at ${new Date().toISOString()}*
`;
}

function generateCombinedMarkdown(target: string, doraResult: any, nis2Result: any): string {
  return `# Combined Compliance Assessment (DORA + NIS2)

## Target
${target}

## DORA Assessment

### ICT Risk Score
${(doraResult.ict_risk_score * 100).toFixed(1)}%

### Incident Response Time
${doraResult.incident_response_time_hours.toFixed(1)} hours

### Resilience Score
${(doraResult.resilience_score * 100).toFixed(1)}%

### Compliance Status
${doraResult.compliance_status}

## NIS2 Assessment

### Risk Level
${nis2Result.risk_level}

### Incident Reporting Time
${nis2Result.incident_reporting_time_hours.toFixed(1)} hours

### Business Continuity Score
${(nis2Result.business_continuity_score * 100).toFixed(1)}%

### Supply Chain Security Score
${(nis2Result.supply_chain_score * 100).toFixed(1)}%

### Compliance Status
${nis2Result.compliance_status}

## Overall Recommendations

### DORA Recommendations
${doraResult.recommendations.map((rec: string, index: number) => `${index + 1}. ${rec}`).join('\n')}

### NIS2 Recommendations
${nis2Result.recommendations.map((rec: string, index: number) => `${index + 1}. ${rec}`).join('\n')}

---
*Assessment completed at ${new Date().toISOString()}*
`;
}
