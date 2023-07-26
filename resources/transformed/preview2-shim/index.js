import importObject from "./browser";

export function getWasiImports() {
  let exports = { ...importObject, "cli-base": importObject.cliBase };

  let wasiExports = {};

  for (let package_name in exports) {
    for (let export_name in exports[package_name]) {
      let export_name_tr = export_name;
      if (export_name == "monotonicClock") {
        export_name_tr = "monotonic-clock";
      }
      if (export_name == "wallClock") {
        export_name_tr = "wall-clock";
      }

      wasiExports[`wasi:${package_name}/${export_name_tr}`] =
        exports[package_name][export_name];
    }
  }

  return wasiExports;
}
