/*
Using a hash map and vectors, 
create a text interface to allow 
a user to add employee names 
to a department in a company. 
For example, “Add Sally to Engineering” or 
“Add Amir to Sales.” 
Then let the user retrieve a list of all people 
in a department or all people in 
the company by department, sorted alphabetically.
*/
use std::collections::HashMap;
use std::io;


struct DB {
    department: HashMap<String, Vec<String>>,
}

impl DB {
    fn add_emp(&mut self, department: String, name: String) {
        let dept = self.department.get_mut(&department);
        if let Some(d) = dept {
            d.push(name);
        }
        println!("Updated list:\n{:?}", self.department)
    }

    fn get_all(&self) -> Vec<String> {
        self.department.values().cloned().flat_map(|d| d).collect()
    }

    fn get_all_by_dept(&self, department: String) -> Vec<String> {
        let dept = self.department.get(&department);
        let x: Vec<String> = Vec::new();
        dept.unwrap_or(&x).to_vec()
    }

    fn seed_departments(&mut self, departments: Vec<String>) {
        self.department.clear();
        for d in departments {
            self.department.insert(d, Vec::new());
        }        

    }
}

fn main() {
    let mut db = DB{ department:HashMap::new() };
    db.seed_departments(departments());
    let mut err_msg: Option<String> = None;
    'menu: loop{
        clear_screen(&err_msg);
        println!("**Employee Records Book**");
        println!("\t1. Add Employee");
        println!("\t2. Retrieve Data");
        println!("\t3. Exit");
        
        let mut choice = String::new();
        read_input(&mut choice);

        match choice.as_str().trim() {
            "1" => show_add_emp_menu(&mut db),
            "2" => show_retrieve_menu(&db),
            "3" => break 'menu,
            _ => {
                err_msg = Some(String::from("Invalid Choice. Try again"));
                continue
            }
        }
    }
}

fn show_add_emp_menu(db: &mut DB) {
    let departments = departments();
    let mut err_msg: Option<String> = None;
    loop{
        clear_screen(&err_msg);
        println!("**Add Employees**\nChoose department:");
        for (i, dep) in departments.iter().enumerate() {
            println!("\t{} {}", i+1, dep);
        }
        let go_back = departments.len()+1;
        println!("\t{} Back", go_back);

        let mut choice = String::new();
        read_input(&mut choice);
            
        let choice = choice.trim().parse();
        if choice.is_err() {
            err_msg = Some(String::from("Invalid choice. Try again"));
            continue
        }

        let dep_num: usize = choice.unwrap();
        
        if dep_num == go_back {
            return;
        }

        if dep_num > departments.len() {
            err_msg = Some(String::from("Invalid choice. Try again"));
            continue;
        }

        println!("Name of the employee:");
        let mut name = String::new();
        read_input(&mut name);

        let dep_name = departments.get(dep_num-1).expect("department does not exist!");
        add_employee(db, String::from(name.trim()), dep_name.to_string());
        break;
    }
}

fn show_retrieve_menu(db: &DB) {
    let mut err_msg: Option<String> = None;
    loop{
        clear_screen(&err_msg);
        println!("\n**Retrieve Employees**\nChoose department:");
        let departments = departments();
        for (i, dep) in departments.iter().enumerate() {
            println!("\t{} {}", i+1, dep);
        }
        println!("\t{} Show all", departments.len()+1);
        println!("\t{} Back", departments.len()+2);
        let mut choice = String::new();
        read_input(&mut choice);
        let choice = choice.trim().parse();
        if choice.is_err() {
            err_msg = Some(String::from("Invalid choice. Try again"));
            continue
        }

        let dep_num: usize = choice.unwrap();
        if dep_num == departments.len()+1 {
            println!("{:?}", db.get_all());
            press_enter_to_continue();
            continue;
        } else if dep_num == departments.len()+2 { // go back
            return
        } else if dep_num > departments.len()+2 {
            continue
        }

        let dep_name = departments.get(dep_num-1).unwrap();
        println!("{:?}", db.get_all_by_dept(String::from(dep_name)));
        press_enter_to_continue();
    }
}

fn departments() -> Vec<String> {
    vec![
        String::from("Engineering"), 
        String::from("Sales"), 
        String::from("Finance"),
    ]
}

fn clear_screen(message: &Option<String>) {
    std::process::Command::new("clear").status().unwrap().success();
    
    match message {
        Some(msg) => println!("{}", msg),
        None => ()
    }
}

fn read_input(input: &mut String) {
    io::stdin()
        .read_line(input)
        .expect("Failed to read line");
}

fn press_enter_to_continue() {
    println!("Press enter to continue");
    let mut wait = String::new();
    read_input(&mut wait)
}

fn add_employee(db: &mut DB, name: String, dep_name: String) {
    println!("Adding {} to {}", name, dep_name);
    db.add_emp(dep_name, name);
    press_enter_to_continue();
}
