pub async fn is_docker_installed() -> bool {
    std::process::Command::new("docker")
        .arg("--version")
        .output()
        .unwrap().status.success()
}

pub async fn is_docker_running() -> bool {
    std::process::Command::new("docker")
        .arg("ps")
        .output()
        .unwrap().status.success()
}

pub async fn is_running_in_github_actions() -> bool {
    std::env::var("GITHUB_ACTIONS").is_ok()
}
