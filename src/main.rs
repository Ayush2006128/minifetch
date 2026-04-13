use sysinfo::System;

fn main() {
    println!("Uptime:  {}", System::uptime());
    println!(
        "OS:  {}",
        System::name().unwrap_or_else(|| "Unknown".to_string())
    );
    println!(
        "Kernel:  {}",
        System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    );
    println!(
        "OS Version:  {}",
        System::os_version().unwrap_or_else(|| "Unknown".to_string())
    );
}
