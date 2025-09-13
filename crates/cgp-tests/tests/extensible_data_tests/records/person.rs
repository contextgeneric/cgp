use cgp::core::field::impls::CanBuildFrom;
use cgp::extra::dispatch::{BuildAndMerge, BuildAndSetField, BuildWithHandlers};
use cgp::prelude::*;

#[derive(CgpData)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

#[derive(CgpData)]
pub struct Employee {
    pub employee_id: u64,
    pub first_name: String,
    pub last_name: String,
}

#[derive(CgpData)]
pub struct EmployeeId {
    pub employee_id: u64,
}

#[cgp_producer]
pub fn build_person() -> Person {
    Person {
        first_name: "John".to_owned(),
        last_name: "Smith".to_owned(),
    }
}

#[cgp_producer]
pub fn build_employee_id() -> u64 {
    1
}

#[test]
fn test_person() {
    let person = Person {
        first_name: "John".to_owned(),
        last_name: "Smith".to_owned(),
    };

    let _employee: Employee = Employee::builder() // PartialEmployee<IsNothing, IsNothing, IsNothing>
        .build_from(person) // PartialEmployee<IsNothing, IsPresent, IsPresent>
        .build_field(PhantomData::<Symbol!("employee_id")>, 1) // PartialEmployee<IsPresent, IsPresent, IsPresent>
        .finalize_build(); // Person
}

#[test]
fn test_person2() {
    let person = Person {
        first_name: "John".to_owned(),
        last_name: "Smith".to_owned(),
    };

    let employee_id = EmployeeId { employee_id: 1 };

    let _employee = Employee::builder() // PartialEmployee<IsNothing, IsNothing, IsNothing>
        .build_from(person) // PartialEmployee<IsNothing, IsPresent, IsPresent>
        .build_from(employee_id) // PartialEmployee<IsPresent, IsPresent, IsPresent>
        .finalize_build(); // Person
}

#[test]
fn test_build_with_handler() {
    let _employee = BuildWithHandlers::<
        Employee,
        Product![BuildAndMerge<BuildPerson>, BuildAndSetField<Symbol!("employee_id"), BuildEmployeeId>],
    >::compute(&(), PhantomData::<()>, ());
}
