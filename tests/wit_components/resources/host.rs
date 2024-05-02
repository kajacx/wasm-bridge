use std::collections::HashMap;
use wasm_bridge::{
    component::{Component, Linker, Resource, ResourceTable},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "resources"
});

use component_test::wit_protocol::companies::Company;

#[derive(Default, Clone, Debug, PartialEq)]
struct MyCompany {
    name: String,
    max_salary: u32,
}

#[derive(Default)]
struct State {
    resources: ResourceTable,
}

impl State {
    fn new_company(&mut self, company: MyCompany) -> Resource<Company> {
        Resource::new_own(self.resources.push(company).unwrap().rep())
    }

    fn get_company(&self, company: &Resource<Company>) -> &MyCompany {
        self.resources
            .get(&Resource::<MyCompany>::new_borrow(company.rep()))
            .unwrap()
    }

    fn drop_company(&mut self, company: Resource<Company>) {
        self.resources
            .delete(Resource::<MyCompany>::new_own(company.rep()))
            .unwrap();
    }
}

impl component_test::wit_protocol::companies::HostCompany for State {
    fn new(&mut self, name: String, max_salary: u32) -> Result<Resource<Company>> {
        Ok(self.new_company(MyCompany { name, max_salary }))
    }

    fn get_name(&mut self, self_: Resource<Company>) -> Result<String> {
        Ok(self.get_company(&self_).name.clone())
    }

    fn get_max_salary(&mut self, self_: Resource<Company>) -> Result<u32> {
        Ok(self.get_company(&self_).max_salary)
    }

    fn drop(&mut self, rep: Resource<Company>) -> Result<()> {
        Ok(self.drop_company(rep))
    }
}

impl component_test::wit_protocol::companies::Host for State {
    fn company_roundtrip(&mut self, company: Resource<Company>) -> Result<Resource<Company>> {
        Ok(company)
    }
}

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, State::default());

    #[allow(deprecated)]
    let component = Component::new(&store.engine(), &component_bytes).expect("create component");

    let mut linker = Linker::new(store.engine());
    Resources::add_to_linker(&mut linker, |state| state).unwrap();

    #[allow(deprecated)]
    let (instance, _) = Resources::instantiate(&mut store, &component, &linker).unwrap();

    let employees = instance.component_test_wit_protocol_employees().employee();
    let employee = employees
        .call_constructor(&mut store, "Mike".into(), 50_000)
        .unwrap();
    assert_eq!(
        employees.call_get_name(&mut store, employee).unwrap(),
        "Mike"
    );

    let company1 = store.data_mut().new_company(MyCompany {
        name: "Company1".into(),
        max_salary: 30_000,
    });

    let result = instance
        .component_test_wit_protocol_employees()
        .call_find_job(&mut store, employee, &[company1])
        .unwrap();
    assert!(result.is_none());

    let employee = employees
        .call_constructor(&mut store, "Mike".into(), 50_000)
        .unwrap();

    let company1 = store.data_mut().new_company(MyCompany {
        name: "Company1".into(),
        max_salary: 30_000,
    });
    let company2 = store.data_mut().new_company(MyCompany {
        name: "Company2".into(),
        max_salary: 60_000,
    });

    let result = instance
        .component_test_wit_protocol_employees()
        .call_find_job(&mut store, employee, &[company1, company2])
        .unwrap() // WASM call
        .unwrap(); // Option return type
    assert_eq!(store.data().get_company(&result).name, "Company2");
    store.data_mut().drop_company(result);

    let company = store.data_mut().new_company(MyCompany {
        name: "Company Roundtrip".into(),
        max_salary: 30_000,
    });

    let result = instance
        .component_test_wit_protocol_employees()
        .call_company_roundtrip(&mut store, company)
        .unwrap();
    assert_eq!(store.data().get_company(&result).name, "Company Roundtrip");
    store.data_mut().drop_company(result);

    // TODO: this assert doesn't seem to work, how to check that all resources have been deleted?
    assert!(
        store
            .data_mut()
            .resources
            .iter_entries(HashMap::<_, MyCompany>::new())
            .next()
            .is_none(),
        "all companies should have been dropped by now"
    );

    Ok(())
}
