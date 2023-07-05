(() => {
  let dv = new DataView(new ArrayBuffer());
  const dataView = (mem) =>
    dv.buffer === mem.buffer ? dv : (dv = new DataView(mem.buffer));

  const utf8Decoder = new TextDecoder();

  const utf8Encoder = new TextEncoder();

  let utf8EncodedLen = 0;
  function utf8Encode(s, realloc, memory) {
    if (typeof s !== "string") throw new TypeError("expected a string");
    if (s.length === 0) {
      utf8EncodedLen = 0;
      return 1;
    }
    let allocLen = 0;
    let ptr = 0;
    let writtenTotal = 0;
    while (s.length > 0) {
      ptr = realloc(ptr, allocLen, 1, allocLen + s.length);
      allocLen += s.length;
      const { read, written } = utf8Encoder.encodeInto(
        s,
        new Uint8Array(
          memory.buffer,
          ptr + writtenTotal,
          allocLen - writtenTotal
        )
      );
      writtenTotal += written;
      s = s.slice(read);
    }
    if (allocLen > writtenTotal) ptr = realloc(ptr, allocLen, 1, writtenTotal);
    utf8EncodedLen = writtenTotal;
    return ptr;
  }

  function instantiate(compileCore, imports, instantiateCore) {
    const module0 = compileCore("component.core.wasm");

    let exports0;
    let memory0;
    let realloc0;
    let postReturn0;
    ({ exports: exports0 } = instantiateCore(module0));
    memory0 = exports0.memory;
    realloc0 = exports0.cabi_realloc;
    postReturn0 = exports0["cabi_post_add-hello"];

    function addHello(arg0) {
      const ptr0 = utf8Encode(arg0, realloc0, memory0);
      const len0 = utf8EncodedLen;
      const ret = exports0["add-hello"](ptr0, len0);
      const ptr1 = dataView(memory0).getInt32(ret + 0, true);
      const len1 = dataView(memory0).getInt32(ret + 4, true);
      const result1 = utf8Decoder.decode(
        new Uint8Array(memory0.buffer, ptr1, len1)
      );
      postReturn0(ret);
      return result1;
    }

    return { addHello };
  }

  return instantiate;
})();
