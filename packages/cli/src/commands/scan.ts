import { runScanWithFallback } from '../providers/rustExecutor.js';
import { loadConfig } from '../core/config.js';
import { writeJson } from '../core/output.js';
import { ScanSchema, safeParseJSON } from '../core/validate.js';

export async function runScan(tool: string, target: string, opts: { api?: string; token?: string }) {
  const cfg = loadConfig();
  
  // Use Rust as primary, Python as fallback
  try {
    const result = await runScanWithFallback(tool, target, opts);
    
    if (result.status === 'success') {
      const parsed = safeParseJSON(result.data);
      if (parsed.ok) {
        const validated = ScanSchema.safeParse(parsed.data);
        if (!validated.success) {
          console.warn('Warning: response did not match expected schema');
        }
        writeJson('scan', target, parsed.data as any);
      } else {
        writeJson('scan', target, { status: 'ok', data: result.data });
      }
    } else {
      writeJson('scan', target, { status: 'error', error: result.data });
    }
  } catch (error) {
    console.error(`Scan failed: ${error}`);
    writeJson('scan', target, { status: 'error', error: error instanceof Error ? error.message : String(error) });
  }
}

