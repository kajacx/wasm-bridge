use component_test::wit_protocol::companies;
use std::collections::HashMap;
use wasm_bridge::{
    component::{Component, Linker, Resource},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "resources"
});

#[derive(Default, Clone)]
struct MyCompany {
    name: String,
    max_salary: u32,
}

#[derive(Default, Clone)]
struct ResHolder<T> {
    resources: HashMap<u32, T>,
    next_id: u32,
}

impl<T> ResHolder<T> {
    fn new(&mut self, item: T) -> u32 {
        let id = self.next_id;
        self.resources.insert(id, item);
        self.next_id += 1;
        id
    }

    fn get(&self, id: u32) -> Option<&T> {
        self.resources.get(&id)
    }

    fn drop(&mut self, id: u32) -> Result<(), ()> {
        self.resources.remove(&id).map(|_| ()).ok_or(())
    }
}

#[derive(Default, Clone)]
struct State {
    companies: ResHolder<MyCompany>,
}

impl companies::HostCompany for State {
    fn new(&mut self, name: String, max_salary: u32) -> Result<Resource<companies::Company>> {
        Ok(Resource::new_own(
            self.companies.new(MyCompany { name, max_salary }),
        ))
    }

    fn get_name(&mut self, self_: Resource<companies::Company>) -> Result<String> {
        Ok(self.companies.get(self_.rep()).unwrap().name.clone())
    }

    fn get_max_salary(&mut self, self_: Resource<companies::Company>) -> Result<u32> {
        Ok(self.companies.get(self_.rep()).unwrap().max_salary)
    }

    fn drop(&mut self, rep: Resource<companies::Company>) -> Result<()> {
        self.companies.drop(rep.rep()).unwrap();
        Ok(())
    }
}

impl companies::Host for State {}

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, State::default());

    let component = Component::new(&store.engine(), &component_bytes).expect("create component");

    let mut linker = Linker::new(store.engine());

    Resources::add_to_linker(&mut linker, |state| state).unwrap();
    let (instance, _) = Resources::instantiate(&mut store, &component, &linker).unwrap();

    let employees = instance.component_test_wit_protocol_employees().employee();
    let employee = employees
        .call_constructor(&mut store, "Mike".into(), 50_000)
        .unwrap();
    assert_eq!(
        employees.call_get_name(&mut store, employee).unwrap(),
        "Mike"
    );

    let company1 = Resource::new_own(store.data_mut().companies.new(MyCompany {
        name: "Company1".into(),
        max_salary: 30_000,
    }));

    let result = instance
        .component_test_wit_protocol_employees()
        .call_find_job(&mut store, employee, &[company1])
        .unwrap();
    assert!(result.is_none());

    let company1 = Resource::new_own(store.data_mut().companies.new(MyCompany {
        name: "Company1".into(),
        max_salary: 30_000,
    }));
    let company2 = Resource::new_own(store.data_mut().companies.new(MyCompany {
        name: "Company2".into(),
        max_salary: 60_000,
    }));

    let result = instance
        .component_test_wit_protocol_employees()
        .call_find_job(&mut store, employee, &[company1, company2])
        .unwrap();
    assert_eq!(
        store
            .data()
            .companies
            .get(result.unwrap().rep())
            .unwrap()
            .name,
        "Company2"
    );

    Ok(())
}
