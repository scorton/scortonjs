import { writeProjectConfig, loadConfig } from '../core/config.js';

export async function initProject(opts: { template?: string }) {
  writeProjectConfig({ api: 'http://localhost:8000', template: opts.template });
  const cfg = loadConfig();
  console.log('Initialized .scorton/config.json');
  console.log(JSON.stringify(cfg, null, 2));
}

