use sysinfo::System;

fn main() {
    println!("\u{f0954} Uptime:  {}", System::uptime());
    println!(
        "\u{f0aab} OS:  {}",
        System::name().unwrap_or_else(|| "Unknown".to_string())
    );
    println!(
        "\u{f0ec0} Kernel:  {}",
        System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    );
    println!(
        "\u{ebe9} OS Version:  {}",
        System::os_version().unwrap_or_else(|| "Unknown".to_string())
    );
}
