(() => ({
  "wasi:filesystem/preopens@0.2.0-rc-2023-11-10": {
    "get-directories": (...args) =>
      console.log("Calling get-directories", ...args),
  },
  "wasi:filesystem/types@0.2.0-rc-2023-11-10": {
    "[method]descriptor.append-via-stream": (...args) =>
      console.log("Calling [method]descriptor.append-via-stream", ...args),
    "[method]descriptor.get-type": (...args) =>
      console.log("Calling [method]descriptor.get-type", ...args),
    "[method]descriptor.write-via-stream": (...args) =>
      console.log("Calling [method]descriptor.write-via-stream", ...args),
    "[method]descriptor.read-via-stream": (...args) =>
      console.log("Calling [method]descriptor.read-via-stream", ...args),
    "[resource-drop]descriptor": (...args) =>
      console.log("Calling [resource-drop]descriptor", ...args),
    "filesystem-error-code": (...args) =>
      console.log("Calling filesystem-error-code", ...args),
  },
  "wasi:io/error@0.2.0-rc-2023-11-10": {
    "[resource-drop]error": (...args) =>
      console.log("Calling [resource-drop]error", ...args),
  },
  "wasi:io/streams@0.2.0-rc-2023-11-10": {
    "[method]output-stream.blocking-flush": (...args) =>
      console.log("Calling [method]output-stream.blocking-flush", ...args),
    "[method]output-stream.check-write": (...args) =>
      console.log("Calling [method]output-stream.check-write", ...args),
    "[method]output-stream.write": (...args) =>
      console.log("Calling [method]output-stream.write", ...args),
    "[method]input-stream.read": (...args) =>
      console.log("Calling [method]input-stream.read", ...args),
    "[resource-drop]input-stream": (...args) =>
      console.log("Calling [resource-drop]input-stream", ...args),
    "[resource-drop]output-stream": (...args) =>
      console.log("Calling [resource-drop]output-stream", ...args),
  },
  "wasi:cli/environment@0.2.0-rc-2023-11-10": {
    "get-environment": (...args) =>
      console.log("Calling get-environment", ...args),
  },
  "wasi:sockets/tcp@0.2.0-rc-2023-11-10": {
    "[resource-drop]tcp-socket": (...args) =>
      console.log("Calling [resource-drop]tcp-socket", ...args),
  },
  "wasi:cli/exit@0.2.0-rc-2023-11-10": {
    exit: (...args) => console.log("Calling exit", ...args),
  },
  "wasi:cli/terminal-input@0.2.0-rc-2023-11-10": {
    "[resource-drop]terminal-input": (...args) =>
      console.log("Calling [resource-drop]terminal-input", ...args),
  },
  "wasi:cli/terminal-output@0.2.0-rc-2023-11-10": {
    "[resource-drop]terminal-output": (...args) =>
      console.log("Calling [resource-drop]terminal-output", ...args),
  },
}))();
