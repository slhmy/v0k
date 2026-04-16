const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');
const https = require('https');

const { version } = require('./package.json');

const BINARY_NAME = 'v0k';
const GITHUB_REPO = 'slhmy/v0k';

// Determine the target architecture and operating system
let rustTarget = '';
const platform = os.platform();
const arch = os.arch();

if (platform === 'linux' && arch === 'x64') {
  rustTarget = 'x86_64-unknown-linux-gnu';
} else if (platform === 'darwin' && arch === 'x64') {
  rustTarget = 'x86_64-apple-darwin';
} else if (platform === 'darwin' && arch === 'arm64') {
  rustTarget = 'aarch64-apple-darwin';
} else if (platform === 'win32' && arch === 'x64') {
  rustTarget = 'x86_64-pc-windows-msvc.exe';
} else {
  console.error(`Unsupported platform/architecture: ${platform}-${arch}`);
  process.exit(1);
}

const artifactName = `v0k-${rustTarget}`;
const downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/v${version}/${artifactName}`;

const binDir = path.join(__dirname, 'bin');
const destFile = path.join(binDir, platform === 'win32' ? `${BINARY_NAME}.exe` : BINARY_NAME);

if (!fs.existsSync(binDir)) {
  fs.mkdirSync(binDir, { recursive: true });
}

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);

    // Follow redirects
    const attemptDownload = (url) => {
      https.get(url, (response) => {
        if (response.statusCode >= 300 && response.statusCode < 400 && response.headers.location) {
          attemptDownload(response.headers.location);
        } else if (response.statusCode === 200) {
          response.pipe(file);
          file.on('finish', () => {
            file.close(resolve);
          });
        } else {
          file.close();
          fs.unlink(dest, () => reject(new Error(`Failed with status code: ${response.statusCode}`)));
        }
      }).on('error', (err) => {
        file.close();
        fs.unlink(dest, () => reject(err));
      });
    };

    attemptDownload(url);
  });
}

async function main() {
  console.log(`Downloading ${BINARY_NAME} v${version} for ${platform}-${arch}...`);
  console.log(`URL: ${downloadUrl}`);

  try {
    await download(downloadUrl, destFile);
    console.log(`Successfully downloaded ${BINARY_NAME} to ${destFile}`);

    // Make the file executable on unix
    if (platform !== 'win32') {
      fs.chmodSync(destFile, 0o755);
    }
  } catch (err) {
    console.error(`Failed to download ${BINARY_NAME}: ${err.message}`);
    console.error('If you are building locally, you can ignore this error and build via cargo.');
    // Don't fail the installation entirely so local development still works without releases
    process.exit(0);
  }
}

main();