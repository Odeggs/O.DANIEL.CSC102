use std::fs::File;
use std::io::Write;

fn main() {
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric_number: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric_number: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: 100,
        },
        Student{


        }
    ];

    for student in &students {
        println!(
            "3 {}, {}, {} ,{}",
            student.name, student.matric_number, student.department, student.level
        );
    }

    let mut file = File::create("student_data.xls").expect("Unable to create file");

    for student in &students {
        writeln!(
            file,
            "3 {} {} {} {}",
            student.name, student.matric_number, student.department, student.level
        )
        .expect("Unable to write data to file");
    }
}

struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: usize,
}