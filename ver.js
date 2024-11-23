const { readFileSync, writeFileSync } = require("fs");

const suffix = readFileSync("./suffix").toString();
writeFileSync("./suffix", "");

const date = new Date();
const version =
  `${date.getFullYear()}.${date.getMonth() + 1}.${date.getDate() - 1}` +
  (process.env.NIGHTLY == "true" ? `-nightly.${Date.now()}` : "") +
  suffix;

const values = [
  "./Cargo.toml",
  "./lead/Cargo.toml",
  "./leadman/Cargo.toml",
  "./lead_docs/Cargo.toml",
  "./packages/core/Cargo.toml",
  "./packages/std/Cargo.toml",
  "./interpreter/Cargo.toml",
  "./macros/Cargo.toml",
];

for (const value of values) {
  const file = readFileSync(value).toString();
  const parsed = file.replaceAll('"0.0.0-dev-lead-lang"', '"' + version + '"');

  writeFileSync(value, parsed);
}

writeFileSync("./.version", version);
