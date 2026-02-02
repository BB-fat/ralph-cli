#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const https = require('https');
const { execSync } = require('child_process');

const PACKAGE_VERSION = require('./package.json').version;
const GITHUB_REPO = 'BB-fat/ralph-cli';
const BINARY_NAME = 'ralph';

// Platform and architecture mapping
const PLATFORM_MAP = {
  'darwin': 'apple-darwin',
  'linux': 'unknown-linux-gnu'
};

const ARCH_MAP = {
  'x64': 'x86_64',
  'arm64': 'aarch64'
};

function getPlatform() {
  const platform = process.platform;
  const arch = process.arch;

  if (!PLATFORM_MAP[platform]) {
    throw new Error(`Unsupported platform: ${platform}. Ralph CLI only supports macOS and Linux.`);
  }

  if (!ARCH_MAP[arch]) {
    throw new Error(`Unsupported architecture: ${arch}. Ralph CLI only supports x64 and arm64.`);
  }

  return {
    platform: PLATFORM_MAP[platform],
    arch: ARCH_MAP[arch],
    target: `${ARCH_MAP[arch]}-${PLATFORM_MAP[platform]}`
  };
}

function getBinaryUrl(target) {
  return `https://github.com/${GITHUB_REPO}/releases/download/v${PACKAGE_VERSION}/${BINARY_NAME}-${target}`;
}

function getChecksumUrl(target) {
  return `https://github.com/${GITHUB_REPO}/releases/download/v${PACKAGE_VERSION}/${BINARY_NAME}-${target}.sha256`;
}

function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https.get(url, { redirect: true }, (response) => {
      if (response.statusCode === 301 || response.statusCode === 302) {
        // Follow redirect
        https.get(response.headers.location, (redirectResponse) => {
          if (redirectResponse.statusCode !== 200) {
            reject(new Error(`Failed to download: ${redirectResponse.statusCode}`));
            return;
          }
          redirectResponse.pipe(file);
          file.on('finish', () => {
            file.close();
            resolve();
          });
        }).on('error', reject);
      } else if (response.statusCode === 200) {
        response.pipe(file);
        file.on('finish', () => {
          file.close();
          resolve();
        });
      } else {
        reject(new Error(`Failed to download: ${response.statusCode}`));
      }
    }).on('error', reject);
  });
}

async function downloadChecksum(target) {
  const url = getChecksumUrl(target);
  return new Promise((resolve, reject) => {
    https.get(url, { redirect: true }, (response) => {
      if (response.statusCode === 301 || response.statusCode === 302) {
        https.get(response.headers.location, (redirectResponse) => {
          if (redirectResponse.statusCode !== 200) {
            resolve(null); // Checksum not available, skip verification
            return;
          }
          let data = '';
          redirectResponse.on('data', chunk => data += chunk);
          redirectResponse.on('end', () => resolve(data.trim()));
        }).on('error', () => resolve(null));
      } else if (response.statusCode === 200) {
        let data = '';
        response.on('data', chunk => data += chunk);
        response.on('end', () => resolve(data.trim()));
      } else {
        resolve(null); // Checksum not available, skip verification
      }
    }).on('error', () => resolve(null));
  });
}

function calculateSha256(filePath) {
  const crypto = require('crypto');
  const data = fs.readFileSync(filePath);
  return crypto.createHash('sha256').update(data).digest('hex');
}

function verifyChecksum(filePath, expectedChecksum) {
  if (!expectedChecksum) {
    console.log('Checksum verification skipped (no checksum available)');
    return true;
  }

  const actualChecksum = calculateSha256(filePath);
  const expected = expectedChecksum.split(' ')[0].toLowerCase();

  if (actualChecksum !== expected) {
    throw new Error(`Checksum verification failed. Expected: ${expected}, Got: ${actualChecksum}`);
  }

  console.log('Checksum verification passed');
  return true;
}

async function install() {
  console.log('Installing Ralph CLI...');

  // Check for offline installation via environment variable
  const offlinePath = process.env.RALPH_BINARY_PATH;
  if (offlinePath) {
    console.log(`Using offline binary at: ${offlinePath}`);
    const binDir = path.join(__dirname, 'bin');
    if (!fs.existsSync(binDir)) {
      fs.mkdirSync(binDir, { recursive: true });
    }
    const destPath = path.join(binDir, BINARY_NAME);
    fs.copyFileSync(offlinePath, destPath);
    fs.chmodSync(destPath, 0o755);
    console.log('Offline installation complete!');
    return;
  }

  // Check for offline installation via npm config
  const npmConfigPath = process.env.npm_config_ralph_binary_path;
  if (npmConfigPath) {
    console.log(`Using offline binary at: ${npmConfigPath}`);
    const binDir = path.join(__dirname, 'bin');
    if (!fs.existsSync(binDir)) {
      fs.mkdirSync(binDir, { recursive: true });
    }
    const destPath = path.join(binDir, BINARY_NAME);
    fs.copyFileSync(npmConfigPath, destPath);
    fs.chmodSync(destPath, 0o755);
    console.log('Offline installation complete!');
    return;
  }

  try {
    const { target } = getPlatform();
    console.log(`Detected platform: ${target}`);

    const binDir = path.join(__dirname, 'bin');
    if (!fs.existsSync(binDir)) {
      fs.mkdirSync(binDir, { recursive: true });
    }

    const binaryUrl = getBinaryUrl(target);
    const binaryPath = path.join(binDir, BINARY_NAME);

    console.log(`Downloading binary from: ${binaryUrl}`);
    await downloadFile(binaryUrl, binaryPath);

    // Download and verify checksum
    console.log('Verifying checksum...');
    const checksum = await downloadChecksum(target);
    verifyChecksum(binaryPath, checksum);

    // Make binary executable
    fs.chmodSync(binaryPath, 0o755);

    console.log('Ralph CLI installed successfully!');
    console.log(`Binary location: ${binaryPath}`);
  } catch (error) {
    console.error('Installation failed:', error.message);
    console.error('\nPossible solutions:');
    console.error('1. Check your internet connection');
    console.error('2. Verify that the release exists on GitHub');
    console.error('3. For offline installation, use: npm install ralph-cli --ralph-binary-path=/path/to/ralph');
    console.error('   Or set environment variable: RALPH_BINARY_PATH=/path/to/ralph npm install ralph-cli');
    process.exit(1);
  }
}

install();
