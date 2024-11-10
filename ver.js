const { readFileSync, writeFileSync } = require("fs");

const date = new Date();
const version = `${date.getFullYear()}.${
  date.getMonth() + 1
}.${date.getDate()}`;

const values = [
  "./Cargo.toml",
  "./lead/Cargo.toml",
  "./lead_init/Cargo.toml",
  "./lead_docs/Cargo.toml",
  "./packages/core/Cargo.toml",
  "./packages/std/Cargo.toml",
  "./interpreter/Cargo.toml",
];

for (const value of values) {
  const file = readFileSync(value).toString();
  const parsed = file.replace('"0.0.0-dev-lead-lang"', '"' + version + '"');

  writeFileSync(value, parsed);
}

writeFileSync("./.version", version);
