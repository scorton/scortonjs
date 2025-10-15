#!/usr/bin/env node
import pkg from './package.json' with { type: 'json' };

const commands = {
  hello() {
    console.log('Hello from scortonjs!');
  },
  version() {
    console.log(pkg.version);
  },
};

function showHelp() {
  console.log('Usage: scortonjs <command>');
  console.log('Commands:');
  console.log('  hello    Print greeting');
  console.log('  version  Print version');
}

const [, , command] = process.argv;
(commands[command] ?? showHelp)();
