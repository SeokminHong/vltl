import { appendFile, readFile, writeFile } from "node:fs/promises";

const [bumpType] = process.argv.slice(2);

if (!["major", "minor", "patch"].includes(bumpType)) {
  console.error(`Invalid bump type: ${bumpType}`);
  process.exit(1);
}

const cargoToml = await readFile("Cargo.toml", "utf-8");
const match = cargoToml.match(/^version = "(.+)"$/m);

if (!match) {
  console.error("Could not find version in Cargo.toml");
  process.exit(1);
}

const current = match[1];
const parts = current.split(".");
if (parts.length !== 3 || parts.some((p) => Number.isNaN(Number(p)))) {
  console.error(`Invalid version format: ${current}`);
  process.exit(1);
}
const [major, minor, patch] = parts.map(Number);

const newVersion = {
  major: `${major + 1}.0.0`,
  minor: `${major}.${minor + 1}.0`,
  patch: `${major}.${minor}.${patch + 1}`,
}[bumpType];

const updated = cargoToml.replace(
  /^version = ".+"$/m,
  `version = "${newVersion}"`,
);
await writeFile("Cargo.toml", updated);

const githubOutput = process.env.GITHUB_OUTPUT;
if (githubOutput) {
  await appendFile(githubOutput, `version=${newVersion}\n`);
}

console.log(`${current} -> ${newVersion}`);
