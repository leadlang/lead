import { parse } from "yaml";
import { readFileSync, writeFileSync } from "fs";
import { join } from "path";

const data = parse(
  readFileSync(
    join(import.meta.dirname, "../../.github/workflows/publish.yml")
  ).toString()
);

const jobs = data.jobs;

const t1 = jobs.build.strategy.matrix.include
  .map(({ target }) => target)
  .join("\n");

const t2 = jobs["build-cross"].strategy.matrix.target.join("\n");

const targets = `${t1}\n${t2}`;

writeFileSync("./targets.txt", targets);
