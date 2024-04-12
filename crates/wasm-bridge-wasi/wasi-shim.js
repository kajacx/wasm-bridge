(() => ({
  "wasi:filesystem/types@0.2.0": {
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
  },
  "wasi:io/streams@0.2.0": {
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
  "wasi:sockets/tcp@0.2.0": {
    "[resource-drop]tcp-socket": (...args) =>
      console.log("Calling [resource-drop]tcp-socket", ...args),
  },
  "wasi:cli/terminal-input@0.2.0": {
    "[resource-drop]terminal-input": (...args) =>
      console.log("Calling [resource-drop]terminal-input", ...args),
  },
  "wasi:cli/terminal-output@0.2.0": {
    "[resource-drop]terminal-output": (...args) =>
      console.log("Calling [resource-drop]terminal-output", ...args),
  },
}))();
