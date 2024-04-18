// globals.d.ts
declare global {
  interface Window {
    os: "windows" | "linux";
    workspace: boolean;
  }
}
