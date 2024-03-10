// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.


// use std::fmt::Display;

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(self) -> Self {
        format!("{}Bar", self)
    }
}

// // kita bisa extend struct bawaan rust dengan fungsi yang kita punya
// impl AppendBar for Vec<String> {
//     fn append_bar(self) -> Self {
//         let mut new_vec = self.clone();
//         new_vec.push("Bar".to_string());
//         new_vec
//     }
// }

// trait PrintSesuatu {
//     fn print_sesuatu(&self);
// }
//
// impl<T: Display> PrintSesuatu for T {
//     fn print_sesuatu(&self) {
//         println!("print sesuatu: {}", self);
//     }
// }

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);

    // s.print_sesuatu();
    // 1.print_sesuatu();


    // vec!["Foo".to_string()].append_bar();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
