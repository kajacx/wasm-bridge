(() => ({
  //   wasi_snapshot_preview1: {
  //     clock_time_get: (a, b, c, d) => {
  //       console.log("HOW????", a, b, c, d);
  //       return 0;
  //     },
  //     fd_write: () => {},
  //     environ_get: () => [],
  //     environ_sizes_get: () => [],
  //     proc_exit: () => {},
  //   },
  env: {},
  "wasi:filesystem/preopens@0.2.0-rc-2023-11-10": {
    "get-directories": () => [],
  },
  //   "wasi:clocks/wall-clock@0.2.0-rc-2023-11-10": {},
  //   "wasi:clocks/monotonic-clock@0.2.0-rc-2023-11-10": {},
  "wasi:filesystem/types@0.2.0-rc-2023-11-10": {
    "[method]descriptor.append-via-stream": () => {},
    "[method]descriptor.get-type": () => {},
    "[method]descriptor.write-via-stream": () => {},
    "[resource-drop]descriptor": () => {},
    "filesystem-error-code": () => {},
  },
  "wasi:io/error@0.2.0-rc-2023-11-10": {
    "[resource-drop]error": () => {},
  },
  "wasi:io/streams@0.2.0-rc-2023-11-10": {
    "[method]output-stream.blocking-flush": () => {},
    "[method]output-stream.blocking-write-and-flush": () => {},
    "[method]output-stream.check-write": () => {},
    "[method]output-stream.write": () => {},
    "[resource-drop]input-stream": () => {},
    "[resource-drop]output-stream": () => {},
  },
  __main_module__: {},
  "wasi:cli/environment@0.2.0-rc-2023-11-10": {
    "get-environment": () => {},
  },
  "wasi:sockets/tcp@0.2.0-rc-2023-11-10": {
    "[resource-drop]tcp-socket": () => {},
  },
  "wasi:cli/exit@0.2.0-rc-2023-11-10": {
    exit: () => {},
  },
  "wasi:cli/stdin@0.2.0-rc-2023-11-10": {
    "get-stdin": () => {},
  },
  "wasi:cli/stdout@0.2.0-rc-2023-11-10": {
    "get-stdout": () => {},
  },
  "wasi:cli/stderr@0.2.0-rc-2023-11-10": {
    "get-stderr": () => {},
  },
  "wasi:cli/terminal-stdin@0.2.0-rc-2023-11-10": {
    "get-terminal-stdin": () => {},
  },
  "wasi:cli/terminal-stdout@0.2.0-rc-2023-11-10": {
    "get-terminal-stdout": () => {},
  },
  "wasi:cli/terminal-stderr@0.2.0-rc-2023-11-10": {
    "get-terminal-stderr": () => {},
  },
  "wasi:cli/terminal-input@0.2.0-rc-2023-11-10": {
    "[resource-drop]terminal-input": () => {},
  },
  "wasi:cli/terminal-output@0.2.0-rc-2023-11-10": {
    "[resource-drop]terminal-output": () => {},
  },
}))();
