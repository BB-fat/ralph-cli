#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { spawn } = require('child_process');

const BINARY_NAME = 'ralph';
const BINARY_DIR = path.join(__dirname, 'bin');
const BINARY_PATH = path.join(BINARY_DIR, BINARY_NAME);

function main() {
  // Check if binary exists
  if (!fs.existsSync(BINARY_PATH)) {
    console.error('Error: Ralph CLI binary not found.');
    console.error('');
    console.error('The binary should have been downloaded during installation.');
    console.error('Please try reinstalling:');
    console.error('  npm uninstall -g ralph-cli && npm install -g ralph-cli');
    console.error('');
    console.error('For offline installation, use:');
    console.error('  npm install ralph-cli --ralph-binary-path=/path/to/ralph');
    process.exit(1);
  }

  // Get all command line arguments (excluding node and script path)
  const args = process.argv.slice(2);

  // Spawn the binary process
  const child = spawn(BINARY_PATH, args, {
    stdio: 'inherit', // Pass through stdin, stdout, stderr
    env: process.env  // Pass through environment variables
  });

  // Handle process exit
  child.on('close', (code) => {
    process.exit(code || 0);
  });

  // Handle errors
  child.on('error', (error) => {
    console.error('Failed to start Ralph CLI:', error.message);
    process.exit(1);
  });

  // Handle Ctrl+C gracefully
  process.on('SIGINT', () => {
    child.kill('SIGINT');
  });

  process.on('SIGTERM', () => {
    child.kill('SIGTERM');
  });
}

main();
