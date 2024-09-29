use std::process::{ExitStatus, Output};

use crate::executors::executor_trait::ExecutorTrait;
use crate::task::CodeLanguage;

pub struct NoopExecutor {}

impl NoopExecutor {
    pub fn new() -> Self {
        NoopExecutor {}
    }
}

impl ExecutorTrait for NoopExecutor {
    async fn run(&self, code: String, language: CodeLanguage) -> Output {
        println!("NoopExecutor is running for language: {:?}", language);
        println!("NoopExecutor is running the code: {}", code);
        Output {
            stdout: Vec::new(),
            stderr: Vec::new(),
            status: ExitStatus::default(),
        }
    }
}
