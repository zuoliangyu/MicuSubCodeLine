#!/usr/bin/env node
const { spawnSync } = require('child_process');
const path = require('path');
const fs = require('fs');
const os = require('os');

// 1. Priority: Use ~/.claude/micusubcodeline/micusubcodeline if exists
const claudePath = path.join(
  os.homedir(),
  '.claude',
  'micusubcodeline',
  process.platform === 'win32' ? 'micusubcodeline.exe' : 'micusubcodeline'
);

if (fs.existsSync(claudePath)) {
  const result = spawnSync(claudePath, process.argv.slice(2), {
    stdio: 'inherit',
    shell: false
  });
  process.exit(result.status || 0);
}

// 2. Fallback: Use npm package binary
const platform = process.platform;
const arch = process.arch;

let platformKey = `${platform}-${arch}`;
if (platform === 'linux') {
  function shouldUseStaticBinary() {
    try {
      const { execSync } = require('child_process');
      const lddOutput = execSync('ldd --version 2>/dev/null || echo ""', {
        encoding: 'utf8',
        timeout: 1000
      });
      const match = lddOutput.match(/(?:GNU libc|GLIBC).*?(\d+)\.(\d+)/);
      if (match) {
        const major = parseInt(match[1]);
        const minor = parseInt(match[2]);
        return major < 2 || (major === 2 && minor < 35);
      }
    } catch (e) {
      return false;
    }
    return false;
  }
  if (shouldUseStaticBinary()) {
    platformKey = 'linux-x64-musl';
  }
}

const packageMap = {
  'darwin-x64': '@zuolan/micusubcodeline-darwin-x64',
  'darwin-arm64': '@zuolan/micusubcodeline-darwin-arm64',
  'linux-x64': '@zuolan/micusubcodeline-linux-x64',
  'linux-x64-musl': '@zuolan/micusubcodeline-linux-x64-musl',
  'win32-x64': '@zuolan/micusubcodeline-win32-x64',
  'win32-ia32': '@zuolan/micusubcodeline-win32-x64',
};

const packageName = packageMap[platformKey];
if (!packageName) {
  console.error(`Error: Unsupported platform ${platformKey}`);
  process.exit(1);
}

const binaryName = platform === 'win32' ? 'micusubcodeline.exe' : 'micusubcodeline';
const binaryPath = path.join(__dirname, '..', 'node_modules', packageName, binaryName);

if (!fs.existsSync(binaryPath)) {
  console.error(`Error: Binary not found at ${binaryPath}`);
  console.error('Please try reinstalling: npm install -g @zuolan/micusubcodeline');
  process.exit(1);
}

const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  shell: false
});

process.exit(result.status || 0);
