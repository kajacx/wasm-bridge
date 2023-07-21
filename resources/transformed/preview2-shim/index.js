import importObject from "./browser";

export function getWasiImports() {
  let exports = { ...importObject, "cli-base": importObject.cliBase };

  let wasiExports = {};

  for (let package_name in exports) {
    for (let export_name in exports[package_name]) {
      wasiExports[`wasi:${package_name}/${export_name}`] =
        exports[package_name][export_name];
    }
  }

  return wasiExports;
}
