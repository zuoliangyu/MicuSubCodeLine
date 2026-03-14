const fs = require('fs');
const path = require('path');
const os = require('os');

const silent = process.env.npm_config_loglevel === 'silent' ||
               process.env.MICUSUBCODELINE_SKIP_POSTINSTALL === '1';

if (!silent) {
  console.log('🚀 Setting up MicuSubCodeLine for Claude Code...');
}

try {
  const platform = process.platform;
  const arch = process.arch;
  const homeDir = os.homedir();
  const claudeDir = path.join(homeDir, '.claude', 'micusubcodeline');

  fs.mkdirSync(claudeDir, { recursive: true });

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
    if (!silent) {
      console.log(`Platform ${platformKey} not supported for auto-setup`);
    }
    process.exit(0);
  }

  const binaryName = platform === 'win32' ? 'micusubcodeline.exe' : 'micusubcodeline';
  const targetPath = path.join(claudeDir, binaryName);

  const findBinaryPath = () => {
    const possiblePaths = [
      path.join(__dirname, '..', 'node_modules', packageName, binaryName),
      (() => {
        try {
          const packagePath = require.resolve(packageName + '/package.json');
          return path.join(path.dirname(packagePath), binaryName);
        } catch {
          return null;
        }
      })(),
      (() => {
        const currentPath = __dirname;
        const pnpmMatch = currentPath.match(/(.+\.pnpm)[\\/]([^\\//]+)[\\/]/);
        if (pnpmMatch) {
          const pnpmRoot = pnpmMatch[1];
          const packageNameEncoded = packageName.replace('/', '+');
          try {
            const pnpmContents = fs.readdirSync(pnpmRoot);
            const packagePattern = new RegExp(`^${packageNameEncoded.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}@`);
            const matchingPackage = pnpmContents.find(dir => packagePattern.test(dir));
            if (matchingPackage) {
              return path.join(pnpmRoot, matchingPackage, 'node_modules', packageName, binaryName);
            }
          } catch {
            // ignore
          }
        }
        return null;
      })()
    ].filter(p => p !== null);

    for (const testPath of possiblePaths) {
      if (fs.existsSync(testPath)) {
        return testPath;
      }
    }
    return null;
  };

  const sourcePath = findBinaryPath();
  if (!sourcePath) {
    if (!silent) {
      console.log('Binary package not installed, skipping Claude Code setup');
      console.log('The global micusubcodeline command will still work via npm');
    }
    process.exit(0);
  }

  if (platform === 'win32') {
    fs.copyFileSync(sourcePath, targetPath);
  } else {
    try {
      if (fs.existsSync(targetPath)) {
        fs.unlinkSync(targetPath);
      }
      fs.linkSync(sourcePath, targetPath);
    } catch {
      fs.copyFileSync(sourcePath, targetPath);
    }
    fs.chmodSync(targetPath, '755');
  }

  if (!silent) {
    console.log('✨ MicuSubCodeLine is ready for Claude Code!');
    console.log(`📍 Location: ${targetPath}`);
    console.log('');
  }

  try {
    const { spawnSync } = require('child_process');
    const result = spawnSync(targetPath, ['--init'], {
      stdio: silent ? 'ignore' : 'inherit',
      shell: false,
      timeout: 10000
    });
    if (!silent && result.status === 0) {
      console.log('✅ Configuration initialized!');
    }
  } catch (error) {
    if (!silent) {
      console.log('⚠️  Could not auto-initialize. Run manually: micusubcodeline --init');
    }
  }

  if (!silent) {
    console.log('🎉 You can now use: micusubcodeline --help');
  }
} catch (error) {
  if (!silent) {
    console.log('Note: Could not auto-configure for Claude Code');
    console.log('The global micusubcodeline command will still work.');
  }
}
