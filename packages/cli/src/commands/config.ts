import { loadConfig, writeProjectConfig } from '../core/config.js';

export async function showConfig() {
  const cfg = loadConfig();
  const masked = { ...cfg, token: cfg.token ? '***' : undefined } as any;
  console.log(JSON.stringify(masked, null, 2));
}

export async function setConfig(kv: string) {
  const [key, value] = kv.split('=');
  if (!key || value === undefined) {
    console.error('Use --set key=value');
    process.exit(1);
  }
  writeProjectConfig({ [key]: value } as any);
  const cfg = loadConfig();
  console.log(JSON.stringify(cfg, null, 2));
}

