#! /usr/bin/env node

const os = require("os");
const child = require("child_process");

const platform = os.platform();

switch (platform) {
  case "win32":
    child.spawn(
      `${__dirname}/typescript-generator.exe`,
      process.argv.slice(2),
      {
        stdio: "inherit",
      }
    );
    break;
  case "darwin":
    child.spawn(
      `${__dirname}/typescript-generator-M1-osx`,
      process.argv.slice(2),
      {
        stdio: "inherit",
      }
    );
    break;
  case "linux":
    child.spawn(`${__dirname}/typescript-generator`, process.argv.slice(2), {
      stdio: "inherit",
    });
    break;
  default:
    console.log("Unsupported platform");
    break;
}
