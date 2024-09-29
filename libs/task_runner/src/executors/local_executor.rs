use std::process::Output;

use crate::executors::executor_trait::ExecutorTrait;
use crate::task::CodeLanguage;

pub struct LocalExecutor {}

impl LocalExecutor {
    pub fn new() -> Self {
        LocalExecutor {}
    }
}

impl ExecutorTrait for LocalExecutor {
    async fn run(&self, code: String, language: CodeLanguage) -> Output {
        let mut command = match language {
            CodeLanguage::Shell => {
                let mut command = std::process::Command::new("sh");
                command.arg("-c").arg(code);
                command
            }
            CodeLanguage::Node => {
                let mut command = std::process::Command::new("node");
                command.arg("-e").arg(code);
                command
            }
            CodeLanguage::Python => {
                let mut command = std::process::Command::new("python3");
                command.arg("-c").arg(code);
                command
            }
            _ => panic!("Unsupported language"),
        };

        command.output().expect("Failed to execute command")
    }
}

#[cfg(test)]
mod tests {
    use crate::executors::executor_trait::ExecutorTrait;
    use crate::executors::local_executor::LocalExecutor;
    use crate::task::CodeLanguage;

    #[tokio::test]
    async fn test_execute_shell() {
        let code = "echo 'Hello, World!'".to_string();
        let executor = LocalExecutor::new();
        let output = executor.run(code, CodeLanguage::Shell).await;

        assert_eq!(output.status.success(), true);
        assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, World!\n");
    }

    #[tokio::test]
    async fn test_execute_node() {
        let code = "console.log('Hello, World!')".to_string();
        let executor = LocalExecutor::new();
        let output = executor.run(code, CodeLanguage::Node).await;

        assert_eq!(output.status.success(), true);
        assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, World!\n");
    }

    #[tokio::test]
    async fn test_execute_python() {
        let code = "print('Hello, World!')".to_string();
        let executor = LocalExecutor::new();
        let output = executor.run(code, CodeLanguage::Python).await;

        assert_eq!(output.status.success(), true);
        assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, World!\n");
    }
}
