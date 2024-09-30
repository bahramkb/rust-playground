use crate::executors::executor_trait::ExecutorTrait;
use crate::executors::noop_executor::NoopExecutor;
use crate::executors::{docker_executor::DockerExecutor, local_executor::LocalExecutor};
use crate::utils::{is_docker_running, is_docker_installed, is_running_in_github_actions};
use chrono::{DateTime, Utc};
use std::{marker::PhantomData, process::Output};
use uuid::Uuid;

pub struct EmptyTask;
pub struct ReadyTask;
pub struct FailedTask;
pub struct SuccessfulTask;

#[derive(Debug, Clone, Copy)]
pub enum CodeLanguage {
    None,
    Shell,
    Node,
    Python,
}

pub struct Task<S = EmptyTask> {
    id: Uuid,
    name: String,
    code_language: CodeLanguage,
    code: String,
    tries: u8,
    output: Option<Output>,
    started_at: Option<DateTime<Utc>>,
    finished_at: Option<DateTime<Utc>>,
    state: PhantomData<S>,
}

impl Task {
    pub fn new(name: String) -> Task<EmptyTask> {
        Task {
            id: Uuid::new_v4(),
            name,
            code_language: CodeLanguage::None,
            code: String::new(),
            tries: 0,
            output: None,
            started_at: None,
            finished_at: None,
            state: Default::default(),
        }
    }
}

impl<S> Task<S> {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Task<EmptyTask> {
    pub fn code(self, code_language: CodeLanguage, code: impl Into<String>) -> Task<ReadyTask> {
        Task {
            id: self.id,
            name: self.name,
            code: code.into(),
            code_language,
            tries: 0,
            output: None,
            started_at: None,
            finished_at: None,
            state: PhantomData::<ReadyTask>,
        }
    }
}

impl Task<ReadyTask> {
    pub async fn execute(&mut self) -> Result<Task<SuccessfulTask>, Task<FailedTask>> {
        self.started_at = Some(Utc::now());
        self.tries += 1;

        self.output = match is_docker_installed().await && is_docker_running().await {
            true => Some(
                DockerExecutor::new()
                    .run(self.code.clone(), self.code_language)
                    .await,
            ),
            false => match is_running_in_github_actions().await {
                true => Some(
                    NoopExecutor::new()
                        .run(self.code.clone(), self.code_language)
                        .await,
                ),
                false => Some(
                    LocalExecutor::new()
                        .run(self.code.clone(), self.code_language)
                        .await,
                ),
            },
        };

        self.finished_at = Some(Utc::now());

        if self.output.as_ref().unwrap().status.success() {
            Ok(Task {
                id: self.id,
                name: self.name.clone(),
                code: self.code.clone(),
                code_language: self.code_language,
                tries: self.tries,
                output: self.output.take(),
                started_at: self.started_at,
                finished_at: self.finished_at,
                state: PhantomData::<SuccessfulTask>,
            })
        } else {
            Err(Task {
                id: self.id,
                name: self.name.clone(),
                code: self.code.clone(),
                code_language: self.code_language,
                tries: self.tries,
                output: self.output.take(),
                started_at: self.started_at,
                finished_at: self.finished_at,
                state: PhantomData::<FailedTask>,
            })
        }
    }
}

impl Task<FailedTask> {
    pub fn retry(self) -> Task<ReadyTask> {
        if self.tries >= 3 {
            panic!("Task has reached the maximum number of retries");
        }

        Task {
            id: self.id,
            name: self.name,
            code: self.code,
            code_language: self.code_language,
            tries: self.tries,
            output: None,
            started_at: None,
            finished_at: None,
            state: PhantomData::<ReadyTask>,
        }
    }

    pub fn output(&self) -> &Output {
        self.output.as_ref().unwrap()
    }

    pub fn duration(&self) -> i64 {
        self.finished_at.unwrap().timestamp() - self.started_at.unwrap().timestamp()
    }
}

impl Task<SuccessfulTask> {
    pub fn output(&self) -> &Output {
        self.output.as_ref().unwrap()
    }

    pub fn duration(&self) -> i64 {
        self.finished_at.unwrap().timestamp_millis() - self.started_at.unwrap().timestamp_millis()
    }
}

#[cfg(test)]
mod tests {
    use crate::task::CodeLanguage;

    use super::Task;

    #[tokio::test]
    async fn a_nodejs_task_can_execute_and_return_result() {
        let mut task = Task::new("js-initial-code".to_string());
        task.set_name("js-code".to_string());
        let mut task = task.code(
            CodeLanguage::Node,
            "console.log(`this is javascript: ${2 + 2}`)".to_string(),
        );
        let output = task.execute().await;

        assert!(output.is_ok());

        match output {
            Ok(task) => {
                let stdout = std::str::from_utf8(&task.output().stdout).unwrap();
                println!("Task executed successfully: {}", stdout);
                println!("Task duration: {:?} milliseconds", task.duration());

                assert!(task.output().status.success());
                assert_eq!(task.output().stdout, b"this is javascript: 4\n");
            }
            Err(task) => {
                println!("Task failed: {:?}", task.output().stderr);
                println!("Task duration: {:?} milliseconds", task.duration());
            }
        }
    }

    #[tokio::test]
    async fn a_python_task_can_execute_and_return_result() {
        let task = Task::new("py-code".to_string());
        let mut task = task.code(
            CodeLanguage::Python,
            "print('this is python: {}'.format(2 + 2))".to_string(),
        );
        let output = task.execute().await;

        assert!(output.is_ok());

        match output {
            Ok(task) => {
                let stdout = std::str::from_utf8(&task.output().stdout).unwrap();
                println!("Task executed successfully: {}", stdout);
                println!("Task duration: {:?} milliseconds", task.duration());

                assert!(task.output().status.success());
                assert_eq!(task.output().stdout, b"this is python: 4\n");
            }
            Err(task) => {
                println!("Task failed: {:?}", task.output().stderr);
                println!("Task duration: {:?} milliseconds", task.duration());
            }
        }
    }

    #[tokio::test]
    async fn a_shell_task_can_execute_and_return_result() {
        let task = Task::new("shell-code".to_string());
        let mut task = task.code(
            CodeLanguage::Shell,
            r#"#!/bin/bash

    printf "this is shell: %d" "$((2 + 2))"
                "#
            .to_string(),
        );
        let output = task.execute().await;

        assert!(output.is_ok());

        match output {
            Ok(task) => {
                let stdout = std::str::from_utf8(&task.output().stdout).unwrap();
                println!("Task executed successfully: {}", stdout);
                println!("Task duration: {:?} milliseconds", task.duration());

                assert!(task.output().status.success());
                assert_eq!(task.output().stdout, b"this is shell: 4");
            }
            Err(task) => {
                println!("Task failed: {:?}", task.output().stderr);
                println!("Task duration: {:?} milliseconds", task.duration());
            }
        }
    }
}
