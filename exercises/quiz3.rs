// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// 定义一个 Trait 来返回成绩的字符串表示
pub trait DisplayGrade {
    fn display(&self) -> String;
}

// 使报告卡结构体支持泛型参数 T
pub struct ReportCard<T: DisplayGrade> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

// 实现 ReportCard 的打印功能，使用 display 方法来获取成绩的字符串表示
impl<T: DisplayGrade> ReportCard<T> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name,
            &self.student_age,
            self.grade.display()
        )
    }
}

// 为 f32 类型实现 DisplayGrade trait，以便可以返回数字成绩的字符串表示
impl DisplayGrade for f32 {
    fn display(&self) -> String {
        self.to_string()
    }
}

// 为 &str 类型实现 DisplayGrade trait，以便可以返回字母成绩的字符串表示
impl DisplayGrade for &str {
    fn display(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // 修改 grade 为字母评分
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
