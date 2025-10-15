import { writeJson } from '../core/output.js';

export async function personaMap(_sub: string, opts: { user?: string }) {
  const data = { user: opts.user ?? 'anonymous', persona: 'Explorer', confidence: 0.5 };
  const file = writeJson('persona', data.user, data);
  console.log(`Persona map written: ${file}`);
}

