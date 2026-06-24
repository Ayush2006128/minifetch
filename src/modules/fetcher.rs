use std::fs;
use std::process::Command;

/// Get system uptime in seconds
pub fn get_uptime() -> u64 {
    if let Ok(content) = fs::read_to_string("/proc/uptime") {
        if let Some(uptime_str) = content.split_whitespace().next() {
            if let Ok(uptime_f) = uptime_str.parse::<f64>() {
                return uptime_f as u64;
            }
        }
    }
    0
}

/// Get OS name/distribution
pub fn get_os_name() -> String {
    // Try reading from /etc/os-release first
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                let value = line.strip_prefix("PRETTY_NAME=").unwrap_or("");
                return value.trim_matches('"').to_string();
            }
        }
        // Fallback to NAME if PRETTY_NAME not found
        for line in content.lines() {
            if line.starts_with("NAME=") {
                let value = line.strip_prefix("NAME=").unwrap_or("");
                return value.trim_matches('"').to_string();
            }
        }
    }

    // Try lsb_release command as fallback
    if let Ok(output) = Command::new("lsb_release").arg("-ds").output() {
        if output.status.success() {
            let name = String::from_utf8_lossy(&output.stdout);
            return name.trim().to_string();
        }
    }

    "Unknown".to_string()
}

/// Get kernel version
pub fn get_kernel_version() -> String {
    // Try /proc/version first
    if let Ok(content) = fs::read_to_string("/proc/version") {
        if let Some(version_part) = content.split_whitespace().nth(2) {
            return version_part.to_string();
        }
    }

    // Try uname -r command
    if let Ok(output) = Command::new("uname").arg("-r").output() {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            return version.trim().to_string();
        }
    }

    "Unknown".to_string()
}

/// Get OS version
pub fn get_os_version() -> String {
    // Try reading from /etc/os-release
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("VERSION_ID=") {
                let value = line.strip_prefix("VERSION_ID=").unwrap_or("");
                return value.trim_matches('"').to_string();
            }
        }
        // Fallback to VERSION if VERSION_ID not found
        for line in content.lines() {
            if line.starts_with("VERSION=") {
                let value = line.strip_prefix("VERSION=").unwrap_or("");
                return value.trim_matches('"').to_string();
            }
        }
    }

    "Unknown".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_uptime() {
        let uptime = get_uptime();
        assert!(uptime > 0, "Uptime should be greater than 0");
    }

    #[test]
    fn test_get_os_name() {
        let os_name = get_os_name();
        assert!(!os_name.is_empty(), "OS name should not be empty");
    }

    #[test]
    fn test_get_kernel_version() {
        let kernel_version = get_kernel_version();
        assert!(!kernel_version.is_empty(), "Kernel version should not be empty");
    }

    #[test]
    fn test_get_os_version() {
        let os_version = get_os_version();
        assert!(!os_version.is_empty(), "OS version should not be empty");
    }  
}
