use minifetch::{get_uptime, get_os_name, get_kernel_version, get_os_version};

fn main() {
    let ascii_art = [
        r#"    _~_        _~^~_    "#,
        r#"   (o o)     \) o o (/  "#,
        r#"  /  V  \      \_-_/    "#,
        r#" /(  _  )\    / ' ' \   "#,
        r#"   ^^ ^^                "#,
    ];
    let uptime = format!("\u{f0954} Uptime:  {}", get_uptime());
    let os = format!(
        "\u{f0aab} OS:  {}",
        get_os_name()
    );
    let kernel = format!(
        "\u{f0ec0} Kernel:  {}",
        get_kernel_version()
    );
    let os_version = format!(
        "\u{ebe9} OS Version:  {}",
        get_os_version()
    );

    let info = [uptime, os, kernel, os_version];
    for (line, info) in ascii_art.iter().zip(info.iter()) {
        println!("{}  {}", line, info);
    }
}
