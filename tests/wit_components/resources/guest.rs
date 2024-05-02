wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "resources",
});

struct MyEmployees;

impl exports::component_test::wit_protocol::employees::Guest for MyEmployees {
    type Employee = MyEmployee;

    fn find_job(
        employee: exports::component_test::wit_protocol::employees::EmployeeBorrow,
        companies: Vec<exports::component_test::wit_protocol::employees::Company>,
    ) -> Option<exports::component_test::wit_protocol::employees::Company> {
        companies
            .into_iter()
            .find(|company| employee.get::<MyEmployee>().min_salary <= company.get_max_salary())
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

    fn company_roundtrip(company: Company) -> Company {
        company_roundtrip_import(company)
    }
}

export!(MyEmployees);
