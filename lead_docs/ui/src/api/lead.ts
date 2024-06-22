import { getServer } from "./workspace";

type String = string;

export interface Package {
  name: String,
  modules: Module[]
}

export interface Module {
  name: String,
  own: String,
  methods: Method[],
}

export interface Method {
  name: String,
  desc: String,
}

export async function getCorePackages() {
  return await fetch(`${getServer()}/base_pkg`).then((d) => d.json() as Promise<Package[]>).catch(() => ([] as Package[]))
}