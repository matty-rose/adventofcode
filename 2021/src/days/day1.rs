use crate::registry::{DayCommand, Function};

fn foo() -> Option<()> {
    println!("hello from test");
    None
}

inventory::submit! {
    DayCommand::new("test", Function{func: foo})
}
