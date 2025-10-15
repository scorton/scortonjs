import { loadConfig } from '../core/config.js';
import { writeJson } from '../core/output.js';

// Mock implementations until Rust modules are built
interface ScanResult {
  target: string;
  tool: string;
  status: string;
  data: string;
  duration_ms: number;
  timestamp: string;
}

interface CyberScore {
  technical: number;
  behavioral: number;
  organizational: number;
  overall: number;
}

interface DORAResult {
  ict_risk_score: number;
  incident_response_time_hours: number;
  resilience_score: number;
  compliance_status: string;
  recommendations: string[];
}

interface NIS2Result {
  risk_level: string;
  incident_reporting_time_hours: number;
  business_continuity_score: number;
  supply_chain_score: number;
  compliance_status: string;
  recommendations: string[];
}

interface ServerConfig {
  host: string;
  port: number;
  jwt_secret: string;
}

// Mock functions - replace with actual Rust imports when available
async function runSecurityScan(tool: string, target: string): Promise<ScanResult> {
  // Mock implementation
  return {
    target,
    tool,
    status: 'success',
    data: JSON.stringify({ message: 'Mock Rust scan result', tool, target }),
    duration_ms: 100,
    timestamp: new Date().toISOString()
  };
}

async function calculateCyberScore(target: string): Promise<CyberScore> {
  // Mock implementation
  return {
    technical: 0.75,
    behavioral: 0.65,
    organizational: 0.80,
    overall: 0.73
  };
}

async function assessDoraCompliance(target: string): Promise<DORAResult> {
  // Mock implementation
  return {
    ict_risk_score: 0.75,
    incident_response_time_hours: 2.0,
    resilience_score: 0.85,
    compliance_status: 'PartiallyCompliant',
    recommendations: ['Improve incident response procedures', 'Enhance third-party risk management']
  };
}

async function assessNis2Compliance(target: string): Promise<NIS2Result> {
  // Mock implementation
  return {
    risk_level: 'Medium',
    incident_reporting_time_hours: 2.0,
    business_continuity_score: 0.80,
    supply_chain_score: 0.75,
    compliance_status: 'Compliant',
    recommendations: ['Maintain current compliance posture']
  };
}

async function runComprehensiveScan(target: string): Promise<ScanResult> {
  // Mock implementation
  return {
    target,
    tool: 'comprehensive',
    status: 'success',
    data: JSON.stringify({ 
      port_scan: { ports: [80, 443], status: 'completed' },
      ssl_scan: { certificate_valid: true },
      dns_enum: { records: ['A', 'MX', 'TXT'] },
      headers_check: { security_score: 0.8 }
    }),
    duration_ms: 500,
    timestamp: new Date().toISOString()
  };
}

function startRustServerMock(config: ServerConfig): void {
  console.log(`Mock: Starting Rust server on ${config.host}:${config.port}`);
}

function getRustVersion(): string {
  return '0.1.0-mock';
}

function getSupportedTools(): string[] {
  return ['port_scan', 'ssl_scan', 'dns_enum', 'headers_check', 'comprehensive'];
}

export async function runRustScan(tool: string, target: string): Promise<ScanResult> {
  try {
    const result = await runSecurityScan(tool, target);
    writeJson('rust-scan', target, JSON.parse(result.data));
    return result;
  } catch (error) {
    console.error(`Rust scan failed: ${error}`);
    throw error;
  }
}

export async function runRustScore(target: string): Promise<CyberScore> {
  try {
    const result = await calculateCyberScore(target);
    writeJson('rust-score', target, result);
    return result;
  } catch (error) {
    console.error(`Rust score calculation failed: ${error}`);
    throw error;
  }
}

export async function runRustDoraCompliance(target: string): Promise<DORAResult> {
  try {
    const result = await assessDoraCompliance(target);
    writeJson('rust-dora', target, result);
    return result;
  } catch (error) {
    console.error(`DORA compliance assessment failed: ${error}`);
    throw error;
  }
}

export async function runRustNis2Compliance(target: string): Promise<NIS2Result> {
  try {
    const result = await assessNis2Compliance(target);
    writeJson('rust-nis2', target, result);
    return result;
  } catch (error) {
    console.error(`NIS2 compliance assessment failed: ${error}`);
    throw error;
  }
}

export async function runRustComprehensiveScan(target: string): Promise<ScanResult> {
  try {
    const result = await runComprehensiveScan(target);
    writeJson('rust-comprehensive', target, JSON.parse(result.data));
    return result;
  } catch (error) {
    console.error(`Rust comprehensive scan failed: ${error}`);
    throw error;
  }
}

export function startRustServer(config: ServerConfig): void {
  try {
    startRustServerMock(config);
    console.log(`Rust server started on ${config.host}:${config.port}`);
  } catch (error) {
    console.error(`Failed to start Rust server: ${error}`);
    throw error;
  }
}

export function getRustInfo() {
  return {
    version: getRustVersion(),
    supportedTools: getSupportedTools(),
  };
}

// Fallback to Python if Rust is not available
export async function runScanWithFallback(tool: string, target: string, opts: { api?: string; token?: string }) {
  try {
    return await runRustScan(tool, target);
  } catch (rustError) {
    console.warn(`Rust scan failed, falling back to Python: ${rustError}`);
    
    // Import Python executor as fallback
    const { runPythonCapture } = await import('../providers/pythonExecutor.js');
    const cfg = loadConfig();
    const api = opts.api ?? cfg.api;
    const token = opts.token ?? cfg.token;
    const args = ['scan', tool, target];
    if (api) args.push('--api', api);
    if (token) args.push('--token', token);
    
    const { code, stdout, stderr } = await runPythonCapture(args);
    return {
      target,
      tool,
      status: code === 0 ? 'success' : 'error',
      data: stdout,
      duration_ms: 0,
      timestamp: new Date().toISOString(),
    };
  }
}