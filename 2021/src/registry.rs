pub struct DayCommand {
    pub name: &'static str,
    pub func: Function,
}

pub struct Function {
    pub func: fn() -> Option<()>,
}

impl DayCommand {
    pub const fn new(name: &'static str, func: Function) -> Self {
        Self { name, func }
    }
}

inventory::collect!(DayCommand);
