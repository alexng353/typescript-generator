#! /usr/bin/env node

const os = require('os');
console.log(os.platform());
const child = require("child_process");

const platform = os.platform();

switch (platform) {
  case 'win32':
    child.spawn(`${__dirname}/typescript-generator.exe`, process.argv.slice(2), {
      stdio: "inherit",
    });
    break;
  case 'darwin':
  case 'linux':
    child.spawn(`${__dirname}/typescript-generator`, process.argv.slice(2), {
      stdio: "inherit",
    });
    break;
  default:
    console.log('Unsupported platform');
    break;
}

