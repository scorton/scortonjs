import { existsSync, mkdirSync, writeFileSync } from 'fs';
import { join } from 'path';

function slugify(input: string): string {
  return input.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/(^-|-$)/g, '');
}

export function ensureOutputDir(): string {
  const dir = join(process.cwd(), 'scorton-output');
  if (!existsSync(dir)) mkdirSync(dir, { recursive: true });
  return dir;
}

export function writeJson(cmd: string, target: string, data: unknown): string {
  const dir = ensureOutputDir();
  const ts = Date.now();
  const file = join(dir, `${cmd}-${slugify(target)}-${ts}.json`);
  writeFileSync(file, JSON.stringify(data, null, 2));
  return file;
}

export function writeMarkdown(cmd: string, target: string, content: string): string {
  const dir = ensureOutputDir();
  const ts = Date.now();
  const file = join(dir, `${cmd}-${slugify(target)}-${ts}.md`);
  writeFileSync(file, content);
  return file;
}

