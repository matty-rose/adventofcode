pub struct DayCommand {
    pub name: &'static str,
    pub func: Function,
}

pub struct Function {
    pub func: fn(&str, Option<&str>) -> Option<()>,
}

impl DayCommand {
    pub const fn new(name: &'static str, func: Function) -> Self {
        Self { name, func }
    }

    pub fn execute(&self, part: &str, file: Option<&str>) -> Option<()> {
        let executor = self.func.func;
        executor(part, file)
    }
}

inventory::collect!(DayCommand);
