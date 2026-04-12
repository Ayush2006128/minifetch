use sysinfo::System;

fn main() {
    System::new_all().refresh_all();

    println!("Uptime:  {}", System::uptime());
    println!("OS:  {:?}", System::name());
    println!("Kernel:  {:?}", System::kernel_version());
    println!("OS Version:  {:?}", System::os_version());
}
