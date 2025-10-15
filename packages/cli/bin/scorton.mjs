#!/usr/bin/env node
try {
  await import('../dist/index.js');
} catch (e) {
  // Dev fallback: ts-node
  const { pathToFileURL, fileURLToPath } = await import('url');
  const { dirname, join } = await import('path');
  const __filename = fileURLToPath(import.meta.url);
  const __dirname = dirname(__filename);
  const entry = pathToFileURL(join(__dirname, '../src/index.ts')).href;
  const { register } = await import('ts-node/esm');
  register({ transpileOnly: true });
  await import(entry);
}

