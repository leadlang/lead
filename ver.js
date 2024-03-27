const { readFileSync, writeFileSync } = require("fs");

const version = readFileSync("./.version").toString();

const values = [
  "./Cargo.toml",
  "./lead/Cargo.toml",
  "./lead_init/Cargo.toml",
  "./interpreter/Cargo.toml",
];

for (const value of values) {
  const file = readFileSync(value).toString();
  const parsed = file.replace("0.0.0-dev-lead-lang", version);

  writeFileSync(value, parsed);
}
