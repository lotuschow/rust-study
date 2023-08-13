 // 定义学生结构体
struct Student {
    id: u32,
    name: String,
    age: u32,
    class_id: u32,
}

// 定义班级结构体
struct Class {
    id: u32,
    name: String,
}

// 定义课程结构体
struct Course {
    id: u32,
    name: String,
}

// 学生管理系统
struct StudentManagement {
    students: Vec<Student>,
    classes: Vec<Class>,
    courses: Vec<Course>,
}

impl StudentManagement {
    // 添加学生
    fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    // 获取所有学生
    fn get_students(&self) -> &Vec<Student> {
        &self.students
    }

    // 添加班级
    fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }

    // 获取所有班级
    fn get_classes(&self) -> &Vec<Class> {
        &self.classes
    }

    // 添加课程
    fn add_course(&mut self, course: Course) {
        self.courses.push(course);
    }

    // 获取所有课程
    fn get_courses(&self) -> &Vec<Course> {
        &self.courses
    }
}

fn main() {
    let mut student_management = StudentManagement {
        students: Vec::new(),
        classes: Vec::new(),
        courses: Vec::new(),
    };

    // 添加班级和课程
    student_management.add_class(Class { id: 1, name: "Class A".to_string() });
    student_management.add_class(Class { id: 2, name: "Class B".to_string() });
    student_management.add_course(Course { id: 1, name: "Math".to_string() });
    student_management.add_course(Course { id: 2, name: "English".to_string() });

    // 添加学生
    student_management.add_student(Student {
        id: 1,
        name: "Alice".to_string(),
        age: 18,
        class_id: 1,
    });

    student_management.add_student(Student {
        id: 2,
        name: "Bob".to_string(),
        age: 19,
        class_id: 2,
    });

    // 打印学生信息
    for student in student_management.get_students() {
        println!(
            "Student ID: {}, Name: {}, Age: {}, Class ID: {}",
            student.id, student.name, student.age, student.class_id
        );
    }

    // 打印班级信息
    for class in student_management.get_classes() {
        println!("Class ID: {}, Name: {}", class.id, class.name);
    }

    // 打印课程信息
    for course in student_management.get_courses() {
        println!("Course ID: {}, Name: {}", course.id, course.name);
    }
}
