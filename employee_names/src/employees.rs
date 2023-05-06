use std::collections::HashMap;

pub fn employee_database(details: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut employee_database: HashMap<String, Vec<String>> = HashMap::new();

    for (department, employee) in details {
        employee_database
            .entry(department)
            .or_insert(vec![])
            .push(employee);
    }

    employee_database
}