//! Examples based on AutoFac 'getting started' example
//! (http://autofac.readthedocs.io/en/latest/getting-started/index.html)

use shaku_derive::Component;

// IOutput & ConsoleOutput implementation
// ---------------------------------------------------------------------
pub trait IOutput : Send {
    fn write(&self, content: String);
    fn get_date(&self, content: String) -> String;
}

#[derive(Component)]
#[interface(IOutput)]
pub struct ConsoleOutput {
    prefix: String,
    other_param: usize,
}

impl IOutput for ConsoleOutput {
    fn write(&self, content: String) {
        println!(
            "[Outputting to the console] {} #{} {}",
            self.prefix,
            self.other_param,
            content
        );
    }

    fn get_date(&self, content: String) -> String {
        format!(
            "{}#{} {}",
            self.prefix,
            self.other_param,
            content
        )
    }
}

// IDateWriter & TodayWriter implementation
// ---------------------------------------------------------------------
pub trait IDateWriter : Send {
    fn write_date(&self);
    fn get_date(&self) -> String;
}

#[derive(Component)]
#[interface(IDateWriter)]
pub struct TodayWriter {
    #[inject]
    output: Box<dyn IOutput>,
    today: String,
}

impl IDateWriter for TodayWriter {
    fn write_date(&self) {
        let mut content = "Today is ".to_string();
        content.push_str(self.today.as_str());
        self.output.write(content);
    }

    fn get_date(&self) -> String {
        let mut content = "Today is ".to_string();
        content.push_str(self.today.as_str());
        self.output.get_date(content)
    }
}
