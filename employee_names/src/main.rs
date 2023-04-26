use prettytable::{row, Table};
use std::collections::HashMap;
use std::io;

fn employee_database(details: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut employee_database: HashMap<String, Vec<String>> = HashMap::new();

    for (department, employee) in details {
        employee_database
            .entry(department)
            .or_insert(vec![])
            .push(employee);
    }

    employee_database
}

fn read_input() -> String {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    input_line
}

fn main() {
    let eng_details: Vec<(String, String)> = vec![
        ("Engineering".to_string(), "John".to_string()),
        ("Engineering".to_string(), "Jane".to_string()),
        ("Engineering".to_string(), "Jack".to_string()),
        ("Engineering".to_string(), "Jill".to_string()),
        ("Engineering".to_string(), "Jenny".to_string()),
        ("Engineering".to_string(), "Jen".to_string()),
        ("Engineering".to_string(), "John".to_string()),
    ];

    let sales_details: Vec<(String, String)> = vec![
        ("Sales".to_string(), "Sally".to_string()),
        ("Sales".to_string(), "Bolaji".to_string()),
        ("Sales".to_string(), "Bola".to_string()),
        ("Sales".to_string(), "Bolade".to_string()),
        ("Sales".to_string(), "Bolatito".to_string()),
        ("Sales".to_string(), "Boluwatife".to_string()),
        ("Sales".to_string(), "Boluwatife".to_string()),
    ];

    // merge the two vectors
    let merged_details = [&eng_details[..], &sales_details[..]].concat();

    let mut employee_details = employee_database(merged_details);

    //Let's add input for stdin to add more employees
    let mut stdinput = String::new();

    io::stdin()
        .read_line(&mut stdinput)
        .expect("Failed to read line");

    
    // Let add as many as possible employees through stdin and break when the user enters "continue"
    while let Some(first_word) = stdinput.split_whitespace().next() {

        let mut parsed_input: Vec<&str>;
        let mut new_input = String::new();
        let mut read_result: String;
    
        let mut input_line = String::new();
    
        let compare: &String = &"continue".to_string();

        if first_word == compare {
            break;
        }

        read_result = read_input();
        // turn read_result into a vec of &str
        let returned_result = read_result.split(" ").collect();
        parsed_input = returned_result;

        let department = parsed_input[0].to_string();
        let employee = parsed_input[1].to_string();
    
        employee_details
            .entry(department)
            .or_insert(vec![])
            .push(employee);
    
        new_input.push_str(&parsed_input.join(" "));
        new_input.push('\n');
    
        // update input_line variable at the end of each iteration
        input_line.push_str(&new_input);
    }

    // Retrieve the details by department and sort alphatically
    let mut eng_details = employee_details.get("Engineering").unwrap().clone();
    let mut sales_details = employee_details.get("Sales").unwrap().clone();

    eng_details.sort();
    sales_details.sort();

    // create a table with engineering and sales details and print to stdout
    let mut table = Table::new();

    // add color to the titles
    table.set_titles(row![bFg->"Engineering", bFb->"Sales"]);

    for (eng, sales) in eng_details.iter().zip(sales_details.iter()) {
        table.add_row(row![eng, sales]);
    }

    table.printstd();

    println!(
        "Engineering: {:?}\n Sales: {:?}",
        eng_details, sales_details
    );
}
