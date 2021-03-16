use tlayuda::Tlayuda;

#[allow(dead_code)]
#[derive(Tlayuda)]
pub struct Student {
    id: u64,
    first_name: String,
    last_name: String,
    telephone: String,
    date_of_birth: String,
    final_grade: u32,
}

struct StudentsPartitionedByGrade {
    a_students: Vec<Student>,
    b_students: Vec<Student>,
    c_students: Vec<Student>,
    d_students: Vec<Student>,
    f_students: Vec<Student>,
}

fn group_students_by_grade(mut students: Vec<Student>) -> StudentsPartitionedByGrade {
    let result = StudentsPartitionedByGrade {
        a_students: Vec::new(),
        b_students: Vec::new(),
        c_students: Vec::new(),
        d_students: Vec::new(),
        f_students: Vec::new(),
    };

    students.drain(..).fold(result, |mut acc, student| {
        match student.final_grade {
            90..=100 => acc.a_students.push(student),
            80..=89 => acc.b_students.push(student),
            70..=79 => acc.c_students.push(student),
            60..=69 => acc.d_students.push(student),
            0..=50 => acc.f_students.push(student),
            _ => (),
        }

        acc
    })
}

#[test]
fn test_group_students_by_grade() {
    // sets up a vec of students with 10 students per number grade
    let students = Student::tlayuda() // create tlayuda test builder
        .set_final_grade(|index| (index % 101) as u32) // returns a 0-100 grade based on index
        .build_vec(200); // creates a vec of 200 students

    // call function we're testing
    let result = group_students_by_grade(students);

    // verifies expected # of students per group
    assert_eq!(20, result.a_students.len());
    assert_eq!(20, result.b_students.len());
    assert_eq!(20, result.c_students.len());
    assert_eq!(20, result.d_students.len());
    assert_eq!(102, result.f_students.len());

    // verifies every group has the correct grade range
    result
        .a_students
        .iter()
        .for_each(|s| assert!(s.final_grade >= 90 && s.final_grade <= 100));
    result
        .b_students
        .iter()
        .for_each(|s| assert!(s.final_grade >= 80 && s.final_grade < 90));
    result
        .c_students
        .iter()
        .for_each(|s| assert!(s.final_grade >= 70 && s.final_grade < 80));
    result
        .d_students
        .iter()
        .for_each(|s| assert!(s.final_grade >= 60 && s.final_grade < 70));
    result
        .f_students
        .iter()
        .for_each(|s| assert!(s.final_grade <= 50));
}
