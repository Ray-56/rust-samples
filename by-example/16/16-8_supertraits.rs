trait Person {
    fn name(&self) -> String;
}

// Person is the parent trait of Student
// Implementing Student requires that you also impl Person
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student, computer science student) is a subclass of both Programmer and Student
// Implementing CompSciStudent requires you to impl both parent traits at the same time
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {}