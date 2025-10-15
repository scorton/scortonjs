import { loadConfig } from './dist/core/config.js';
import { homedir } from 'os';
import { existsSync, readFileSync } from 'fs';
import { join } from 'path';

const HOME_CFG = join(homedir(), '.scorton', 'config.json');
const PROJ_CFG = join(process.cwd(), '.scorton', 'config.json');

console.log('HOME_CFG exists:', existsSync(HOME_CFG));
console.log('PROJ_CFG exists:', existsSync(PROJ_CFG));

if (existsSync(PROJ_CFG)) {
  console.log('PROJ_CFG content:', readFileSync(PROJ_CFG, 'utf8'));
}

console.log('Final config:', JSON.stringify(loadConfig(), null, 2));
