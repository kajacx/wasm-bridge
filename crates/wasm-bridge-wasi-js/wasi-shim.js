(() => ({
  wasi_snapshot_preview1: {
    clock_time_get: () => 0,
    fd_write: () => {},
    environ_get: () => [],
    environ_sizes_get: () => [],
    proc_exit: () => {},
  },
}))();
