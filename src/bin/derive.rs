#[derive(Debug, Clone, Copy)]
enum Position {
  Manager,
  Supervisor,
  Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
  position: Position,
  work_hours: i64,
}

fn print_employee(emp: Employee) {
  println!("{:?}", emp);
}

fn main() {
  let me = Employee {
    position: Position::Worker,
    work_hours: 40,
  };
  print_employee(me);
  print_employee(me);

  let supervisor = Employee {
    position: Position::Supervisor,
    work_hours: 44,
  };
  print_employee(supervisor);
  print_employee(supervisor);

  let manager = Employee {
    position: Position::Manager,
    work_hours: 44,
  };
  print_employee(manager);
  print_employee(manager);
}
