import { writeMarkdown } from '../core/output.js';

export async function makeReport(opts: { format?: 'md' | 'json' }) {
  const md = `# Scorton Report\n\nThis is a placeholder report. Use 'score' and 'audit' to generate data.`;
  const file = writeMarkdown('report', 'summary', md);
  console.log(`Report written: ${file}`);
}

