use wasm_bridge::{
    component::{new_universal_component, Component, Linker},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "test-world",
});

struct HostData {
    number: i32,
}

impl TestWorldImports for HostData {
    fn set_salary(&mut self, mut employee: Person, amount: u32) -> Result<Person> {
        employee.salary = amount;
        Ok(employee)
    }

    fn increment(&mut self) -> Result<()> {
        self.number += 1;
        Ok(())
    }

    fn add_sub_two(&mut self, num: i32) -> Result<(i32, i32)> {
        Ok((num + 2, num - 2))
    }

    fn add_sub_ten(&mut self, num: i32) -> Result<(i32, i32)> {
        Ok((num + 10, num - 10))
    }

    fn add_all(
        &mut self,
        a: i32,
        b: i64,
        c: u32,
        d: u64,
        e: f32,
        f: f64,
        g: String,
    ) -> Result<f64> {
        Ok(a as f64 + b as f64 + c as f64 + d as f64 + e as f64 + f + g.parse::<f64>().unwrap())
    }
}

pub fn run_test(component_bytes: &[u8], universal_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, HostData { number: 0 });

    let component = Component::new(&store.engine(), &component_bytes)?;
    run_with_component(&mut store, &component)?;

    if !cfg!(target_arch = "wasm32") {
        // Doesn't work on sys
        Component::new(&store.engine(), universal_bytes)
            .map(|_| ())
            .expect_err("should not load component normally with universal bytes");
    } else {
        // But it works on js
        let component = Component::new(&store.engine(), &universal_bytes)?;
        run_with_component(&mut store, &component)?;
    }

    let component = new_universal_component(&store.engine(), &component_bytes)?;
    run_with_component(&mut store, &component)?;

    let component = new_universal_component(&store.engine(), &universal_bytes)?;
    run_with_component(&mut store, &component)?;

    Ok(())
}

fn run_with_component(mut store: &mut Store<HostData>, component: &Component) -> Result<()> {
    let mut linker = Linker::new(store.engine());
    TestWorld::add_to_linker(&mut linker, |data| data)?;

    let (instance, _) = TestWorld::instantiate(&mut store, component, &linker)?;

    let result = instance.call_promote_person(
        &mut store,
        &Person {
            full_name: "John Conner".into(),
            age: 30,
            salary: 10_000,
        },
        5_000,
    )?;
    assert_eq!(result.full_name, "John Conner");
    assert_eq!(result.age, 30);
    assert_eq!(result.salary, 15_000);

    store.data_mut().number = 0;
    instance.call_increment_twice(&mut store)?;
    assert_eq!(store.data().number, 2);

    let result = instance.call_add_all_and_one(
        &mut store, 10i32, 20i64, 30u32, 40u64, 50.25f32, 60.25f64, "70",
    )?;
    assert_eq!(
        result,
        10.0 + 20.0 + 30.0 + 40.0 + 50.25 + 60.25 + 70.0 + 1.0
    );

    // multiple references to data
    let data1 = store.data();
    let data2 = store.data();
    assert_eq!(data1.number, data2.number);

    // TODO: need to manually drop read "references" before making a mutable one
    #[allow(warnings)]
    drop(data1);
    #[allow(warnings)]
    drop(data2);

    let result = instance.call_add_sub_one(&mut store, 5)?;
    assert_eq!(result, (6, 4));

    let result = instance.call_add_sub_twenty(&mut store, 5)?;
    assert_eq!(result, (25, -15));

    Ok(())
}
