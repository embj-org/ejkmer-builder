use std::path::{Path, PathBuf};
use tokio::process::Command;

use ej_builder_sdk::{Action, BuilderEvent, BuilderSdk, prelude::*};

const PI_USERNAME: &str = "andrebian";
const PI_ADDRESS: &str = "192.168.1.118";

async fn kill_application_in_rpi(sdk: &BuilderSdk) -> Result<()> {
    let result = Command::new("ssh")
        .arg(format!("{PI_USERNAME}@{PI_ADDRESS}"))
        .arg(format!("killall {}", sdk.board_config_name()))
        .spawn()?
        .wait()
        .await?;
    assert!(result.success(), "Failed to kill process in RPI");
    Ok(())
}
fn workspace_folder(config_path: &Path) -> PathBuf {
    config_path
        .parent()
        .expect(&format!(
            "Failed to get folder containing `config.toml` - Config path is: {}",
            config_path.display()
        ))
        .to_path_buf()
}

fn source_folder(config_path: &Path) -> PathBuf {
    workspace_folder(config_path).join("kmer")
}

fn build_folder(config_path: &Path) -> PathBuf {
    source_folder(config_path).join("build-pi")
}
fn toolchain_file(config_path: &Path) -> PathBuf {
    source_folder(config_path).join("aarch64_toolchain.cmake")
}
fn application_path(config_path: &Path, application_name: &str) -> PathBuf {
    build_folder(config_path).join(application_name)
}

fn inputs_path(config_path: &Path) -> PathBuf {
    source_folder(config_path).join("inputs")
}

fn results_path(config_path: &Path, application_name: &str) -> PathBuf {
    workspace_folder(config_path).join(format!("results_{}", application_name))
}
async fn run_application(sdk: &BuilderSdk) -> Result<()> {
    let config_path = &sdk.config_path();
    let app_name = &sdk.board_config_name();

    let result = Command::new("scp")
        .arg("-r")
        .arg(application_path(config_path, app_name))
        .arg(inputs_path(config_path))
        .arg(&format!("{PI_USERNAME}@{PI_ADDRESS}:~"))
        .spawn()?
        .wait()
        .await?;

    assert!(result.success(), "SCP execution failed");

    let result = Command::new("ssh")
        .arg(&format!("{}@{}", PI_USERNAME, PI_ADDRESS))
        .arg(&format!("time ./{} inputs/input.txt 3", app_name))
        .spawn()?
        .wait_with_output()
        .await?;

    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);

    assert!(result.status.success(), "SSH execution failed");

    std::fs::write(
        results_path(config_path, app_name),
        format!("{}\n{}", stdout, stderr),
    )?;

    Ok(())
}
async fn build_application(sdk: &BuilderSdk) -> Result<()> {
    let config_path = &sdk.config_path();

    let status = Command::new("cmake")
        .arg("-B")
        .arg(build_folder(config_path))
        .arg("-S")
        .arg(source_folder(config_path))
        .arg(&format!(
            "-DCMAKE_TOOLCHAIN_FILE={}",
            toolchain_file(config_path).display()
        ))
        .spawn()?
        .wait()
        .await?;

    assert!(status.success(), "CMake execution failed");

    Command::new("cmake")
        .arg("--build")
        .arg(build_folder(config_path))
        .arg("-j")
        .arg(num_cpus::get().to_string())
        .spawn()?
        .wait()
        .await?;

    assert!(status.success(), "Build failed");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let sdk = BuilderSdk::init(|sdk, event| async move {
        match event {
            BuilderEvent::Exit => kill_application_in_rpi(&sdk).await,
        }
    })
    .await?;

    match sdk.action() {
        Action::Build => build_application(&sdk).await,
        Action::Run => run_application(&sdk).await,
    }
}
