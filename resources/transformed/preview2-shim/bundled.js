(() => {
  var __defProp = Object.defineProperty;
  var __export = (target, all) => {
    for (var name in all)
      __defProp(target, name, { get: all[name], enumerable: true });
  };

  // browser/clocks.js
  var clocks_exports = {};
  __export(clocks_exports, {
    monotonicClock: () => monotonicClock,
    timezone: () => timezone,
    wallClock: () => wallClock
  });
  function _hrtimeBigint() {
    return BigInt(Math.floor(performance.now() * 1e9));
  }
  var _hrStart = _hrtimeBigint();
  var monotonicClock = {
    resolution() {
      return 1n;
    },
    now() {
      return _hrtimeBigint() - _hrStart;
    },
    subscribe(_when, _absolute) {
      console.log(`[monotonic-clock] Subscribe`);
    }
  };
  var timezone = {
    display(timezone2, when) {
      console.log(`[timezone] DISPLAY ${timezone2} ${when}`);
    },
    utcOffset(timezone2, when) {
      console.log(`[timezone] UTC OFFSET ${timezone2} ${when}`);
      return 0;
    },
    dropTimezone(timezone2) {
      console.log(`[timezone] DROP ${timezone2}`);
    }
  };
  var wallClock = {
    now() {
      const seconds = BigInt(Math.floor(Date.now() / 1e3));
      const nanoseconds = Date.now() % 1e3 * 1e6;
      return { seconds, nanoseconds };
    },
    resolution() {
      console.log(`[wall-clock] Wall clock resolution`);
    }
  };

  // browser/filesystem.js
  var filesystem_exports = {};
  __export(filesystem_exports, {
    filesystem: () => filesystem,
    filesystemFilesystem: () => filesystem
  });
  var filesystem = {
    readViaStream(fd, offset) {
      console.log(`[filesystem] READ STREAM ${fd} ${offset}`);
    },
    writeViaStream(fd, offset) {
      console.log(`[filesystem] WRITE STREAM ${fd} ${offset}`);
    },
    appendViaStream(fd) {
      console.log(`[filesystem] APPEND STREAM ${fd}`);
    },
    advise(fd, offset, length, advice) {
      console.log(`[filesystem] ADVISE`, fd, offset, length, advice);
    },
    syncData(fd) {
      console.log(`[filesystem] SYNC DATA ${fd}`);
    },
    getFlags(fd) {
      console.log(`[filesystem] FLAGS FOR ${fd}`);
    },
    getType(fd) {
      console.log(`[filesystem] GET TYPE ${fd}`);
    },
    setFlags(fd, flags) {
      console.log(`[filesystem] SET FLAGS ${fd} ${JSON.stringify(flags)}`);
    },
    setSize(fd, size) {
      console.log(`[filesystem] SET SIZE`, fd, size);
    },
    setTimes(fd, dataAccessTimestamp, dataModificationTimestamp) {
      console.log(`[filesystem] SET TIMES`, fd, dataAccessTimestamp, dataModificationTimestamp);
    },
    read(fd, length, offset) {
      console.log(`[filesystem] READ`, fd, length, offset);
    },
    write(fd, buffer, offset) {
      console.log(`[filesystem] WRITE`, fd, buffer, offset);
    },
    readDirectory(fd) {
      console.log(`[filesystem] READ DIR`, fd);
    },
    sync(fd) {
      console.log(`[filesystem] SYNC`, fd);
    },
    createDirectoryAt(fd, path) {
      console.log(`[filesystem] CREATE DIRECTORY`, fd, path);
    },
    stat(fd) {
      console.log(`[filesystem] STAT`, fd);
    },
    statAt(fd, pathFlags, path) {
      console.log(`[filesystem] STAT`, fd, pathFlags, path);
    },
    setTimesAt(fd) {
      console.log(`[filesystem] SET TIMES AT`, fd);
    },
    linkAt(fd) {
      console.log(`[filesystem] LINK AT`, fd);
    },
    openAt(fd) {
      console.log(`[filesystem] OPEN AT ${fd}`);
    },
    readlinkAt(fd) {
      console.log(`[filesystem] READLINK AT`, fd);
    },
    removeDirectoryAt(fd) {
      console.log(`[filesystem] REMOVE DIR AT`, fd);
    },
    renameAt(fd) {
      console.log(`[filesystem] RENAME AT`, fd);
    },
    symlinkAt(fd) {
      console.log(`[filesystem] SYMLINK AT`, fd);
    },
    unlinkFileAt(fd) {
      console.log(`[filesystem] UNLINK FILE AT`, fd);
    },
    changeFilePermissionsAt(fd) {
      console.log(`[filesystem] CHANGE FILE PERMISSIONS AT`, fd);
    },
    changeDirectoryPermissionsAt(fd) {
      console.log(`[filesystem] CHANGE DIR PERMISSIONS AT`, fd);
    },
    lockShared(fd) {
      console.log(`[filesystem] LOCK SHARED`, fd);
    },
    lockExclusive(fd) {
      console.log(`[filesystem] LOCK EXCLUSIVE`, fd);
    },
    tryLockShared(fd) {
      console.log(`[filesystem] TRY LOCK SHARED`, fd);
    },
    tryLockExclusive(fd) {
      console.log(`[filesystem] TRY LOCK EXCLUSIVE`, fd);
    },
    unlock(fd) {
      console.log(`[filesystem] UNLOCK`, fd);
    },
    dropDescriptor(fd) {
      console.log(`[filesystem] DROP DESCRIPTOR`, fd);
    },
    readDirectoryEntry(stream) {
      console.log(`[filesystem] READ DIRECTRY ENTRY`, stream);
    },
    dropDirectoryEntryStream(stream) {
      console.log(`[filesystem] DROP DIRECTORY ENTRY`, stream);
    }
  };

  // browser/http.js
  var http_exports = {};
  __export(http_exports, {
    incomingHandler: () => incomingHandler,
    outgoingHandler: () => outgoingHandler,
    send: () => send,
    types: () => types
  });

  // http/error.js
  var UnexpectedError = class extends Error {
    /** @type { import("../types/http").HttpErrorUnexpectedError } */
    payload;
    constructor(message = "unexpected-error") {
      super(message);
      this.payload = {
        tag: "unexpected-error",
        val: message
      };
    }
  };

  // browser/http.js
  function send(req) {
    console.log(`[http] Send (browser) ${req.uri}`);
    try {
      const xhr = new XMLHttpRequest();
      xhr.open(req.method.toString(), req.uri, false);
      const requestHeaders = new Headers(req.headers);
      for (let [name, value] of requestHeaders.entries()) {
        if (name !== "user-agent" && name !== "host") {
          xhr.setRequestHeader(name, value);
        }
      }
      xhr.send(req.body && req.body.length > 0 ? req.body : null);
      const body = xhr.response ? new TextEncoder().encode(xhr.response) : void 0;
      const headers = [];
      xhr.getAllResponseHeaders().trim().split(/[\r\n]+/).forEach((line) => {
        var parts = line.split(": ");
        var key = parts.shift();
        var value = parts.join(": ");
        headers.push([key, value]);
      });
      return {
        status: xhr.status,
        headers,
        body
      };
    } catch (err) {
      throw new UnexpectedError(err.message);
    }
  }
  var incomingHandler = {
    handle() {
    }
  };
  var outgoingHandler = {
    handle() {
    }
  };
  var types = {
    dropFields(_fields) {
      console.log("[types] Drop fields");
    },
    newFields(_entries) {
      console.log("[types] New fields");
    },
    fieldsGet(_fields, _name) {
      console.log("[types] Fields get");
    },
    fieldsSet(_fields, _name, _value) {
      console.log("[types] Fields set");
    },
    fieldsDelete(_fields, _name) {
      console.log("[types] Fields delete");
    },
    fieldsAppend(_fields, _name, _value) {
      console.log("[types] Fields append");
    },
    fieldsEntries(_fields) {
      console.log("[types] Fields entries");
    },
    fieldsClone(_fields) {
      console.log("[types] Fields clone");
    },
    finishIncomingStream(s) {
      console.log(`[types] Finish incoming stream ${s}`);
    },
    finishOutgoingStream(s, _trailers) {
      console.log(`[types] Finish outgoing stream ${s}`);
    },
    dropIncomingRequest(_req) {
      console.log("[types] Drop incoming request");
    },
    dropOutgoingRequest(_req) {
      console.log("[types] Drop outgoing request");
    },
    incomingRequestMethod(_req) {
      console.log("[types] Incoming request method");
    },
    incomingRequestPath(_req) {
      console.log("[types] Incoming request path");
    },
    incomingRequestQuery(_req) {
      console.log("[types] Incoming request query");
    },
    incomingRequestScheme(_req) {
      console.log("[types] Incoming request scheme");
    },
    incomingRequestAuthority(_req) {
      console.log("[types] Incoming request authority");
    },
    incomingRequestHeaders(_req) {
      console.log("[types] Incoming request headers");
    },
    incomingRequestConsume(_req) {
      console.log("[types] Incoming request consume");
    },
    newOutgoingRequest(_method, _path, _query, _scheme, _authority, _headers) {
      console.log("[types] New outgoing request");
    },
    outgoingRequestWrite(_req) {
      console.log("[types] Outgoing request write");
    },
    dropResponseOutparam(_res) {
      console.log("[types] Drop response outparam");
    },
    setResponseOutparam(_response) {
      console.log("[types] Drop fields");
    },
    dropIncomingResponse(_res) {
      console.log("[types] Drop incoming response");
    },
    dropOutgoingResponse(_res) {
      console.log("[types] Drop outgoing response");
    },
    incomingResponseStatus(_res) {
      console.log("[types] Incoming response status");
    },
    incomingResponseHeaders(_res) {
      console.log("[types] Incoming response headers");
    },
    incomingResponseConsume(_res) {
      console.log("[types] Incoming response consume");
    },
    newOutgoingResponse(_statusCode, _headers) {
      console.log("[types] New outgoing response");
    },
    outgoingResponseWrite(_res) {
      console.log("[types] Outgoing response write");
    },
    dropFutureIncomingResponse(_f) {
      console.log("[types] Drop future incoming response");
    },
    futureIncomingResponseGet(_f) {
      console.log("[types] Future incoming response get");
    },
    listenToFutureIncomingResponse(_f) {
      console.log("[types] Listen to future incoming response");
    }
  };

  // browser/io.js
  var io_exports = {};
  __export(io_exports, {
    streams: () => streams
  });
  var streams = {
    read(s, len) {
      console.log(`[streams] Read ${s} ${len}`);
    },
    blockingRead(s, len) {
      console.log(`[streams] Blocking read ${s} ${len}`);
    },
    skip(s, _len) {
      console.log(`[streams] Skip ${s}`);
    },
    blockingSkip(s, _len) {
      console.log(`[streams] Blocking skip ${s}`);
    },
    subscribeToInputStream(s) {
      console.log(`[streams] Subscribe to input stream ${s}`);
    },
    dropInputStream(s) {
      console.log(`[streams] Drop input stream ${s}`);
    },
    write(s, buf) {
      switch (s) {
        case 0:
          throw new Error(`TODO: write stdin`);
        case 1: {
          const decoder = new TextDecoder();
          console.log(decoder.decode(buf));
          return BigInt(buf.byteLength);
        }
        case 2: {
          const decoder = new TextDecoder();
          console.error(decoder.decode(buf));
          return BigInt(buf.byteLength);
        }
        default:
          throw new Error(`TODO: write ${s}`);
      }
    },
    blockingWrite(s, _buf) {
      console.log(`[streams] Blocking write ${s}`);
    },
    writeZeroes(s, _len) {
      console.log(`[streams] Write zeroes ${s}`);
    },
    blockingWriteZeroes(s, _len) {
      console.log(`[streams] Blocking write zeroes ${s}`);
    },
    splice(s, _src, _len) {
      console.log(`[streams] Splice ${s}`);
    },
    blockingSplice(s, _src, _len) {
      console.log(`[streams] Blocking splice ${s}`);
    },
    forward(s, _src) {
      console.log(`[streams] Forward ${s}`);
    },
    subscribeToOutputStream(s) {
      console.log(`[streams] Subscribe to output stream ${s}`);
    },
    dropOutputStream(s) {
      console.log(`[streams] Drop output stream ${s}`);
    }
  };

  // browser/logging.js
  var logging_exports = {};
  __export(logging_exports, {
    handler: () => handler,
    setLevel: () => setLevel
  });
  var levels = ["trace", "debug", "info", "warn", "error"];
  var logLevel = levels.indexOf("warn");
  var handler = {
    log(level, context, msg) {
      if (logLevel > levels.indexOf(level))
        return;
      console[level](`(${context}) ${msg}
`);
    }
  };
  function setLevel(level) {
    logLevel = levels.indexOf(level);
  }

  // browser/poll.js
  var poll_exports = {};
  __export(poll_exports, {
    poll: () => poll
  });
  var poll = {
    dropPollable(pollable) {
      console.log(`[poll] Drop (${pollable})`);
    },
    pollOneoff(input) {
      console.log(`[poll] Oneoff (${input})`);
      return [];
    }
  };

  // browser/random.js
  var random_exports = {};
  __export(random_exports, {
    insecure: () => insecure,
    insecureSeed: () => insecureSeed,
    random: () => random
  });
  var insecure = {
    getInsecureRandomBytes(len) {
      return random.getRandomBytes(len);
    },
    getInsecureRandomU64() {
      return random.getRandomU64();
    }
  };
  var insecureSeedValue1;
  var insecureSeedValue2;
  var insecureSeed = {
    insecureSeed() {
      if (insecureSeedValue1 === void 0) {
        insecureSeedValue1 = random.getRandomU64();
        insecureSeedValue2 = random.getRandomU64();
      }
      return [insecureSeedValue1, insecureSeedValue2];
    }
  };
  var random = {
    getRandomBytes(len) {
      const bytes = new Uint8Array(Number(len));
      return bytes;
    },
    getRandomU64() {
      return 0n;
    },
    insecureRandom() {
      if (insecureRandomValue1 === void 0) {
        insecureRandomValue1 = random.getRandomU64();
        insecureRandomValue2 = random.getRandomU64();
      }
      return [insecureRandomValue1, insecureRandomValue2];
    }
  };

  // browser/sockets.js
  var sockets_exports = {};
  __export(sockets_exports, {
    instanceNetwork: () => instanceNetwork,
    ipNameLookup: () => ipNameLookup,
    network: () => network,
    tcp: () => tcp,
    tcpCreateSocket: () => tcpCreateSocket,
    udp: () => udp,
    udpCreateSocket: () => udpCreateSocket
  });
  var instanceNetwork = {
    instanceNetwork() {
      console.log(`[sockets] instance network`);
    }
  };
  var ipNameLookup = {
    dropResolveAddressStream() {
    },
    subscribe() {
    },
    resolveAddresses() {
    },
    resolveNextAddress() {
    },
    nonBlocking() {
    },
    setNonBlocking() {
    }
  };
  var network = {
    dropNetwork() {
    }
  };
  var tcpCreateSocket = {
    createTcpSocket() {
    }
  };
  var tcp = {
    subscribe() {
    },
    dropTcpSocket() {
    },
    bind() {
    },
    connect() {
    },
    listen() {
    },
    accept() {
    },
    localAddress() {
    },
    remoteAddress() {
    },
    addressFamily() {
    },
    ipv6Only() {
    },
    setIpv6Only() {
    },
    setListenBacklogSize() {
    },
    keepAlive() {
    },
    setKeepAlive() {
    },
    noDelay() {
    },
    setNoDelay() {
    },
    unicastHopLimit() {
    },
    setUnicastHopLimit() {
    },
    receiveBufferSize() {
    },
    setReceiveBufferSize() {
    },
    sendBufferSize() {
    },
    setSendBufferSize() {
    },
    nonBlocking() {
    },
    setNonBlocking() {
    },
    shutdown() {
    }
  };
  var udp = {
    subscribe() {
    },
    dropUdpSocket() {
    },
    bind() {
    },
    connect() {
    },
    receive() {
    },
    send() {
    },
    localAddress() {
    },
    remoteAddress() {
    },
    addressFamily() {
    },
    ipv6Only() {
    },
    setIpv6Only() {
    },
    unicastHopLimit() {
    },
    setUnicastHopLimit() {
    },
    receiveBufferSize() {
    },
    setReceiveBufferSize() {
    },
    sendBufferSize() {
    },
    setSendBufferSize() {
    },
    nonBlocking() {
    },
    setNonBlocking() {
    }
  };
  var udpCreateSocket = {
    createUdpSocket() {
    }
  };

  // browser/cli-base.js
  var cli_base_exports = {};
  __export(cli_base_exports, {
    _setEnv: () => _setEnv,
    environment: () => environment,
    exit: () => exit,
    preopens: () => preopens,
    stderr: () => stderr,
    stdin: () => stdin,
    stdout: () => stdout
  });
  var _env;
  function _setEnv(envObj) {
    _env = Object.entries(envObj);
  }
  var environment = {
    getEnvironment() {
      if (!_env)
        _env = [];
      return _env;
    }
  };
  var ComponentExit = class extends Error {
    constructor(code) {
      super(`Component exited ${code === 0 ? "successfully" : "with error"}`);
      this.code = code;
    }
  };
  var exit = {
    exit(status) {
      throw new ComponentExit(status.tag === "err" ? 1 : 0);
    }
  };
  var preopens = {
    getDirectories() {
      return [];
    }
  };
  var stdin = {
    getStdin() {
      return 0;
    }
  };
  var stdout = {
    getStdout() {
      return 1;
    }
  };
  var stderr = {
    getStderr() {
      return 2;
    }
  };

  // browser/index.js
  var importObject = {
    clocks: clocks_exports,
    filesystem: filesystem_exports,
    http: http_exports,
    io: io_exports,
    logging: logging_exports,
    poll: poll_exports,
    random: random_exports,
    sockets: sockets_exports,
    cliBase: cli_base_exports
  };
  var browser_default = importObject;

  // index.js
  function getWasiImports() {
    let exports = { ...browser_default, "cli-base": browser_default.cliBase };
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
        wasiExports[`wasi:${package_name}/${export_name_tr}`] = exports[package_name][export_name];
      }
    }
    return wasiExports;
  }

  return getWasiImports();
})();
