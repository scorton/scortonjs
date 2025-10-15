import { homedir } from 'os';
import { existsSync, readFileSync } from 'fs';
import { join } from 'path';

const HOME_CFG = join(homedir(), '.scorton', 'config.json');
const PROJ_CFG = join(process.cwd(), '.scorton', 'config.json');

function readJson(path) {
    try {
        return JSON.parse(readFileSync(path, 'utf8'));
    }
    catch {
        return {};
    }
}

function loadConfig() {
    const defaults = {
        api: 'http://localhost:8000',
    };
    const env = {
        api: process.env.SCORTON_API_URL,
        token: process.env.SCORTON_TOKEN,
    };
    const home = existsSync(HOME_CFG) ? readJson(HOME_CFG) : {};
    const proj = existsSync(PROJ_CFG) ? readJson(PROJ_CFG) : {};
    
    console.log('Debug merge:');
    console.log('  defaults:', defaults);
    console.log('  home:', home);
    console.log('  proj:', proj);
    console.log('  env:', env);
    
    const result = { ...defaults, ...home, ...proj, ...env };
    console.log('  result:', result);
    
    return result;
}

console.log('Final config:', JSON.stringify(loadConfig(), null, 2));
