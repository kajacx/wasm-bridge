wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "resources",
});

struct MyEmployees;

impl Guest for MyEmployees {
    fn create_company(name: String) -> String {
        let company = component_test::wit_protocol::companies::Company::new(&name, 80_000);
        company.get_name().to_owned()
    }

    fn company_roundtrip(company: Company) -> Company {
        company
    }
}

export!(MyEmployees);
