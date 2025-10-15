import { runRustComprehensiveScan } from '../providers/rustExecutor.js';
import { runPythonCapture } from '../providers/pythonExecutor.js';
import { loadConfig } from '../core/config.js';
import { writeJson, writeMarkdown } from '../core/output.js';
import { safeParseJSON } from '../core/validate.js';

export async function runAudit(target: string, opts: { api?: string; token?: string }) {
  const cfg = loadConfig();
  
  try {
    // Try Rust comprehensive scan first
    const rustResult = await runRustComprehensiveScan(target);
    
    if (rustResult.status === 'success') {
      const parsed = safeParseJSON(rustResult.data);
      if (parsed.ok) {
        writeJson('audit', target, parsed.data as any);
      } else {
        writeJson('audit', target, { status: 'ok', data: rustResult.data });
      }
      
      const md = `# Deep Audit Summary (Rust)\n\nTarget: ${target}\n\nStatus: OK\n\nDuration: ${rustResult.duration_ms}ms\n\nTimestamp: ${rustResult.timestamp}\n`;
      writeMarkdown('audit', target, md);
    } else {
      throw new Error(`Rust audit failed: ${rustResult.data}`);
    }
  } catch (rustError) {
    console.warn(`Rust audit failed, falling back to Python: ${rustError}`);
    
    // Fallback to Python
    const api = opts.api ?? cfg.api;
    const token = opts.token ?? cfg.token;
    const args = ['audit', target];
    if (api) args.push('--api', api);
    if (token) args.push('--token', token);
    
    const { code, stdout, stderr } = await runPythonCapture(args);
    const parsed = safeParseJSON(stdout);
    if (parsed.ok) {
      writeJson('audit', target, parsed.data as any);
    } else {
      writeJson('audit', target, { status: code === 0 ? 'ok' : 'error', stdout, stderr });
    }
    
    const md = `# Audit Summary (Python Fallback)\n\nTarget: ${target}\n\nStatus: ${code === 0 ? 'OK' : 'ERROR'}\n`;
    writeMarkdown('audit', target, md);
  }
}

