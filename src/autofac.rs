//! Examples based on AutoFac 'getting started' example
//! (http://autofac.readthedocs.io/en/latest/getting-started/index.html)

use std::sync::Arc;

#[cfg(test)]
use mockall::automock;
use shaku::{Component, Interface};

// IOutput & ConsoleOutput implementation
// ---------------------------------------------------------------------
#[cfg_attr(test, automock)]
pub trait IOutput: Interface {
    fn write(&self, content: String);
    fn format_output(&self, content: String) -> String;
}

#[derive(Component)]
#[shaku(interface = IOutput)]
pub struct ConsoleOutput {
    prefix: String,
    other_param: usize,
}

impl IOutput for ConsoleOutput {
    fn write(&self, content: String) {
        println!(
            "[Outputting to the console] {} #{} {}",
            self.prefix, self.other_param, content
        );
    }

    fn format_output(&self, content: String) -> String {
        format!("{}#{} {}", self.prefix, self.other_param, content)
    }
}

// IDateWriter & TodayWriter implementation
// ---------------------------------------------------------------------
pub trait IDateWriter: Interface {
    fn write_date(&self);
    fn get_date(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = IDateWriter)]
pub struct TodayWriter {
    #[shaku(inject)]
    output: Arc<dyn IOutput>,
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
        self.output.format_output(content)
    }
}

mod tests {
    use shaku::{ContainerBuildContext, ContainerBuilder, Error};
    use shaku::parameter::ParameterMap;

    use crate::autofac::{IDateWriter, IOutput, MockIOutput, TodayWriter};

    #[test]
    fn date_writer_formats_correctly() {
        // Create the mocked component data
        let build = |build_context: &mut ContainerBuildContext,
                     _params: &mut ParameterMap|
         -> Result<(), Error> {
            let mut mock = MockIOutput::new();
            mock.expect_format_output()
                .times(1)
                .returning(|content| format!("PREFIX: {}", content));

            build_context.insert_resolved_component::<dyn IOutput>(Box::new(mock));
            Ok(())
        };
        let dependencies = Vec::new();

        // Register components
        let mut builder = ContainerBuilder::new();
        builder.register_lambda::<dyn IOutput>("MockIOutput", Box::new(build), dependencies);
        builder
            .register_type::<TodayWriter>()
            .with_named_parameter("today", "TODAY".to_owned());
        let container = builder.build().unwrap();

        // Test date writer
        let date_writer = container.resolve_ref::<dyn IDateWriter>().unwrap();
        assert_eq!("PREFIX: Today is TODAY", date_writer.get_date());
    }
}
