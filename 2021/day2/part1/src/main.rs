use std::{fs,env};


// 1. Load up data text and store the commands in an List of Enums (command: &str, value: u32)
// 2. Iterate through the list and execute commands in order. Store the accumulated horizontal and
// vertical position.
// 3. Return horizontal*vertical

struct Submarine {
    x: i32,
    y: i32,
}

impl Submarine {
    fn travel(&self, planned_course: PlannedCourse) -> (i32, i32){
        return planned_course.execute(self.x,self.y);
    }
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn go(&mut self, x: i32, y: i32) {
        self.x = self.x + x;
        self.y = self.y + y;
        println!("({}, {}) - ({}, {})",x,y,self.x, self.y);
    }

    fn cartesian(&self) -> (i32, i32) {
        return (self.x, self.y);
    }
}

struct PlannedCourse {
    plan: Vec<Command>
}

impl PlannedCourse {
    fn execute(&self,x: i32, y:i32) -> (i32, i32){
        let mut position = Position {x,y};
        for command in &self.plan {
            position.go(command.cartesian().0, command.cartesian().1);
        }
        return position.cartesian();
    }
}

struct Command {
    r#type: String,
    value: i32,
}

impl Command {
    fn cartesian(&self) -> (i32, i32){
        if &self.r#type == "up"{
            (self.value,0)
        }else if &self.r#type == "down" {
            (-self.value,0)
        }else {
            (0,self.value)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines : Vec<&str> = contents.split("\n").collect();
    let mut plan: Vec<Command> = Vec::new();
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let r#type: String =  split[0].to_string();
        let value: i32 = split[1].parse().unwrap();
        let command = Command {
            r#type,
            value
        };
        plan.push(command);
    }

    let submarine = Submarine {x: 0,y: 0};
    let course = PlannedCourse {plan: plan};
    let end = submarine.travel(course);

    println!("({} {}) -> {}",end.0,end.1,end.0*end.1);
}
