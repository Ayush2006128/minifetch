use sysinfo::System;

fn main() {
    let ascii_art = [
        r#"    _~_        _~^~_    "#,
        r#"   (o o)     \) o o (/  "#,
        r#"  /  V  \      \_-_/    "#,
        r#" /(  _  )\    / ' ' \   "#,
        r#"   ^^ ^^                "#,
    ];
    let uptime = format!("\u{f0954} Uptime:  {}", System::uptime());
    let os = format!(
        "\u{f0aab} OS:  {}",
        System::name().unwrap_or_else(|| "Unknown".to_string())
    );
    let kernel = format!(
        "\u{f0ec0} Kernel:  {}",
        System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    );
    let os_version = format!(
        "\u{ebe9} OS Version:  {}",
        System::os_version().unwrap_or_else(|| "Unknown".to_string())
    );

    let info = [uptime, os, kernel, os_version];
    for (line, info) in ascii_art.iter().zip(info.iter()) {
        println!("{}  {}", line, info);
    }
}
