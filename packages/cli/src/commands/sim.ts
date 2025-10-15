import { writeJson } from '../core/output.js';

export async function runSimPhishing(type: string, opts: { email?: string }) {
  if (type !== 'phishing') {
    console.error('Only phishing simulation stub is available');
  }
  const data = { type: 'phishing', targetEmail: opts.email ?? 'user@example.com', plan: 'Send simulated phishing email' };
  const file = writeJson('sim', 'phishing', data);
  console.log(`Simulation plan written: ${file}`);
}

