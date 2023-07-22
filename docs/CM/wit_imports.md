# Defining wit imports

First, read [My first component](./my_first_component.md) to understand how to get started.

## Full minimal example

A full example of using wit imports can be found [here](https://github.com/kajacx/wasm-playground/tree/wasm-bridge-06-readd-imports).

## Steps to add imports

Wit imports work the same way as they do in wasmtime.

This is how to add imports to the "My first component" created in the previous tutorial:

1. Add an "import" function to the wit file
```wit
package usage:example

world calculator {
  import add-one: func(num: s32) -> s32

  export add-three: func(num: s32) -> s32
}
```

2. Use the imported functions in the guest
```rust
// In guest

struct MyCalculator;

impl Calculator for MyCalculator {
    fn add(num: i32) -> i32 {
        let num = add_one(num);
        let num = add_one(num);
        let num = add_one(num);
        num
    }
}
```

3. Define host import struct
```rust
// In host (runtime)

struct CalculatorData {}
```

4. Implement the world imports for your struct
```rust
// Name based on world name
impl CalculatorImports for CalculatorData {
    fn add_one(&mut self, num: i32) -> Result<i32> {
        Ok(num + 1)
    }
}
```

The functions must return a `wasm_bride::Result`, which is re-exported `anyhow::Result`.

Returning an `Err` variant will result in an error return on the call site,
but the instance will not be re-entriable.

5. Add your struct to the store's data.

```rust
let mut store = Store::new(&engine, CalculatorData {});
```

Note: the store's data can be a different type than your struct that implements the imports,
but it is easiest to use same struct in both cases.

6. Add the imports to the linker

```rust
let mut linker = Linker::new(store.engine());
Calculator::add_to_linker(&mut linker, |data| data)?;
```

Note: the closure passed as the second argument gets the CalculatorData from store's data.
If store's data *is* CalculatorData, it can just return the input.

7. And that's it

Now you can instantiate your guest with this linker and call exported functions on the instance as before:

```rust
let (calculator, _) = Calculator::instantiate(&mut store, &component, &linker)?;

let result = calculator.call_add(5, 3)?;
assert_eq!(result, 8);
```
