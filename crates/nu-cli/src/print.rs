use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, Example, IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};

#[derive(Clone)]
pub struct Print;

impl Command for Print {
    fn name(&self) -> &str {
        "print"
    }

    fn signature(&self) -> Signature {
        Signature::build("print")
            .rest("rest", SyntaxShape::Any, "the values to print")
            .switch(
                "no-newline",
                "print without inserting a newline for the line ending",
                Some('n'),
            )
            .category(Category::Strings)
    }

    fn usage(&self) -> &str {
        "Prints the values given"
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["display"]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let args: Vec<Value> = call.rest(engine_state, stack, 0)?;
        let no_newline = call.has_flag("no-newline");
        let head = call.head;

        for arg in args {
            arg.into_pipeline_data()
                .print(engine_state, stack, no_newline)?;
        }

        Ok(PipelineData::new(head))
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Print 'hello world'",
                example: r#"print "hello world""#,
                result: None,
            },
            Example {
                description: "Print the sum of 2 and 3",
                example: r#"print (2 + 3)"#,
                result: None,
            },
        ]
    }
}
