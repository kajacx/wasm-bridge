(module
  (func $add_ten_all (export "add_ten_all") (param $p0 i32) (param $p1 i64) (result i32 i64) (
    (i32.add (local.get $p0) (i32.const 10))
    (i64.add (local.get $p1) (i64.const 10))
  ))
)
