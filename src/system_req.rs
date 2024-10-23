use std::process::Command;
use std::path::Path;

fn check_for_gpu() -> bool {
    #[cfg(target_os = "windows")]
    let output = Command::new("wmic")
        .arg("path")
        .arg("win32_VideoController")
        .arg("get")
        .output()
        .expect("Failed to execute wmic");

    #[cfg(target_os = "linux")]
    let output = Command::new("lspci")
        .output()
        .expect("Failed to execute lspci");

    #[cfg(target_os = "macos")]
    let output = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output()
        .expect("Failed to execute system_profiler");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("NVIDIA")
}

fn is_cuda_installed() -> bool {
    #[cfg(target_os = "windows")]
    let cuda_path = Path::new("C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v11.0"); // Adjust as needed

    #[cfg(target_os = "linux")]
    let cuda_path = Path::new("/usr/local/cuda");

    #[cfg(target_os = "macos")]
    let cuda_path = Path::new("/usr/local/cuda");

    cuda_path.exists()
}

fn install_cuda() {
    #[cfg(target_os = "windows")]
    {
        // Windows installation steps, typically requires manual installation
        println!("Please install CUDA from the NVIDIA website: https://developer.nvidia.com/cuda-downloads");
    }

    #[cfg(target_os = "linux")]
    {
        let install_command = "sudo apt-get install -y nvidia-cuda-toolkit";
        let output = Command::new("sh")
            .arg("-c")
            .arg(install_command)
            .output()
            .expect("Failed to install CUDA");

        if !output.status.success() {
            eprintln!("CUDA installation failed: {}", String::from_utf8_lossy(&output.stderr));
        } else {
            println!("CUDA installed successfully!");
        }
    }

    #[cfg(target_os = "macos")]
    {
        // macOS installation typically requires downloading from NVIDIA
        println!("Please install CUDA from the NVIDIA website: https://developer.nvidia.com/cuda-downloads");
    }
}

pub fn execute() {
    if check_for_gpu() {
        println!("Supported GPU found.");

        if !is_cuda_installed() {
            println!("CUDA not found. Installing...");
            install_cuda();
        } else {
            println!("CUDA is already installed.");
        }
    } else {
        println!("No supported GPU found.");
    }
}

