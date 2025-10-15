import { spawn, execSync } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

function detectPython(): string {
  const candidates = ['python3', 'python'];
  for (const cmd of candidates) {
    try {
      const verOut = execSync(`${cmd} --version`, { encoding: 'utf8' }).trim();
      const match = verOut.match(/Python\s+(\d+)\.(\d+)\.(\d+)/);
      if (!match) continue;
      const major = parseInt(match[1], 10);
      const minor = parseInt(match[2], 10);
      if (major > 3 || (major === 3 && minor >= 7)) {
        // Check requests availability
        try {
          execSync(`${cmd} -c "import requests; print('ok')"`, { stdio: 'pipe' });
          return cmd;
        } catch (_) {
          throw new Error('Python found but requests is missing. Install with: pip3 install requests');
        }
      }
    } catch (_) {}
  }
  throw new Error('Python 3 not found in PATH');
}

export async function runPython(args: string[]): Promise<number> {
  const python = detectPython();
  const override = process.env.SCORTON_PY_PATH;
  const projectRoot = join(__dirname, '../../../..');
  const pyPath = override || join(projectRoot, 'scorton.py');

  return await new Promise((resolve: (v: number) => void, reject: (e: unknown) => void) => {
    const child = spawn(python, [pyPath, ...args], { stdio: 'inherit', cwd: projectRoot });
    child.on('close', (code: number | null) => resolve(code ?? 1));
    child.on('error', (err: unknown) => reject(err));
  });
}

export async function runPythonCapture(args: string[]): Promise<{ code: number; stdout: string; stderr: string }> {
  const python = detectPython();
  const override = process.env.SCORTON_PY_PATH;
  const projectRoot = join(__dirname, '../../../..');
  const pyPath = override || join(projectRoot, 'scorton.py');

  return await new Promise((resolve: (v: { code: number; stdout: string; stderr: string }) => void, reject: (e: unknown) => void) => {
    const child = spawn(python, [pyPath, ...args], { stdio: ['ignore', 'pipe', 'pipe'], cwd: projectRoot });
    let out: string = '';
    let err: string = '';
    child.stdout.on('data', (d: Buffer | string) => (out += d.toString()));
    child.stderr.on('data', (d: Buffer | string) => (err += d.toString()));
    child.on('close', (code: number | null) => resolve({ code: code ?? 1, stdout: out, stderr: err }));
    child.on('error', (e: unknown) => reject(e));
  });
}

