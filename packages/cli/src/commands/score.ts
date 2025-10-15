import { runRustScore } from '../providers/rustExecutor.js';
import { loadConfig } from '../core/config.js';
import { writeJson } from '../core/output.js';
import { safeParseJSON, ScoreSchema } from '../core/validate.js';

export async function runScore(target: string, opts: { api?: string; token?: string }) {
  const cfg = loadConfig();
  
  try {
    // Use Rust executor for scoring
    const result = await runRustScore(target);
    
    // Validate the result
    const validated = ScoreSchema.safeParse({
      status: 'ok',
      score: result.overall,
      factors: {
        technical: result.technical,
        behavioral: result.behavioral,
        organizational: result.organizational
      }
    });
    
    if (!validated.success) {
      console.warn('Warning: score response schema mismatch');
    }
    
    writeJson('score', target, {
      status: 'ok',
      score: result.overall,
      factors: {
        technical: result.technical,
        behavioral: result.behavioral,
        organizational: result.organizational
      },
      timestamp: new Date().toISOString()
    });
    
    console.log(`Cyberscore for ${target}: ${(result.overall * 100).toFixed(1)}%`);
    console.log(`Technical: ${(result.technical * 100).toFixed(1)}%`);
    console.log(`Behavioral: ${(result.behavioral * 100).toFixed(1)}%`);
    console.log(`Organizational: ${(result.organizational * 100).toFixed(1)}%`);
    
  } catch (error) {
    console.error(`Score calculation failed: ${error}`);
    writeJson('score', target, { 
      status: 'error', 
      error: error instanceof Error ? error.message : String(error),
      timestamp: new Date().toISOString()
    });
  }
}

