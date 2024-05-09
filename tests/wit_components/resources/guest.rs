use std::cell::Cell;

wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "resources",
});

use component_test::wit_protocol::companies::CompanyRes;
use exports::component_test::wit_protocol::employees::EmployeeRes;
use exports::component_test::wit_protocol::employees::GuestEmployeeRes;

struct MyEmployees;

impl exports::component_test::wit_protocol::employees::Guest for MyEmployees {
    type EmployeeRes = MyEmployee;
}

impl exports::component_test::wit_protocol::guest_fns::Guest for MyEmployees {
    fn company_roundtrip(company: CompanyRes) -> CompanyRes {
        let name = company.get_name();
        let name = name + " round";
        company.set_name(&name);
        component_test::wit_protocol::host_fns::company_roundtrip(company)
    }

    fn employee_roundtrip(employee: EmployeeRes) -> EmployeeRes {
        let name = employee.get::<MyEmployee>().get_name();
        let name = name + " round trip";
        employee.get::<MyEmployee>().set_name(name);
        employee
    }

    fn method_find_job(employee: EmployeeRes, companies: Vec<CompanyRes>) -> Option<CompanyRes> {
        companies
            .into_iter()
            .find(|company| employee.get::<MyEmployee>().min_salary <= company.get_max_salary())
    }
}

pub struct MyEmployee {
    name: Cell<String>,
    min_salary: u32,
}

impl exports::component_test::wit_protocol::employees::GuestEmployeeRes for MyEmployee {
    fn new(name: String, min_salary: u32) -> Self {
        Self {
            name: Cell::new(name),
            min_salary,
        }
    }

    fn get_name(&self) -> String {
        let name = self.name.take();
        self.name.set(name.clone());
        name
    }

    fn set_name(&self, name: String) {
        self.name.set(name)
    }

    fn get_min_salary(&self) -> u32 {
        self.min_salary
    }
}

impl Guest for MyEmployees {
    fn simple() {
        log("Hello guest")
    }
}

export!(MyEmployees);
