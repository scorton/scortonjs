import { homedir } from 'os';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'fs';
import { join } from 'path';

export type ScortonConfig = {
  api?: string;
  token?: string;
  template?: string;
  useRustBackend?: boolean;
  rustServerPort?: number;
  complianceMode?: 'dora' | 'nis2' | 'both';
  rustTimeout?: number;
  pythonFallback?: boolean;
};

const HOME_CFG = join(homedir(), '.scorton', 'config.json');
const PROJ_CFG = join(process.cwd(), '.scorton', 'config.json');

function readJson(path: string): any {
  try {
    return JSON.parse(readFileSync(path, 'utf8'));
  } catch {
    return {};
  }
}

export function loadConfig(): ScortonConfig {
  const defaults: ScortonConfig = {
    api: 'http://localhost:8000',
    useRustBackend: true,
    rustServerPort: 8001,
    complianceMode: 'both',
    rustTimeout: 30000,
    pythonFallback: true,
  };
  const env: ScortonConfig = {};
  if (process.env.SCORTON_API_URL) env.api = process.env.SCORTON_API_URL;
  if (process.env.SCORTON_TOKEN) env.token = process.env.SCORTON_TOKEN;
  if (process.env.SCORTON_USE_RUST) env.useRustBackend = process.env.SCORTON_USE_RUST === 'true';
  if (process.env.SCORTON_RUST_PORT) env.rustServerPort = parseInt(process.env.SCORTON_RUST_PORT);
  if (process.env.SCORTON_COMPLIANCE_MODE) env.complianceMode = process.env.SCORTON_COMPLIANCE_MODE as 'dora' | 'nis2' | 'both';
  if (process.env.SCORTON_RUST_TIMEOUT) env.rustTimeout = parseInt(process.env.SCORTON_RUST_TIMEOUT);
  if (process.env.SCORTON_PYTHON_FALLBACK) env.pythonFallback = process.env.SCORTON_PYTHON_FALLBACK === 'true';
  
  const home = existsSync(HOME_CFG) ? readJson(HOME_CFG) : {};
  const proj = existsSync(PROJ_CFG) ? readJson(PROJ_CFG) : {};
  // Merge order: defaults -> home -> project -> env (env wins)
  return { ...defaults, ...home, ...proj, ...env };
}

export function writeProjectConfig(partial: Partial<ScortonConfig>): void {
  const dir = join(process.cwd(), '.scorton');
  if (!existsSync(dir)) mkdirSync(dir, { recursive: true });
  const current = existsSync(PROJ_CFG) ? readJson(PROJ_CFG) : {};
  const next = { ...current, ...partial };
  writeFileSync(PROJ_CFG, JSON.stringify(next, null, 2));
}

