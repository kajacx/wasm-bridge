# Defining wit imports

First, read [My first component](./my_first_component.md) to understand how to get started.

## Full minimal example

A full example of using wit imports can be found [here](https://github.com/kajacx/wasm-playground/tree/wasm-bridge-04-wit-imports).

## Steps to add imports

Wit imports work the same way as they do in wasmtime. Here is a quick refresher:

1. Define a wit file with imports
```wit
package usage:example

world calculator {
  import store-variable(name: string, value: s32)
  import get-variable(name: string) -> option<s32>

  export add: func(a: s32, b: s32) -> s32
}
```

2. Use the imported functions in the plugin
```rust
// In plugin

struct MyCalculator;

impl Calculator for MyCalculator {
    fn add(a: i32, b: i32) -> i32 {
        let times_called = get_variable("times_called").unwrap_or(0);
        store_variable("times_called", times_called + 1);
        a + b + times_called
    }
}
```

3. Define host import struct
```rust
// In host (runtime)

struct CalculatorData {
    variables: HashMap<String, i32>,
}
```

4. Implement the world imports for your struct
```rust
// Name based on world name
impl CalculatorImports for CalculatorData {
    fn store_variable(&mut self, name: String, value: i32) -> Result<()> {
        self.variables.insert(name, value);
        Ok(())
    }

    fn get_variable(&mut self, name: String) -> Result<Option<i32>> {
        Ok(self.variables.get(&name).copied())
    }
}
```

The functions must return a `wasm_bride::Result`, but handling the `Err` case is not yet implemented and will result in a panic on the web.

5. Add your struct to the store's data.

```rust
let mut store = Store::new(&engine, CalculatorData::new());
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

Now you can instantiate you plugin with this linker:

```rust
let (calculator, _) = Calculator::instantiate(&mut store, &component, &linker)?;

let result = calculator.call_add(5, 3)?;
assert_eq!(result, 8);

// Calling again, should add an extra 1
let result = calculator.call_add(5, 3)?;
assert_eq!(result, 9);
```

8. Access store's data

You can also read and manipulate the store's data between calls:

```rust
// Get a variable
let var_x = store.data().variables.get("x").copied();

// Clear all variables
store.data_mut().variables.clear();
```

This is not unique to the component model, but it might come in handy.
