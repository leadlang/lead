export function isWorkspace() {
  return (window as unknown as { workspace?: boolean }).workspace || false
}

export function version() {
  const win = window as unknown as { [key: string]: string };
  return `Lead Lang Docs v${win.leadver} ${win.os}_${win.arch}`
}

export function getServer() {
  const win = (window as unknown as { os?: string }).os || "windows";

  if (win == "windows") {
    return "http://api.localhost"
  } else {
    return "api://localhost"
  }
}