use wasm_bridge::{
    component::{Component, Linker, Resource, ResourceTable},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "resources",
    with: {
        "component-test:wit-protocol/companies/company": MyCompany
    }
});

#[derive(Default, Clone, Debug, PartialEq)]
pub struct MyCompany {
    pub name: String,
    pub max_salary: u32,
}

#[derive(Default)]
struct State {
    resources: ResourceTable,
}

impl State {
    fn new_company(&mut self, company: MyCompany) -> Resource<MyCompany> {
        self.resources.push(company).unwrap()
    }

    fn get_company(&self, company: &Resource<MyCompany>) -> &MyCompany {
        self.resources.get(company).unwrap()
    }

    fn get_company_mut(&mut self, company: &Resource<MyCompany>) -> &mut MyCompany {
        self.resources.get_mut(company).unwrap()
    }

    fn drop_company(&mut self, company: Resource<MyCompany>) {
        self.resources.delete(company).unwrap();
    }
}

impl component_test::wit_protocol::companies::HostCompany for State {
    fn new(&mut self, name: String, max_salary: u32) -> Result<Resource<MyCompany>> {
        Ok(self.new_company(MyCompany { name, max_salary }))
    }

    fn get_name(&mut self, self_: Resource<MyCompany>) -> Result<String> {
        Ok(self.get_company(&self_).name.clone())
    }

    fn set_name(&mut self, self_: Resource<MyCompany>, name: String) -> Result<()> {
        self.get_company_mut(&self_).name = name;
        Ok(())
    }

    fn get_max_salary(&mut self, self_: Resource<MyCompany>) -> Result<u32> {
        Ok(self.get_company(&self_).max_salary)
    }

    fn drop(&mut self, rep: Resource<MyCompany>) -> Result<()> {
        Ok(self.drop_company(rep))
    }
}

impl component_test::wit_protocol::companies::Host for State {}

impl component_test::wit_protocol::host_fns::Host for State {
    fn company_roundtrip(&mut self, company: Resource<MyCompany>) -> Result<Resource<MyCompany>> {
        self.get_company_mut(&company).name += " trip";
        Ok(company)
    }
}

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, State::default());

    #[allow(deprecated)]
    let component = Component::new(&store.engine(), &component_bytes).unwrap();

    let mut linker = Linker::new(store.engine());
    Resources::add_to_linker(&mut linker, |state| state).unwrap();

    #[allow(deprecated)]
    let (instance, _) = Resources::instantiate(&mut store, &component, &linker).unwrap();
    let employees = instance.component_test_wit_protocol_employees().employee();
    let guest_fns = instance.component_test_wit_protocol_guest_fns();

    // Company roundtrip
    let company = store.data_mut().new_company(MyCompany {
        name: "CompanyName".into(),
        max_salary: 30_000,
    });

    let result = guest_fns
        .call_company_roundtrip(&mut store, company)
        .unwrap();
    assert_eq!(
        store.data().get_company(&result).name,
        "CompanyName round trip"
    );
    store.data_mut().drop_company(result);

    // Employee roundtrip
    let employee = employees
        .call_constructor(&mut store, "EmployeeName".into(), 50_000)
        .unwrap();
    let result = guest_fns
        .call_employee_roundtrip(&mut store, employee)
        .unwrap();
    assert_eq!(
        employees.call_get_name(&mut store, result).unwrap(),
        "EmployeeName round trip"
    );

    // Find job - no job found
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

    let result = guest_fns
        .call_find_job(&mut store, employee, &[company1])
        .unwrap();
    assert!(result.is_none());

    // Find job - job found
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

    let result = guest_fns
        .call_find_job(&mut store, employee, &[company1, company2])
        .unwrap() // WASM call
        .unwrap(); // Option return type
    assert_eq!(store.data().get_company(&result).name, "Company2");
    store.data_mut().drop_company(result);

    // TODO: assert that all resources have been deleted

    Ok(())
}
