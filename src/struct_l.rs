#[derive(Debug)]
struct Student {
    name: String,
    roll: i32,
    section: String,
}

pub fn student_printer() {
    let student_info = Student {
        name: "Sakib".to_string(),
        roll: 123,
        section: "A".to_string(),
    };
    println!(
        "Student info : \nname:{}\n roll:{}\n section:{}",
        student_info.name, student_info.roll, student_info.roll
    );
    //debuging
    println!("full struct {:?}", student_info);
}
