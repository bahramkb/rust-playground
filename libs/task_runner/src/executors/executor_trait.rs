use crate::task::CodeLanguage;
use std::process::Output;

#[trait_variant:: make(ExecutorTrait: Send)]
pub trait LocalExecutorTrait {
    async fn run(&self, code: String, language: CodeLanguage) -> Output;
}
