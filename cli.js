#!/usr/bin/env node
import { readFileSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));

const [, , command] = process.argv;

function showHelp() {
  console.log('Usage: scortonjs <command>');
  console.log('Commands:');
  console.log('  hello    Print greeting');
  console.log('  version  Print version');
  console.log('  init     Initialize project (not implemented)');
  console.log('  generate Generate code (not implemented)');
  console.log('  build    Build project (not implemented)');
}

  switch (command) {
    case 'hello':
      console.log('Hello from scortonjs!');
      break;
    case 'version':
      const pkg = JSON.parse(
        readFileSync(join(__dirname, 'package.json'), 'utf8')
      );
      console.log(pkg.version);
      break;
    case 'init':
    case 'generate':
    case 'build':
      console.log('Not implemented yet.');
      break;
    default:
      showHelp();
      break;
  }
