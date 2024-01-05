wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "resources",
    exports: {
        "component-test:wit-protocol/employees": MyEmployees,
        "component-test:wit-protocol/employees/employee": MyEmployee,
    }
});

struct MyEmployees;

impl exports::component_test::wit_protocol::employees::Guest for MyEmployees {
    fn find_job(
        employee: &MyEmployee,
        companies: wit_bindgen::rt::vec::Vec<
            exports::component_test::wit_protocol::employees::Company,
        >,
    ) -> Option<exports::component_test::wit_protocol::employees::Company> {
        companies
            .into_iter()
            .find(|company| employee.min_salary <= company.get_max_salary())
    }
}

pub struct MyEmployee {
    name: String,
    min_salary: u32,
}

impl exports::component_test::wit_protocol::employees::GuestEmployee for MyEmployee {
    fn new(name: String, min_salary: u32) -> Self {
        Self { name, min_salary }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_min_salary(&self) -> u32 {
        self.min_salary
    }
}
