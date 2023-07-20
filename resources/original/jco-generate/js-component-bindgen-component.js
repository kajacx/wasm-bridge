class ComponentError extends Error {
  constructor (value) {
    const enumerable = typeof value !== 'string';
    super(enumerable ? `${String(value)} (see error.payload)` : value);
    Object.defineProperty(this, 'payload', { value, enumerable });
  }
}

let dv = new DataView(new ArrayBuffer());
const dataView = mem => dv.buffer === mem.buffer ? dv : dv = new DataView(mem.buffer);

function getErrorPayload(e) {
  if (e && hasOwnProperty.call(e, 'payload')) return e.payload;
  return e;
}

const hasOwnProperty = Object.prototype.hasOwnProperty;

function throwUninitialized() {
  throw new TypeError('Wasm uninitialized use `await $init` first');
}

const toUint64 = val => BigInt.asUintN(64, val);

function toUint32(val) {
  return val >>> 0;
}

const utf8Decoder = new TextDecoder();

const utf8Encoder = new TextEncoder();

let utf8EncodedLen = 0;
function utf8Encode(s, realloc, memory) {
  if (typeof s !== 'string') throw new TypeError('expected a string');
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
    new Uint8Array(memory.buffer, ptr + writtenTotal, allocLen - writtenTotal),
    );
    writtenTotal += written;
    s = s.slice(read);
  }
  if (allocLen > writtenTotal)
  ptr = realloc(ptr, allocLen, 1, writtenTotal);
  utf8EncodedLen = writtenTotal;
  return ptr;
}

export async function instantiate(compileCore, imports, instantiateCore = WebAssembly.instantiate) {
  const module0 = compileCore('js-component-bindgen-component.core.wasm');
  const module1 = compileCore('js-component-bindgen-component.core2.wasm');
  const module2 = compileCore('js-component-bindgen-component.core3.wasm');
  const module3 = compileCore('js-component-bindgen-component.core4.wasm');
  
  const { environment, exit: exit$1, preopens, stderr, stdin, stdout } = imports['@bytecodealliance/preview2-shim/cli-base'];
  const { filesystem } = imports['@bytecodealliance/preview2-shim/filesystem'];
  const { streams } = imports['@bytecodealliance/preview2-shim/io'];
  const { random } = imports['@bytecodealliance/preview2-shim/random'];
  const { getEnvironment } = environment;
  const { exit } = exit$1;
  const { getDirectories } = preopens;
  const { getStderr } = stderr;
  const { getStdin } = stdin;
  const { getStdout } = stdout;
  const { appendViaStream,
    dropDescriptor,
    dropDirectoryEntryStream,
    getType,
    openAt,
    readViaStream,
    stat,
    writeViaStream } = filesystem;
  const { blockingRead,
    blockingWrite,
    dropInputStream,
    dropOutputStream,
    read,
    write } = streams;
  const { getRandomBytes } = random;
  let exports0;
  let exports1;
  
  function lowering0(arg0) {
    dropDirectoryEntryStream(arg0 >>> 0);
  }
  
  function lowering1(arg0) {
    dropDescriptor(arg0 >>> 0);
  }
  
  function lowering2(arg0) {
    let variant0;
    switch (arg0) {
      case 0: {
        variant0= {
          tag: 'ok',
          val: undefined
        };
        break;
      }
      case 1: {
        variant0= {
          tag: 'err',
          val: undefined
        };
        break;
      }
      default: {
        throw new TypeError('invalid variant discriminant for expected');
      }
    }
    exit(variant0);
  }
  
  function lowering3() {
    const ret = getStderr();
    return toUint32(ret);
  }
  
  function lowering4() {
    const ret = getStdin();
    return toUint32(ret);
  }
  
  function lowering5() {
    const ret = getStdout();
    return toUint32(ret);
  }
  
  function lowering6(arg0) {
    dropInputStream(arg0 >>> 0);
  }
  
  function lowering7(arg0) {
    dropOutputStream(arg0 >>> 0);
  }
  let exports2;
  let memory0;
  let realloc0;
  
  function lowering8(arg0) {
    const ret = getDirectories();
    const vec2 = ret;
    const len2 = vec2.length;
    const result2 = realloc0(0, 0, 4, len2 * 12);
    for (let i = 0; i < vec2.length; i++) {
      const e = vec2[i];
      const base = result2 + i * 12;const [tuple0_0, tuple0_1] = e;
      dataView(memory0).setInt32(base + 0, toUint32(tuple0_0), true);
      const ptr1 = utf8Encode(tuple0_1, realloc0, memory0);
      const len1 = utf8EncodedLen;
      dataView(memory0).setInt32(base + 8, len1, true);
      dataView(memory0).setInt32(base + 4, ptr1, true);
    }
    dataView(memory0).setInt32(arg0 + 4, len2, true);
    dataView(memory0).setInt32(arg0 + 0, result2, true);
  }
  
  function lowering9(arg0, arg1, arg2) {
    let ret;
    try {
      ret = { tag: 'ok', val: readViaStream(arg0 >>> 0, BigInt.asUintN(64, arg1)) };
    } catch (e) {
      ret = { tag: 'err', val: getErrorPayload(e) };
    }
    const variant1 = ret;
    switch (variant1.tag) {
      case 'ok': {
        const e = variant1.val;
        dataView(memory0).setInt8(arg2 + 0, 0, true);
        dataView(memory0).setInt32(arg2 + 4, toUint32(e), true);
        break;
      }
      case 'err': {
        const e = variant1.val;
        dataView(memory0).setInt8(arg2 + 0, 1, true);
        const val0 = e;
        let enum0;
        switch (val0) {
          case 'access': {
            enum0 = 0;
            break;
          }
          case 'would-block': {
            enum0 = 1;
            break;
          }
          case 'already': {
            enum0 = 2;
            break;
          }
          case 'bad-descriptor': {
            enum0 = 3;
            break;
          }
          case 'busy': {
            enum0 = 4;
            break;
          }
          case 'deadlock': {
            enum0 = 5;
            break;
          }
          case 'quota': {
            enum0 = 6;
            break;
          }
          case 'exist': {
            enum0 = 7;
            break;
          }
          case 'file-too-large': {
            enum0 = 8;
            break;
          }
          case 'illegal-byte-sequence': {
            enum0 = 9;
            break;
          }
          case 'in-progress': {
            enum0 = 10;
            break;
          }
          case 'interrupted': {
            enum0 = 11;
            break;
          }
          case 'invalid': {
            enum0 = 12;
            break;
          }
          case 'io': {
            enum0 = 13;
            break;
          }
          case 'is-directory': {
            enum0 = 14;
            break;
          }
          case 'loop': {
            enum0 = 15;
            break;
          }
          case 'too-many-links': {
            enum0 = 16;
            break;
          }
          case 'message-size': {
            enum0 = 17;
            break;
          }
          case 'name-too-long': {
            enum0 = 18;
            break;
          }
          case 'no-device': {
            enum0 = 19;
            break;
          }
          case 'no-entry': {
            enum0 = 20;
            break;
          }
          case 'no-lock': {
            enum0 = 21;
            break;
          }
          case 'insufficient-memory': {
            enum0 = 22;
            break;
          }
          case 'insufficient-space': {
            enum0 = 23;
            break;
          }
          case 'not-directory': {
            enum0 = 24;
            break;
          }
          case 'not-empty': {
            enum0 = 25;
            break;
          }
          case 'not-recoverable': {
            enum0 = 26;
            break;
          }
          case 'unsupported': {
            enum0 = 27;
            break;
          }
          case 'no-tty': {
            enum0 = 28;
            break;
          }
          case 'no-such-device': {
            enum0 = 29;
            break;
          }
          case 'overflow': {
            enum0 = 30;
            break;
          }
          case 'not-permitted': {
            enum0 = 31;
            break;
          }
          case 'pipe': {
            enum0 = 32;
            break;
          }
          case 'read-only': {
            enum0 = 33;
            break;
          }
          case 'invalid-seek': {
            enum0 = 34;
            break;
          }
          case 'text-file-busy': {
            enum0 = 35;
            break;
          }
          case 'cross-device': {
            enum0 = 36;
            break;
          }
          default: {
            if ((e) instanceof Error) {
              console.error(e);
            }
            
            throw new TypeError(`"${val0}" is not one of the cases of error-code`);
          }
        }
        dataView(memory0).setInt8(arg2 + 4, enum0, true);
        break;
      }
      default: {
        throw new TypeError('invalid variant specified for result');
      }
    }
  }
  
  function lowering10(arg0, arg1, arg2) {
    let ret;
    try {
      ret = { tag: 'ok', val: writeViaStream(arg0 >>> 0, BigInt.asUintN(64, arg1)) };
    } catch (e) {
      ret = { tag: 'err', val: getErrorPayload(e) };
    }
    const variant1 = ret;
    switch (variant1.tag) {
      case 'ok': {
        const e = variant1.val;
        dataView(memory0).setInt8(arg2 + 0, 0, true);
        dataView(memory0).setInt32(arg2 + 4, toUint32(e), true);
        break;
      }
      case 'err': {
        const e = variant1.val;
        dataView(memory0).setInt8(arg2 + 0, 1, true);
        const val0 = e;
        let enum0;
        switch (val0) {
          case 'access': {
            enum0 = 0;
            break;
          }
          case 'would-block': {
            enum0 = 1;
            break;
          }
          case 'already': {
            enum0 = 2;
            break;
          }
          case 'bad-descriptor': {
            enum0 = 3;
            break;
          }
          case 'busy': {
            enum0 = 4;
            break;
          }
          case 'deadlock': {
            enum0 = 5;
            break;
          }
          case 'quota': {
            enum0 = 6;
            break;
          }
          case 'exist': {
            enum0 = 7;
            break;
          }
          case 'file-too-large': {
            enum0 = 8;
            break;
          }
          case 'illegal-byte-sequence': {
            enum0 = 9;
            break;
          }
          case 'in-progress': {
            enum0 = 10;
            break;
          }
          case 'interrupted': {
            enum0 = 11;
            break;
          }
          case 'invalid': {
            enum0 = 12;
            break;
          }
          case 'io': {
            enum0 = 13;
            break;
          }
          case 'is-directory': {
            enum0 = 14;
            break;
          }
          case 'loop': {
            enum0 = 15;
            break;
          }
          case 'too-many-links': {
            enum0 = 16;
            break;
          }
          case 'message-size': {
            enum0 = 17;
            break;
          }
          case 'name-too-long': {
            enum0 = 18;
            break;
          }
          case 'no-device': {
            enum0 = 19;
            break;
          }
          case 'no-entry': {
            enum0 = 20;
            break;
          }
          case 'no-lock': {
            enum0 = 21;
            break;
          }
          case 'insufficient-memory': {
            enum0 = 22;
            break;
          }
          case 'insufficient-space': {
            enum0 = 23;
            break;
          }
          case 'not-directory': {
            enum0 = 24;
            break;
          }
          case 'not-empty': {
            enum0 = 25;
            break;
          }
          case 'not-recoverable': {
            enum0 = 26;
            break;
          }
          case 'unsupported': {
            enum0 = 27;
            break;
          }
          case 'no-tty': {
            enum0 = 28;
            break;
          }
          case 'no-such-device': {
            enum0 = 29;
            break;
          }
          case 'overflow': {
            enum0 = 30;
            break;
          }
          case 'not-permitted': {
            enum0 = 31;
            break;
          }
          case 'pipe': {
            enum0 = 32;
            break;
          }
          case 'read-only': {
            enum0 = 33;
            break;
          }
          case 'invalid-seek': {
            enum0 = 34;
            break;
          }
          case 'text-file-busy': {
            enum0 = 35;
            break;
          }
          case 'cross-device': {
            enum0 = 36;
            break;
          }
          default: {
            if ((e) instanceof Error) {
              console.error(e);
            }
            
            throw new TypeError(`"${val0}" is not one of the cases of error-code`);
          }
        }
        dataView(memory0).setInt8(arg2 + 4, enum0, true);
        break;
      }
      default: {
        throw new TypeError('invalid variant specified for result');
      }
    }
  }
  
  function lowering11(arg0, arg1) {
    let ret;
    try {
      ret = { tag: 'ok', val: appendViaStream(arg0 >>> 0) };
    } catch (e) {
      ret = { tag: 'err', val: getErrorPayload(e) };
    }
    const variant1 = ret;
    switch (variant1.tag) {
      case 'ok': {
        const e = variant1.val;
        dataView(memory0).setInt8(arg1 + 0, 0, true);
        dataView(memory0).setInt32(arg1 + 4, toUint32(e), true);
        break;
      }
      case 'err': {
        const e = variant1.val;
        dataView(memory0).setInt8(arg1 + 0, 1, true);
        const val0 = e;
        let enum0;
        switch (val0) {
          case 'access': {
            enum0 = 0;
            break;
          }
          case 'would-block': {
            enum0 = 1;
            break;
          }
          case 'already': {
            enum0 = 2;
            break;
          }
          case 'bad-descriptor': {
            enum0 = 3;
            break;
          }
          case 'busy': {
            enum0 = 4;
            break;
          }
          case 'deadlock': {
            enum0 = 5;
            break;
          }
          case 'quota': {
            enum0 = 6;
            break;
          }
          case 'exist': {
            enum0 = 7;
            break;
          }
          case 'file-too-large': {
            enum0 = 8;
            break;
          }
          case 'illegal-byte-sequence': {
            enum0 = 9;
            break;
          }
          case 'in-progress': {
            enum0 = 10;
            break;
          }
          case 'interrupted': {
            enum0 = 11;
            break;
          }
          case 'invalid': {
            enum0 = 12;
            break;
          }
          case 'io': {
            enum0 = 13;
            break;
          }
          case 'is-directory': {
            enum0 = 14;
            break;
          }
          case 'loop': {
            enum0 = 15;
            break;
          }
          case 'too-many-links': {
            enum0 = 16;
            break;
          }
          case 'message-size': {
            enum0 = 17;
            break;
          }
          case 'name-too-long': {
            enum0 = 18;
            break;
          }
          case 'no-device': {
            enum0 = 19;
            break;
          }
          case 'no-entry': {
            enum0 = 20;
            break;
          }
          case 'no-lock': {
            enum0 = 21;
            break;
          }
          case 'insufficient-memory': {
            enum0 = 22;
            break;
          }
          case 'insufficient-space': {
            enum0 = 23;
            break;
          }
          case 'not-directory': {
            enum0 = 24;
            break;
          }
          case 'not-empty': {
            enum0 = 25;
            break;
          }
          case 'not-recoverable': {
            enum0 = 26;
            break;
          }
          case 'unsupported': {
            enum0 = 27;
            break;
          }
          case 'no-tty': {
            enum0 = 28;
            break;
          }
          case 'no-such-device': {
            enum0 = 29;
            break;
          }
          case 'overflow': {
            enum0 = 30;
            break;
          }
          case 'not-permitted': {
            enum0 = 31;
            break;
          }
          case 'pipe': {
            enum0 = 32;
            break;
          }
          case 'read-only': {
            enum0 = 33;
            break;
          }
          case 'invalid-seek': {
            enum0 = 34;
            break;
          }
          case 'text-file-busy': {
            enum0 = 35;
            break;
          }
          case 'cross-device': {
            enum0 = 36;
            break;
          }
          default: {
            if ((e) instanceof Error) {
              console.error(e);
            }
            
            throw new TypeError(`"${val0}" is not one of the cases of error-code`);
          }
        }
        dataView(memory0).setInt8(arg1 + 4, enum0, true);
        break;
      }
      default: {
        throw new TypeError('invalid variant specified for result');
      }
    }
  }
  
  function lowering12(arg0, arg1) {
    let ret;
    try {
      ret = { tag: 'ok', val: getType(arg0 >>> 0) };
    } catch (e) {
      ret = { tag: 'err', val: getErrorPayload(e) };
    }
    const variant2 = ret;
    switch (variant2.tag) {
      case 'ok': {
        const e = variant2.val;
        dataView(memory0).setInt8(arg1 + 0, 0, true);
        const val0 = e;
        let enum0;
        switch (val0) {
          case 'unknown': {
            enum0 = 0;
            break;
          }
          case 'block-device': {
            enum0 = 1;
            break;
          }
          case 'character-device': {
            enum0 = 2;
            break;
          }
          case 'directory': {
            enum0 = 3;
            break;
          }
          case 'fifo': {
            enum0 = 4;
            break;
          }
          case 'symbolic-link': {
            enum0 = 5;
            break;
          }
          case 'regular-file': {
            enum0 = 6;
            break;
          }
          case 'socket': {
            enum0 = 7;
            break;
          }
          default: {
            if ((e) instanceof Error) {
              console.error(e);
            }
            
            throw new TypeError(`"${val0}" is not one of the cases of descriptor-type`);
          }
        }
        dataView(memory0).setInt8(arg1 + 1, enum0, true);
        break;
      }
      case 'err': {
        const e = variant2.val;
        dataView(memory0).setInt8(arg1 + 0, 1, true);
        const val1 = e;
        let enum1;
        switch (val1) {
          case 'access': {
            enum1 = 0;
            break;
          }
          case 'would-block': {
            enum1 = 1;
            break;
          }
          case 'already': {
            enum1 = 2;
            break;
          }
          case 'bad-descriptor': {
            enum1 = 3;
            break;
          }
          case 'busy': {
            enum1 = 4;
            break;
          }
          case 'deadlock': {
            enum1 = 5;
            break;
          }
          case 'quota': {
            enum1 = 6;
            break;
          }
          case 'exist': {
            enum1 = 7;
            break;
          }
          case 'file-too-large': {
            enum1 = 8;
            break;
          }
          case 'illegal-byte-sequence': {
            enum1 = 9;
            break;
          }
          case 'in-progress': {
            enum1 = 10;
            break;
          }
          case 'interrupted': {
            enum1 = 11;
            break;
          }
          case 'invalid': {
            enum1 = 12;
            break;
          }
          case 'io': {
            enum1 = 13;
            break;
          }
          case 'is-directory': {
            enum1 = 14;
            break;
          }
          case 'loop': {
            enum1 = 15;
            break;
          }
          case 'too-many-links': {
            enum1 = 16;
            break;
          }
          case 'message-size': {
            enum1 = 17;
            break;
          }
          case 'name-too-long': {
            enum1 = 18;
            break;
          }
          case 'no-device': {
            enum1 = 19;
            break;
          }
          case 'no-entry': {
            enum1 = 20;
            break;
          }
          case 'no-lock': {
            enum1 = 21;
            break;
          }
          case 'insufficient-memory': {
            enum1 = 22;
            break;
          }
          case 'insufficient-space': {
            enum1 = 23;
            break;
          }
          case 'not-directory': {
            enum1 = 24;
            break;
          }
          case 'not-empty': {
            enum1 = 25;
            break;
          }
          case 'not-recoverable': {
            enum1 = 26;
            break;
          }
          case 'unsupported': {
            enum1 = 27;
            break;
          }
          case 'no-tty': {
            enum1 = 28;
            break;
          }
          case 'no-such-device': {
            enum1 = 29;
            break;
          }
          case 'overflow': {
            enum1 = 30;
            break;
          }
          case 'not-permitted': {
            enum1 = 31;
            break;
          }
          case 'pipe': {
            enum1 = 32;
            break;
          }
          case 'read-only': {
            enum1 = 33;
            break;
          }
          case 'invalid-seek': {
            enum1 = 34;
            break;
          }
          case 'text-file-busy': {
            enum1 = 35;
            break;
          }
          case 'cross-device': {
            enum1 = 36;
            break;
          }
          default: {
            if ((e) instanceof Error) {
              console.error(e);
            }
            
            throw new TypeError(`"${val1}" is not one of the cases of error-code`);
          }
        }
        dataView(memory0).setInt8(arg1 + 1, enum1, true);
        break;
      }
      default: {
        throw new TypeError('invalid variant specified for result');
      }
    }
  }
  
  function lowering13(arg0, arg1) {
    let ret;
    try {
      ret = { tag: 'ok', val: stat(arg0 >>> 0) };
    } catch (e) {
      ret = { tag: 'err', val: getErrorPayload(e) };
    }
    const variant6 = ret;
    switch (variant6.tag) {
      case 'ok': {
        const e = variant6.val;
        dataView(memory0).setInt8(arg1 + 0, 0, true);
        const {device: v0_0, inode: v0_1, type: v0_2, linkCount: v0_3, size: v0_4, dataAccessTimestamp: v0_5, dataModificationTimestamp: v0_6, statusChangeTimestamp: v0_7 } = e;
        dataView(memory0).setBigInt64(arg1 + 8, toUint64(v0_0), true);
        dataView(memory0).setBigInt64(arg1 + 16, toUint64(v0_1), true);
        const val1 = v0_2;
        let enum1;
        switch (val1) {
          case 'unknown': {
            enum1 = 0;
            break;
          }
          case 'block-device': {
            enum1 = 1;
            break;
          }
          case 'character-device': {
            enum1 = 2;
            break;
          }
          case 'directory': {
            enum1 = 3;
            break;
          }
          case 'fifo': {
            enum1 = 4;
            break;
          }
          case 'symbolic-link': {
            enum1 = 5;
            break;
          }
          case 'regular-file': {
            enum1 = 6;
            break;
          }
          case 'socket': {
            enum1 = 7;
            break;
          }
          default: {
            if ((v0_2) instanceof Error) {
              console.error(v0_2);
            }
            
            throw new TypeError(`"${val1}" is not one of the cases of descriptor-type`);
          }
        }
        dataView(memory0).setInt8(arg1 + 24, enum1, true);
        dataView(memory0).setBigInt64(arg1 + 32, toUint64(v0_3), true);
        dataView(memory0).setBigInt64(arg1 + 40, toUint64(v0_4), true);
        const {seconds: v2_0, nanoseconds: v2_1 } = v0_5;
        dataView(memory0).setBigInt64(arg1 + 48, toUint64(v2_0), true);
        dataView(memory0).setInt32(arg1 + 56, toUint32(v2_1), true);
        const {seconds: v3_0, nanoseconds: v3_1 } = v0_6;
        dataView(memory0).setBigInt64(arg1 + 64, toUint64(v3_0), true);
        dataView(memory0).setInt32(arg1 + 72, toUint32(v3_1), true);
        const {seconds: v4_0, nanoseconds: v4_1 } = v0_7;
        dataView(memory0).setBigInt64(arg1 + 80, toUint64(v4_0), true);
        dataView(memory0).setInt32(arg1 + 88, toUint32(v4_1), true);
        break;
      }
      case 'err': {
        const e = variant6.val;
        dataView(memory0).setInt8(arg1 + 0, 1, true);
        const val5 = e;
        let enum5;
        switch (val5) {
          case 'access': {
            enum5 = 0;
            break;
          }
          case 'would-block': {
            enum5 = 1;
            break;
          }
          case 'already': {
            enum5 = 2;
            break;
          }
          case 'bad-descriptor': {
            enum5 = 3;
            break;
          }
          case 'busy': {
            enum5 = 4;
            break;
          }
          case 'deadlock': {
            enum5 = 5;
            break;
          }
          case 'quota': {
            enum5 = 6;
            break;
          }
          case 'exist': {
            enum5 = 7;
            break;
          }
          case 'file-too-large': {
            enum5 = 8;
            break;
          }
          case 'illegal-byte-sequence': {
            enum5 = 9;
            break;
          }
          case 'in-progress': {
            enum5 = 10;
            break;
          }
          case 'interrupted': {
            enum5 = 11;
            break;
          }
          case 'invalid': {
            enum5 = 12;
            break;
          }
          case 'io': {
            enum5 = 13;
            break;
          }
          case 'is-directory': {
            enum5 = 14;
            break;
          }
          case 'loop': {
            enum5 = 15;
            break;
          }
          case 'too-many-links': {
            enum5 = 16;
            break;
          }
          case 'message-size': {
            enum5 = 17;
            break;
          }
          case 'name-too-long': {
            enum5 = 18;
            break;
          }
          case 'no-device': {
            enum5 = 19;
            break;
          }
          case 'no-entry': {
            enum5 = 20;
            break;
          }
          case 'no-lock': {
            enum5 = 21;
            break;
          }
          case 'insufficient-memory': {
            enum5 = 22;
            break;
          }
          case 'insufficient-space': {
            enum5 = 23;
            break;
          }
          case 'not-directory': {
            enum5 = 24;
            break;
          }
          case 'not-empty': {
            enum5 = 25;
            break;
          }
          case 'not-recoverable': {
            enum5 = 26;
            break;
          }
          case 'unsupported': {
            enum5 = 27;
            break;
          }
          case 'no-tty': {
            enum5 = 28;
            break;
          }
          case 'no-such-device': {
            enum5 = 29;
            break;
          }
          case 'overflow': {
            enum5 = 30;
            break;
          }
          case 'not-permitted': {
            enum5 = 31;
            break;
          }
          case 'pipe': {
            enum5 = 32;
            break;
          }
          case 'read-only': {
            enum5 = 33;
            break;
          }
          case 'invalid-seek': {
            enum5 = 34;
            break;
          }
          case 'text-file-busy': {
            enum5 = 35;
            break;
          }
          case 'cross-device': {
            enum5 = 36;
            break;
          }
          default: {
            if ((e) instanceof Error) {
              console.error(e);
            }
            
            throw new TypeError(`"${val5}" is not one of the cases of error-code`);
          }
        }
        dataView(memory0).setInt8(arg1 + 8, enum5, true);
        break;
      }
      default: {
        throw new TypeError('invalid variant specified for result');
      }
    }
  }
  
  function lowering14(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    if ((arg1 & 4294967294) !== 0) {
      throw new TypeError('flags have extraneous bits set');
    }
    const flags0 = {
      symlinkFollow: Boolean(arg1 & 1),
    };
    const ptr1 = arg2;
    const len1 = arg3;
    const result1 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr1, len1));
    if ((arg4 & 4294967280) !== 0) {
      throw new TypeError('flags have extraneous bits set');
    }
    const flags2 = {
      create: Boolean(arg4 & 1),
      directory: Boolean(arg4 & 2),
      exclusive: Boolean(arg4 & 4),
      truncate: Boolean(arg4 & 8),
    };
    if ((arg5 & 4294967232) !== 0) {
      throw new TypeError('flags have extraneous bits set');
    }
    const flags3 = {
      read: Boolean(arg5 & 1),
      write: Boolean(arg5 & 2),
      fileIntegritySync: Boolean(arg5 & 4),
      dataIntegritySync: Boolean(arg5 & 8),
      requestedWriteSync: Boolean(arg5 & 16),
      mutateDirectory: Boolean(arg5 & 32),
    };
    if ((arg6 & 4294967288) !== 0) {
      throw new TypeError('flags have extraneous bits set');
    }
    const flags4 = {
      readable: Boolean(arg6 & 1),
      writable: Boolean(arg6 & 2),
      executable: Boolean(arg6 & 4),
    };
    let ret;
    try {
      ret = { tag: 'ok', val: openAt(arg0 >>> 0, flags0, result1, flags2, flags3, flags4) };
    } catch (e) {
      ret = { tag: 'err', val: getErrorPayload(e) };
    }
    const variant6 = ret;
    switch (variant6.tag) {
      case 'ok': {
        const e = variant6.val;
        dataView(memory0).setInt8(arg7 + 0, 0, true);
        dataView(memory0).setInt32(arg7 + 4, toUint32(e), true);
        break;
      }
      case 'err': {
        const e = variant6.val;
        dataView(memory0).setInt8(arg7 + 0, 1, true);
        const val5 = e;
        let enum5;
        switch (val5) {
          case 'access': {
            enum5 = 0;
            break;
          }
          case 'would-block': {
            enum5 = 1;
            break;
          }
          case 'already': {
            enum5 = 2;
            break;
          }
          case 'bad-descriptor': {
            enum5 = 3;
            break;
          }
          case 'busy': {
            enum5 = 4;
            break;
          }
          case 'deadlock': {
            enum5 = 5;
            break;
          }
          case 'quota': {
            enum5 = 6;
            break;
          }
          case 'exist': {
            enum5 = 7;
            break;
          }
          case 'file-too-large': {
            enum5 = 8;
            break;
          }
          case 'illegal-byte-sequence': {
            enum5 = 9;
            break;
          }
          case 'in-progress': {
            enum5 = 10;
            break;
          }
          case 'interrupted': {
            enum5 = 11;
            break;
          }
          case 'invalid': {
            enum5 = 12;
            break;
          }
          case 'io': {
            enum5 = 13;
            break;
          }
          case 'is-directory': {
            enum5 = 14;
            break;
          }
          case 'loop': {
            enum5 = 15;
            break;
          }
          case 'too-many-links': {
            enum5 = 16;
            break;
          }
          case 'message-size': {
            enum5 = 17;
            break;
          }
          case 'name-too-long': {
            enum5 = 18;
            break;
          }
          case 'no-device': {
            enum5 = 19;
            break;
          }
          case 'no-entry': {
            enum5 = 20;
            break;
          }
          case 'no-lock': {
            enum5 = 21;
            break;
          }
          case 'insufficient-memory': {
            enum5 = 22;
            break;
          }
          case 'insufficient-space': {
            enum5 = 23;
            break;
          }
          case 'not-directory': {
            enum5 = 24;
            break;
          }
          case 'not-empty': {
            enum5 = 25;
            break;
          }
          case 'not-recoverable': {
            enum5 = 26;
            break;
          }
          case 'unsupported': {
            enum5 = 27;
            break;
          }
          case 'no-tty': {
            enum5 = 28;
            break;
          }
          case 'no-such-device': {
            enum5 = 29;
            break;
          }
          case 'overflow': {
            enum5 = 30;
            break;
          }
          case 'not-permitted': {
            enum5 = 31;
            break;
          }
          case 'pipe': {
            enum5 = 32;
            break;
          }
          case 'read-only': {
            enum5 = 33;
            break;
          }
          case 'invalid-seek': {
            enum5 = 34;
            break;
          }
          case 'text-file-busy': {
            enum5 = 35;
            break;
          }
          case 'cross-device': {
            enum5 = 36;
            break;
          }
          default: {
            if ((e) instanceof Error) {
              console.error(e);
            }
            
            throw new TypeError(`"${val5}" is not one of the cases of error-code`);
          }
        }
        dataView(memory0).setInt8(arg7 + 4, enum5, true);
        break;
      }
      default: {
        throw new TypeError('invalid variant specified for result');
      }
    }
  }
  
  function lowering15(arg0, arg1) {
    const ret = getRandomBytes(BigInt.asUintN(64, arg0));
    const val0 = ret;
    const len0 = val0.byteLength;
    const ptr0 = realloc0(0, 0, 1, len0 * 1);
    const src0 = new Uint8Array(val0.buffer || val0, val0.byteOffset, len0 * 1);
    (new Uint8Array(memory0.buffer, ptr0, len0 * 1)).set(src0);
    dataView(memory0).setInt32(arg1 + 4, len0, true);
    dataView(memory0).setInt32(arg1 + 0, ptr0, true);
  }
  
  function lowering16(arg0) {
    const ret = getEnvironment();
    const vec3 = ret;
    const len3 = vec3.length;
    const result3 = realloc0(0, 0, 4, len3 * 16);
    for (let i = 0; i < vec3.length; i++) {
      const e = vec3[i];
      const base = result3 + i * 16;const [tuple0_0, tuple0_1] = e;
      const ptr1 = utf8Encode(tuple0_0, realloc0, memory0);
      const len1 = utf8EncodedLen;
      dataView(memory0).setInt32(base + 4, len1, true);
      dataView(memory0).setInt32(base + 0, ptr1, true);
      const ptr2 = utf8Encode(tuple0_1, realloc0, memory0);
      const len2 = utf8EncodedLen;
      dataView(memory0).setInt32(base + 12, len2, true);
      dataView(memory0).setInt32(base + 8, ptr2, true);
    }
    dataView(memory0).setInt32(arg0 + 4, len3, true);
    dataView(memory0).setInt32(arg0 + 0, result3, true);
  }
  
  function lowering17(arg0, arg1, arg2) {
    let ret;
    try {
      ret = { tag: 'ok', val: read(arg0 >>> 0, BigInt.asUintN(64, arg1)) };
    } catch (e) {
      ret = { tag: 'err', val: getErrorPayload(e) };
    }
    const variant3 = ret;
    switch (variant3.tag) {
      case 'ok': {
        const e = variant3.val;
        dataView(memory0).setInt8(arg2 + 0, 0, true);
        const [tuple0_0, tuple0_1] = e;
        const val1 = tuple0_0;
        const len1 = val1.byteLength;
        const ptr1 = realloc0(0, 0, 1, len1 * 1);
        const src1 = new Uint8Array(val1.buffer || val1, val1.byteOffset, len1 * 1);
        (new Uint8Array(memory0.buffer, ptr1, len1 * 1)).set(src1);
        dataView(memory0).setInt32(arg2 + 8, len1, true);
        dataView(memory0).setInt32(arg2 + 4, ptr1, true);
        dataView(memory0).setInt8(arg2 + 12, tuple0_1 ? 1 : 0, true);
        break;
      }
      case 'err': {
        const e = variant3.val;
        dataView(memory0).setInt8(arg2 + 0, 1, true);
        const { } = e;
        break;
      }
      default: {
        throw new TypeError('invalid variant specified for result');
      }
    }
  }
  
  function lowering18(arg0, arg1, arg2) {
    let ret;
    try {
      ret = { tag: 'ok', val: blockingRead(arg0 >>> 0, BigInt.asUintN(64, arg1)) };
    } catch (e) {
      ret = { tag: 'err', val: getErrorPayload(e) };
    }
    const variant3 = ret;
    switch (variant3.tag) {
      case 'ok': {
        const e = variant3.val;
        dataView(memory0).setInt8(arg2 + 0, 0, true);
        const [tuple0_0, tuple0_1] = e;
        const val1 = tuple0_0;
        const len1 = val1.byteLength;
        const ptr1 = realloc0(0, 0, 1, len1 * 1);
        const src1 = new Uint8Array(val1.buffer || val1, val1.byteOffset, len1 * 1);
        (new Uint8Array(memory0.buffer, ptr1, len1 * 1)).set(src1);
        dataView(memory0).setInt32(arg2 + 8, len1, true);
        dataView(memory0).setInt32(arg2 + 4, ptr1, true);
        dataView(memory0).setInt8(arg2 + 12, tuple0_1 ? 1 : 0, true);
        break;
      }
      case 'err': {
        const e = variant3.val;
        dataView(memory0).setInt8(arg2 + 0, 1, true);
        const { } = e;
        break;
      }
      default: {
        throw new TypeError('invalid variant specified for result');
      }
    }
  }
  
  function lowering19(arg0, arg1, arg2, arg3) {
    const ptr0 = arg1;
    const len0 = arg2;
    const result0 = new Uint8Array(memory0.buffer.slice(ptr0, ptr0 + len0 * 1));
    let ret;
    try {
      ret = { tag: 'ok', val: write(arg0 >>> 0, result0) };
    } catch (e) {
      ret = { tag: 'err', val: getErrorPayload(e) };
    }
    const variant2 = ret;
    switch (variant2.tag) {
      case 'ok': {
        const e = variant2.val;
        dataView(memory0).setInt8(arg3 + 0, 0, true);
        dataView(memory0).setBigInt64(arg3 + 8, toUint64(e), true);
        break;
      }
      case 'err': {
        const e = variant2.val;
        dataView(memory0).setInt8(arg3 + 0, 1, true);
        const { } = e;
        break;
      }
      default: {
        throw new TypeError('invalid variant specified for result');
      }
    }
  }
  
  function lowering20(arg0, arg1, arg2, arg3) {
    const ptr0 = arg1;
    const len0 = arg2;
    const result0 = new Uint8Array(memory0.buffer.slice(ptr0, ptr0 + len0 * 1));
    let ret;
    try {
      ret = { tag: 'ok', val: blockingWrite(arg0 >>> 0, result0) };
    } catch (e) {
      ret = { tag: 'err', val: getErrorPayload(e) };
    }
    const variant2 = ret;
    switch (variant2.tag) {
      case 'ok': {
        const e = variant2.val;
        dataView(memory0).setInt8(arg3 + 0, 0, true);
        dataView(memory0).setBigInt64(arg3 + 8, toUint64(e), true);
        break;
      }
      case 'err': {
        const e = variant2.val;
        dataView(memory0).setInt8(arg3 + 0, 1, true);
        const { } = e;
        break;
      }
      default: {
        throw new TypeError('invalid variant specified for result');
      }
    }
  }
  let exports3;
  let realloc1;
  let postReturn0;
  let postReturn1;
  Promise.all([module0, module1, module2, module3]).catch(() => {});
  ({ exports: exports0 } = await instantiateCore(await module2));
  ({ exports: exports1 } = await instantiateCore(await module0, {
    wasi_snapshot_preview1: {
      environ_get: exports0['18'],
      environ_sizes_get: exports0['19'],
      fd_close: exports0['20'],
      fd_filestat_get: exports0['13'],
      fd_prestat_dir_name: exports0['22'],
      fd_prestat_get: exports0['21'],
      fd_read: exports0['14'],
      fd_write: exports0['15'],
      path_open: exports0['16'],
      proc_exit: exports0['23'],
      random_get: exports0['17'],
    },
  }));
  ({ exports: exports2 } = await instantiateCore(await module1, {
    __main_module__: {
      cabi_realloc: exports1.cabi_realloc,
    },
    env: {
      memory: exports1.memory,
    },
    'wasi:cli-base/environment': {
      'get-environment': exports0['8'],
    },
    'wasi:cli-base/exit': {
      exit: lowering2,
    },
    'wasi:cli-base/preopens': {
      'get-directories': exports0['0'],
    },
    'wasi:cli-base/stderr': {
      'get-stderr': lowering3,
    },
    'wasi:cli-base/stdin': {
      'get-stdin': lowering4,
    },
    'wasi:cli-base/stdout': {
      'get-stdout': lowering5,
    },
    'wasi:filesystem/filesystem': {
      'append-via-stream': exports0['3'],
      'drop-descriptor': lowering1,
      'drop-directory-entry-stream': lowering0,
      'get-type': exports0['4'],
      'open-at': exports0['6'],
      'read-via-stream': exports0['1'],
      stat: exports0['5'],
      'write-via-stream': exports0['2'],
    },
    'wasi:io/streams': {
      'blocking-read': exports0['10'],
      'blocking-write': exports0['12'],
      'drop-input-stream': lowering6,
      'drop-output-stream': lowering7,
      read: exports0['9'],
      write: exports0['11'],
    },
    'wasi:random/random': {
      'get-random-bytes': exports0['7'],
    },
  }));
  memory0 = exports1.memory;
  realloc0 = exports2.cabi_import_realloc;
  ({ exports: exports3 } = await instantiateCore(await module3, {
    '': {
      $imports: exports0.$imports,
      '0': lowering8,
      '1': lowering9,
      '10': lowering18,
      '11': lowering19,
      '12': lowering20,
      '13': exports2.fd_filestat_get,
      '14': exports2.fd_read,
      '15': exports2.fd_write,
      '16': exports2.path_open,
      '17': exports2.random_get,
      '18': exports2.environ_get,
      '19': exports2.environ_sizes_get,
      '2': lowering10,
      '20': exports2.fd_close,
      '21': exports2.fd_prestat_get,
      '22': exports2.fd_prestat_dir_name,
      '23': exports2.proc_exit,
      '3': lowering11,
      '4': lowering12,
      '5': lowering13,
      '6': lowering14,
      '7': lowering15,
      '8': lowering16,
      '9': lowering17,
    },
  }));
  realloc1 = exports1.cabi_realloc;
  postReturn0 = exports1.cabi_post_generate;
  postReturn1 = exports1['cabi_post_generate-types'];
  
  function generate(arg0, arg1) {
    if (!_initialized) throwUninitialized();
    const ptr0 = realloc1(0, 0, 4, 48);
    const val1 = arg0;
    const len1 = val1.byteLength;
    const ptr1 = realloc1(0, 0, 1, len1 * 1);
    const src1 = new Uint8Array(val1.buffer || val1, val1.byteOffset, len1 * 1);
    (new Uint8Array(memory0.buffer, ptr1, len1 * 1)).set(src1);
    dataView(memory0).setInt32(ptr0 + 4, len1, true);
    dataView(memory0).setInt32(ptr0 + 0, ptr1, true);
    const {name: v2_0, noTypescript: v2_1, instantiation: v2_2, map: v2_3, compat: v2_4, noNodejsCompat: v2_5, base64Cutoff: v2_6, tlaCompat: v2_7, validLiftingOptimization: v2_8 } = arg1;
    const ptr3 = utf8Encode(v2_0, realloc1, memory0);
    const len3 = utf8EncodedLen;
    dataView(memory0).setInt32(ptr0 + 12, len3, true);
    dataView(memory0).setInt32(ptr0 + 8, ptr3, true);
    const variant4 = v2_1;
    if (variant4 === null || variant4=== undefined) {
      dataView(memory0).setInt8(ptr0 + 16, 0, true);
    } else {
      const e = variant4;
      dataView(memory0).setInt8(ptr0 + 16, 1, true);
      dataView(memory0).setInt8(ptr0 + 17, e ? 1 : 0, true);
    }
    const variant5 = v2_2;
    if (variant5 === null || variant5=== undefined) {
      dataView(memory0).setInt8(ptr0 + 18, 0, true);
    } else {
      const e = variant5;
      dataView(memory0).setInt8(ptr0 + 18, 1, true);
      dataView(memory0).setInt8(ptr0 + 19, e ? 1 : 0, true);
    }
    const variant10 = v2_3;
    if (variant10 === null || variant10=== undefined) {
      dataView(memory0).setInt8(ptr0 + 20, 0, true);
    } else {
      const e = variant10;
      dataView(memory0).setInt8(ptr0 + 20, 1, true);
      const vec9 = e;
      const len9 = vec9.length;
      const result9 = realloc1(0, 0, 4, len9 * 16);
      for (let i = 0; i < vec9.length; i++) {
        const e = vec9[i];
        const base = result9 + i * 16;const [tuple6_0, tuple6_1] = e;
        const ptr7 = utf8Encode(tuple6_0, realloc1, memory0);
        const len7 = utf8EncodedLen;
        dataView(memory0).setInt32(base + 4, len7, true);
        dataView(memory0).setInt32(base + 0, ptr7, true);
        const ptr8 = utf8Encode(tuple6_1, realloc1, memory0);
        const len8 = utf8EncodedLen;
        dataView(memory0).setInt32(base + 12, len8, true);
        dataView(memory0).setInt32(base + 8, ptr8, true);
      }
      dataView(memory0).setInt32(ptr0 + 28, len9, true);
      dataView(memory0).setInt32(ptr0 + 24, result9, true);
    }
    const variant11 = v2_4;
    if (variant11 === null || variant11=== undefined) {
      dataView(memory0).setInt8(ptr0 + 32, 0, true);
    } else {
      const e = variant11;
      dataView(memory0).setInt8(ptr0 + 32, 1, true);
      dataView(memory0).setInt8(ptr0 + 33, e ? 1 : 0, true);
    }
    const variant12 = v2_5;
    if (variant12 === null || variant12=== undefined) {
      dataView(memory0).setInt8(ptr0 + 34, 0, true);
    } else {
      const e = variant12;
      dataView(memory0).setInt8(ptr0 + 34, 1, true);
      dataView(memory0).setInt8(ptr0 + 35, e ? 1 : 0, true);
    }
    const variant13 = v2_6;
    if (variant13 === null || variant13=== undefined) {
      dataView(memory0).setInt8(ptr0 + 36, 0, true);
    } else {
      const e = variant13;
      dataView(memory0).setInt8(ptr0 + 36, 1, true);
      dataView(memory0).setInt32(ptr0 + 40, toUint32(e), true);
    }
    const variant14 = v2_7;
    if (variant14 === null || variant14=== undefined) {
      dataView(memory0).setInt8(ptr0 + 44, 0, true);
    } else {
      const e = variant14;
      dataView(memory0).setInt8(ptr0 + 44, 1, true);
      dataView(memory0).setInt8(ptr0 + 45, e ? 1 : 0, true);
    }
    const variant15 = v2_8;
    if (variant15 === null || variant15=== undefined) {
      dataView(memory0).setInt8(ptr0 + 46, 0, true);
    } else {
      const e = variant15;
      dataView(memory0).setInt8(ptr0 + 46, 1, true);
      dataView(memory0).setInt8(ptr0 + 47, e ? 1 : 0, true);
    }
    const ret = exports1.generate(ptr0);
    let variant25;
    switch (dataView(memory0).getUint8(ret + 0, true)) {
      case 0: {
        const len18 = dataView(memory0).getInt32(ret + 8, true);
        const base18 = dataView(memory0).getInt32(ret + 4, true);
        const result18 = [];
        for (let i = 0; i < len18; i++) {
          const base = base18 + i * 16;
          const ptr16 = dataView(memory0).getInt32(base + 0, true);
          const len16 = dataView(memory0).getInt32(base + 4, true);
          const result16 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr16, len16));
          const ptr17 = dataView(memory0).getInt32(base + 8, true);
          const len17 = dataView(memory0).getInt32(base + 12, true);
          const result17 = new Uint8Array(memory0.buffer.slice(ptr17, ptr17 + len17 * 1));
          result18.push([result16, result17]);
        }
        const len20 = dataView(memory0).getInt32(ret + 16, true);
        const base20 = dataView(memory0).getInt32(ret + 12, true);
        const result20 = [];
        for (let i = 0; i < len20; i++) {
          const base = base20 + i * 8;
          const ptr19 = dataView(memory0).getInt32(base + 0, true);
          const len19 = dataView(memory0).getInt32(base + 4, true);
          const result19 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr19, len19));
          result20.push(result19);
        }
        const len23 = dataView(memory0).getInt32(ret + 24, true);
        const base23 = dataView(memory0).getInt32(ret + 20, true);
        const result23 = [];
        for (let i = 0; i < len23; i++) {
          const base = base23 + i * 12;
          const ptr21 = dataView(memory0).getInt32(base + 0, true);
          const len21 = dataView(memory0).getInt32(base + 4, true);
          const result21 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr21, len21));
          let enum22;
          switch (dataView(memory0).getUint8(base + 8, true)) {
            case 0: {
              enum22 = 'function';
              break;
            }
            case 1: {
              enum22 = 'instance';
              break;
            }
            default: {
              throw new TypeError('invalid discriminant specified for ExportType');
            }
          }
          result23.push([result21, enum22]);
        }
        variant25= {
          tag: 'ok',
          val: {
            files: result18,
            imports: result20,
            exports: result23,
          }
        };
        break;
      }
      case 1: {
        const ptr24 = dataView(memory0).getInt32(ret + 4, true);
        const len24 = dataView(memory0).getInt32(ret + 8, true);
        const result24 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr24, len24));
        variant25= {
          tag: 'err',
          val: result24
        };
        break;
      }
      default: {
        throw new TypeError('invalid variant discriminant for expected');
      }
    }
    postReturn0(ret);
    if (variant25.tag === 'err') {
      throw new ComponentError(variant25.val);
    }
    return variant25.val;
  }
  
  function generateTypes(arg0, arg1) {
    if (!_initialized) throwUninitialized();
    const ptr0 = utf8Encode(arg0, realloc1, memory0);
    const len0 = utf8EncodedLen;
    const {wit: v1_0, world: v1_1, tlaCompat: v1_2, instantiation: v1_3, map: v1_4 } = arg1;
    const variant5 = v1_0;
    let variant5_0;
    let variant5_1;
    let variant5_2;
    switch (variant5.tag) {
      case 'source': {
        const e = variant5.val;
        const ptr2 = utf8Encode(e, realloc1, memory0);
        const len2 = utf8EncodedLen;
        variant5_0 = 0;
        variant5_1 = ptr2;
        variant5_2 = len2;
        break;
      }
      case 'binary': {
        const e = variant5.val;
        const val3 = e;
        const len3 = val3.byteLength;
        const ptr3 = realloc1(0, 0, 1, len3 * 1);
        const src3 = new Uint8Array(val3.buffer || val3, val3.byteOffset, len3 * 1);
        (new Uint8Array(memory0.buffer, ptr3, len3 * 1)).set(src3);
        variant5_0 = 1;
        variant5_1 = ptr3;
        variant5_2 = len3;
        break;
      }
      case 'path': {
        const e = variant5.val;
        const ptr4 = utf8Encode(e, realloc1, memory0);
        const len4 = utf8EncodedLen;
        variant5_0 = 2;
        variant5_1 = ptr4;
        variant5_2 = len4;
        break;
      }
      default: {
        throw new TypeError('invalid variant specified for Wit');
      }
    }
    const variant7 = v1_1;
    let variant7_0;
    let variant7_1;
    let variant7_2;
    if (variant7 === null || variant7=== undefined) {
      variant7_0 = 0;
      variant7_1 = 0;
      variant7_2 = 0;
    } else {
      const e = variant7;
      const ptr6 = utf8Encode(e, realloc1, memory0);
      const len6 = utf8EncodedLen;
      variant7_0 = 1;
      variant7_1 = ptr6;
      variant7_2 = len6;
    }
    const variant8 = v1_2;
    let variant8_0;
    let variant8_1;
    if (variant8 === null || variant8=== undefined) {
      variant8_0 = 0;
      variant8_1 = 0;
    } else {
      const e = variant8;
      variant8_0 = 1;
      variant8_1 = e ? 1 : 0;
    }
    const variant9 = v1_3;
    let variant9_0;
    let variant9_1;
    if (variant9 === null || variant9=== undefined) {
      variant9_0 = 0;
      variant9_1 = 0;
    } else {
      const e = variant9;
      variant9_0 = 1;
      variant9_1 = e ? 1 : 0;
    }
    const variant14 = v1_4;
    let variant14_0;
    let variant14_1;
    let variant14_2;
    if (variant14 === null || variant14=== undefined) {
      variant14_0 = 0;
      variant14_1 = 0;
      variant14_2 = 0;
    } else {
      const e = variant14;
      const vec13 = e;
      const len13 = vec13.length;
      const result13 = realloc1(0, 0, 4, len13 * 16);
      for (let i = 0; i < vec13.length; i++) {
        const e = vec13[i];
        const base = result13 + i * 16;const [tuple10_0, tuple10_1] = e;
        const ptr11 = utf8Encode(tuple10_0, realloc1, memory0);
        const len11 = utf8EncodedLen;
        dataView(memory0).setInt32(base + 4, len11, true);
        dataView(memory0).setInt32(base + 0, ptr11, true);
        const ptr12 = utf8Encode(tuple10_1, realloc1, memory0);
        const len12 = utf8EncodedLen;
        dataView(memory0).setInt32(base + 12, len12, true);
        dataView(memory0).setInt32(base + 8, ptr12, true);
      }
      variant14_0 = 1;
      variant14_1 = result13;
      variant14_2 = len13;
    }
    const ret = exports1['generate-types'](ptr0, len0, variant5_0, variant5_1, variant5_2, variant7_0, variant7_1, variant7_2, variant8_0, variant8_1, variant9_0, variant9_1, variant14_0, variant14_1, variant14_2);
    let variant19;
    switch (dataView(memory0).getUint8(ret + 0, true)) {
      case 0: {
        const len17 = dataView(memory0).getInt32(ret + 8, true);
        const base17 = dataView(memory0).getInt32(ret + 4, true);
        const result17 = [];
        for (let i = 0; i < len17; i++) {
          const base = base17 + i * 16;
          const ptr15 = dataView(memory0).getInt32(base + 0, true);
          const len15 = dataView(memory0).getInt32(base + 4, true);
          const result15 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr15, len15));
          const ptr16 = dataView(memory0).getInt32(base + 8, true);
          const len16 = dataView(memory0).getInt32(base + 12, true);
          const result16 = new Uint8Array(memory0.buffer.slice(ptr16, ptr16 + len16 * 1));
          result17.push([result15, result16]);
        }
        variant19= {
          tag: 'ok',
          val: result17
        };
        break;
      }
      case 1: {
        const ptr18 = dataView(memory0).getInt32(ret + 4, true);
        const len18 = dataView(memory0).getInt32(ret + 8, true);
        const result18 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr18, len18));
        variant19= {
          tag: 'err',
          val: result18
        };
        break;
      }
      default: {
        throw new TypeError('invalid variant discriminant for expected');
      }
    }
    postReturn1(ret);
    if (variant19.tag === 'err') {
      throw new ComponentError(variant19.val);
    }
    return variant19.val;
  }
  
  return { generate, generateTypes };
}
