use std::fs::File;
use std::io::{self, Write};
use std::process::Output;

use uuid::Uuid;

use super::{local_executor::ExecutorTrait, task::CodeLanguage};

pub struct DockerExecutor {}

impl DockerExecutor {
    pub fn new() -> Self {
        DockerExecutor {}
    }

    async fn prepare(code: String, language: CodeLanguage) -> String {
        let file_extension = match language {
            CodeLanguage::Shell => "sh",
            CodeLanguage::Node => "js",
            CodeLanguage::Python => "py",
            _ => panic!("Unsupported language"),
        };

        let file_name = format!("/tmp/{}.{}", Uuid::new_v4(), file_extension);

        let mut file = std::fs::File::create(&file_name).expect("Failed to create file");
        file.write_all(code.as_bytes())
            .expect("Failed to write to file");

        file_name
    }
}

impl ExecutorTrait for DockerExecutor {
    async fn run(&self, code: String, language: CodeLanguage) -> Output {
        let file_name = DockerExecutor::prepare(code, language).await;

        let container_name = match language {
            CodeLanguage::Shell => "alpine",
            CodeLanguage::Node => "node",
            CodeLanguage::Python => "python",
            _ => panic!("Unsupported language"),
        };

        match language {
            CodeLanguage::Shell => std::process::Command::new("docker")
                .arg("run")
                .arg("--rm")
                .arg("-v")
                .arg(format!("{}:/tmp/code", file_name))
                .arg(container_name)
                .arg("sh")
                .arg("/tmp/code")
                .output()
                .expect("Failed to execute command"),
            CodeLanguage::Node => std::process::Command::new("docker")
                .arg("run")
                .arg("--rm")
                .arg("-v")
                .arg(format!("{}:/tmp/code", file_name))
                .arg(container_name)
                .arg("node")
                .arg("/tmp/code")
                .output()
                .expect("Failed to execute command"),
            CodeLanguage::Python => std::process::Command::new("docker")
                .arg("run")
                .arg("--rm")
                .arg("-v")
                .arg(format!("{}:/tmp/code", file_name))
                .arg(container_name)
                .arg("python")
                .arg("/tmp/code")
                .output()
                .expect("Failed to execute command"),
            _ => panic!("Unsupported language"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::local::{
        docker_executor::DockerExecutor, local_executor::ExecutorTrait, task::CodeLanguage,
    };

    #[tokio::test]
    #[ignore]
    async fn test_execute_shell() {
        let code = "echo 'Hello, World!'".to_string();
        let executor = DockerExecutor::new();
        let output = executor.run(code, CodeLanguage::Shell).await;

        assert_eq!(output.status.success(), true);
        assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, World!\n");
    }

    #[tokio::test]
    #[ignore]
    async fn test_execute_node() {
        let code = "console.log('Hello, World!')".to_string();
        let executor = DockerExecutor::new();
        let output = executor.run(code, CodeLanguage::Node).await;

        assert_eq!(output.status.success(), true);
        assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, World!\n");
    }

    #[tokio::test]
    #[ignore]
    async fn test_execute_python() {
        let code = "print('Hello, World!')".to_string();
        let executor = DockerExecutor::new();
        let output = executor.run(code, CodeLanguage::Python).await;

        assert_eq!(output.status.success(), true);
        assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, World!\n");
    }
}
