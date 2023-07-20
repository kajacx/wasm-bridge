import importObject from "./browser";

export function getWasiImports() {
  return { ...importObject, "cli-base": importObject.cliBase };
}
