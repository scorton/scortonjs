import { execFileSync } from 'child_process';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';
import { readFileSync } from 'fs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const CLI_PATH = join(__dirname, '..', 'cli.js');
const PKG_PATH = join(__dirname, '..', 'package.json');

describe('scortonjs CLI', () => {
  it('prints greeting for hello command', () => {
    const output = execFileSync('node', [CLI_PATH, 'hello'], { encoding: 'utf8' });
    expect(output.trim()).toBe('Hello from scortonjs!');
  });

  it('prints version number for version command', () => {
    const output = execFileSync('node', [CLI_PATH, 'version'], { encoding: 'utf8' });
    const pkg = JSON.parse(readFileSync(PKG_PATH, 'utf8'));
    expect(output.trim()).toBe(pkg.version);
  });
});
