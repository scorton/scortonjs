import { existsSync, mkdirSync, readdirSync } from 'fs';
import { join } from 'path';

export async function moduleCreate(name?: string) {
  if (!name) {
    console.error('Provide --name <moduleName>');
    process.exit(1);
  }
  const dir = join(process.cwd(), 'modules', name);
  if (!existsSync(dir)) mkdirSync(dir, { recursive: true });
  console.log(`Module created: ${dir}`);
}

export async function moduleList() {
  const root = join(process.cwd(), 'modules');
  if (!existsSync(root)) return console.log('No modules.');
  const items = readdirSync(root);
  console.log(items.join('\n'));
}

