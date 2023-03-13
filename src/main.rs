use std::io;

struct Student {
    grades: Vec<f32>
}

impl Student {
    fn new() -> Student {
        Student {
            grades: Vec::new()
        }
    }

    fn add_grade(&mut self, grade: f32) {
        self.grades.push(grade)
    }

    fn avg_grades(&self) -> f32 {
        let total: f32 = self.grades.iter().sum();
    
        total / self.grades.len() as f32
    }
}

fn main() {

    let grades = input_to_back();

    let mut student = Student::new();
    
    for grade in grades {
        student.add_grade(grade)
    }
    
    println!("{}", student.avg_grades())
    
}

fn input_to_back() -> Vec<f32> {
    let mut grades = Vec::new();

    println!("How many grades to insert: ");

    let mut input_grades = String::new();

    io::stdin().read_line(&mut input_grades).unwrap();

    println!("Tip the {} grades: ", input_grades.trim());

    for _ in 0..input_grades.trim().parse::<i8>().unwrap() {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        if let Ok(float_value) = input.trim().parse::<f32>() {
            grades.push(float_value)
        }

        if let Ok(int_value) = input.trim().parse::<i32>() {
            grades.push(int_value as f32)
        }
    }

    grades

}

#[test]
fn it_should_be_a_test() {
    let grades: Vec<f32> = vec![10.0, 7.0, 6.5, 6.5];

    let mut student = Student::new();
    
    for grade in grades {
        student.add_grade(grade)
    }
    
    assert_eq!(7.5, student.avg_grades())
}
