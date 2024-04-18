import { getServer } from "./workspace";

export async function getCorePackages() {
  fetch(`${getServer()}/base_pkg`);
}