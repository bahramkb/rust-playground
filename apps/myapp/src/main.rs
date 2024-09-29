use task_runner::task::{CodeLanguage, Task};

#[tokio::main]
async fn main() {
    println!("Creating a shell task");
    let task = Task::new("shell-code".to_string());

    let mut task = task.code(
        CodeLanguage::Shell,
        r#"#!/bin/bash

    printf "wonderful shell: %d" "$((2 + 2))"
            "#
        .to_string(),
    );

    println!("Executing the shell task");
    let output = task.execute().await;

    match output {
        Ok(task) => {
            let stdout = std::str::from_utf8(&task.output().stdout).unwrap();
            println!("Task executed successfully: {}", stdout);
            println!("Task duration: {:?} milliseconds", task.duration());
        }
        Err(task) => {
            let stdout = std::str::from_utf8(&task.output().stdout).unwrap();
            println!("Task failed: {}", stdout);
            println!("Task duration: {:?} milliseconds", task.duration());
        }
    }
}
