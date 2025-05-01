#![allow(non_snake_case)]

use std::env;
use std::path::PathBuf;
use std::process::Command;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        printUsage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "--set" => {
            if args.len() < 3 {
                println!("Error: Missing proxy address");
                printUsage();
                return;
            }

            let proxy = &args[2];
            if !isValidProxy(proxy) {
                println!("Error: Invalid proxy format. Expected format: host:port");
                return;
            }

            let parts: Vec<&str> = proxy.split(':').collect();
            let host = parts[0];
            let port = parts[1];

            let http_proxy = format!("http://{}:{}", host, port);

            println!("Setting proxy to: {}", http_proxy);
            setAllProxies(&http_proxy).unwrap_or_else(|e| {
                eprintln!("Error setting some proxies: {}", e);
            });
        }
        "--unset" => {
            println!("Removing all proxy settings");

            unsetAllProxies().unwrap_or_else(|e| {
                eprintln!("Error unsetting some proxies: {}", e);
            });
        }
        "--status" => {
            println!("Checking current proxy settings");
            checkProxyStatus();
        }
        "--help" => {
            printUsage();
        }
        _ => {
            println!("Unknown command: {}", command);
            printUsage();
        }
    }
}

fn getUserHome() -> PathBuf {
    if let Ok(sudo_user) = env::var("SUDO_USER") {
        let output = Command::new("getent")
            .args(&["passwd", &sudo_user])
            .output()
            .expect("Failed to execute getent");

        let output_str = String::from_utf8_lossy(&output.stdout);
        let home_dir = output_str.split(':').nth(5).unwrap_or("").trim();

        if !home_dir.is_empty() {
            return PathBuf::from(home_dir);
        }
    }

    // Fallback to the standard home directory detection
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/home"))
}

pub fn unsetNpmProxy() -> std::io::Result<()> {
    let npmFile = getUserHome().join(".npmrc");

    if npmFile.exists() {
        let content = std::fs::read_to_string(&npmFile)?;
        let new_content = content
            .lines()
            .filter(|line| !line.contains("proxy=") && !line.contains("https-proxy="))
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(&npmFile, new_content)?;
        println!("Proxy settings removed successfully");
    } else {
        println!("NPM config file does not exist, nothing to remove");
    }
    Ok(())
}

pub fn unsetGitProxy() -> std::io::Result<()> {
    let http_status = Command::new("git")
        .args(&["config", "--global", "--unset", "http.proxy"])
        .status()?;

    let https_status = Command::new("git")
        .args(&["config", "--global", "--unset", "https.proxy"])
        .status()?;

    if http_status.success() && https_status.success() {
    } else {
        println!("Failed to remove Git proxy settings or settings didn't exist");
    }

    Ok(())
}

pub fn unsetBashProxy() -> std::io::Result<()> {
    let bashrcFile = getUserHome().join(".bashrc");

    if bashrcFile.exists() {
        let content = std::fs::read_to_string(&bashrcFile)?;

        // Filter out any lines containing proxy settings
        let new_content = content
            .lines()
            .filter(|line| {
                !line.contains("export HTTP_PROXY=")
                    && !line.contains("export http_proxy=")
                    && !line.contains("export HTTPS_PROXY=")
                    && !line.contains("export https_proxy=")
                    && !line.contains("# Proxy settings")
            })
            .collect::<Vec<_>>()
            .join("\n");

        std::fs::write(&bashrcFile, new_content)?;
    } else {
        println!(".bashrc file does not exist, nothing to remove");
    }

    Ok(())
}

pub fn unsetZshProxy() -> std::io::Result<()> {
    let zshrcFile = getUserHome().join(".zshrc");

    if zshrcFile.exists() {
        println!("Removing Zsh proxy settings from: {:?}", zshrcFile);
        let content = std::fs::read_to_string(&zshrcFile)?;

        let new_content = content
            .lines()
            .filter(|line| {
                !line.contains("export HTTP_PROXY=")
                    && !line.contains("export http_proxy=")
                    && !line.contains("export HTTPS_PROXY=")
                    && !line.contains("export https_proxy=")
                    && !line.contains("# Proxy settings")
            })
            .collect::<Vec<_>>()
            .join("\n");

        std::fs::write(&zshrcFile, new_content)?;
        println!("Zsh proxy settings removed successfully");
        println!("Run 'source ~/.zshrc' to apply the changes to your current session");
    } else {
        println!(".zshrc file does not exist, nothing to remove");
    }

    Ok(())
}

pub fn unsetAptProxy() -> std::io::Result<()> {
    let apt_conf = PathBuf::from("/etc/apt/apt.conf");

    if apt_conf.exists() {
        let content = std::fs::read_to_string(&apt_conf)?;

        let new_content = content
            .lines()
            .filter(|line| {
                !line.contains("Acquire::http::proxy") && !line.contains("Acquire::https::proxy")
            })
            .collect::<Vec<_>>()
            .join("\n");

        std::fs::write(&apt_conf, new_content).map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("Permission denied: this operation requires root privileges");
                eprintln!("Try running the program with sudo");
            }
            e
        })?;
    } else {
        println!("APT configuration file does not exist, nothing to remove");
    }

    Ok(())
}

fn printUsage() {
    println!("Proxy Manager for Linux");
    println!("Usage:");
    println!("  sudo rustprox --set host:port    Set proxy for all services");
    println!("  sudo rustprox --unset            Remove all proxy settings");
    println!("  rustprox --status           Check current proxy status");
    println!("  rustprox --help             Show this help message");
}

fn isValidProxy(proxy: &str) -> bool {
    let parts: Vec<&str> = proxy.split(':').collect();
    if parts.len() != 2 {
        return false;
    }
    if parts[1].parse::<u16>().is_err() {
        return false;
    }

    true
}

fn setAllProxies(proxy_url: &str) -> std::io::Result<()> {
    println!("Setting proxies for all services...");

    if let Err(e) = setNpmProxy(proxy_url) {
        eprintln!("Failed to set NPM proxy: {}", e);
    }

    if let Err(e) = setGitProxy(proxy_url) {
        eprintln!("Failed to set Git proxy: {}", e);
    }

    if let Err(e) = setBashProxy(proxy_url) {
        eprintln!("Failed to set Bash proxy: {}", e);
    }

    if let Err(e) = setZshProxy(proxy_url) {
        eprintln!("Failed to set Zsh proxy: {}", e);
    }

    match setAptProxy(proxy_url) {
        Ok(_) => println!("APT proxy set successfully"),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("Failed to set APT proxy: permission denied");
                eprintln!(
                    "To set APT proxy, run with sudo: sudo rustprox --set {}",
                    proxy_url
                );
            } else {
                eprintln!("Failed to set APT proxy: {}", e);
            }
        }
    }

    println!("Proxy settings applied successfully");
    Ok(())
}

fn unsetAllProxies() -> std::io::Result<()> {
    println!("Removing proxies from all services...");

    if let Err(e) = unsetNpmProxy() {
        eprintln!("Failed to unset NPM proxy: {}", e);
    }

    if let Err(e) = unsetGitProxy() {
        eprintln!("Failed to unset Git proxy: {}", e);
    }

    if let Err(e) = unsetBashProxy() {
        eprintln!("Failed to unset Bash proxy: {}", e);
    }

    if let Err(e) = unsetZshProxy() {
        eprintln!("Failed to unset Zsh proxy: {}", e);
    }

    match unsetAptProxy() {
        Ok(_) => println!("APT proxy removed successfully"),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("Failed to unset APT proxy: permission denied");
                eprintln!("To unset APT proxy, run with sudo: sudo rustprox --unset");
            } else {
                eprintln!("Failed to unset APT proxy: {}", e);
            }
        }
    }

    println!("Proxy settings removed successfully");
    Ok(())
}

fn checkProxyStatus() {
    println!("Checking proxy status for various services:");

    let npm_file = getUserHome().join(".npmrc");
    println!(
        "NPM proxy: {}",
        if npm_file.exists()
            && std::fs::read_to_string(&npm_file)
                .map(|c| c.contains("proxy="))
                .unwrap_or(false)
        {
            "Set"
        } else {
            "Not set"
        }
    );

    let git_output = Command::new("git")
        .args(&["config", "--global", "--get", "http.proxy"])
        .output();

    println!(
        "Git proxy: {}",
        if git_output.is_ok() && !git_output.unwrap().stdout.is_empty() {
            "Set"
        } else {
            "Not set"
        }
    );

    let bash_file = getUserHome().join(".bashrc");
    println!(
        "Bash proxy: {}",
        if bash_file.exists()
            && std::fs::read_to_string(&bash_file)
                .map(|c| c.contains("export HTTP_PROXY="))
                .unwrap_or(false)
        {
            "Set"
        } else {
            "Not set"
        }
    );

    let zsh_file = getUserHome().join(".zshrc");
    println!(
        "Zsh proxy: {}",
        if zsh_file.exists()
            && std::fs::read_to_string(&zsh_file)
                .map(|c| c.contains("export HTTP_PROXY="))
                .unwrap_or(false)
        {
            "Set"
        } else {
            "Not set"
        }
    );

    let apt_conf = PathBuf::from("/etc/apt/apt.conf");
    println!(
        "APT proxy: {}",
        if apt_conf.exists()
            && std::fs::read_to_string(&apt_conf)
                .map(|c| c.contains("Acquire::http::proxy"))
                .unwrap_or(false)
        {
            "Set"
        } else {
            "Not set"
        }
    );
}

fn setNpmProxy(proxy_url: &str) -> std::io::Result<()> {
    let npmFile = getUserHome().join(".npmrc");
    let mut proxyExists = false;
    let mut httpsProxyExists = false;

    if npmFile.exists() {
        if let Ok(content) = std::fs::read_to_string(&npmFile) {
            proxyExists = content.contains(&format!("proxy={}", proxy_url));
            httpsProxyExists = content.contains(&format!("https-proxy={}", proxy_url));
        }
    }

    if !proxyExists || !httpsProxyExists {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&npmFile)?;

        use std::io::Write;

        if !proxyExists {
            writeln!(file, "proxy={}", proxy_url)?;
        }

        if !httpsProxyExists {
            writeln!(file, "https-proxy={}", proxy_url)?;
        }

        println!("NPM proxy settings updated successfully");
    } else {
        println!("NPM proxy settings already exist, no changes made");
    }

    Ok(())
}

fn setGitProxy(proxy_url: &str) -> std::io::Result<()> {
    let http_status = Command::new("git")
        .args(&["config", "--global", "http.proxy", proxy_url])
        .status()?;

    let https_status = Command::new("git")
        .args(&["config", "--global", "https.proxy", proxy_url])
        .status()?;

    if http_status.success() && https_status.success() {
        println!("Git proxy settings updated successfully");
    } else {
        eprintln!("Failed to update Git proxy settings");
    }

    Ok(())
}

fn setBashProxy(proxy_url: &str) -> std::io::Result<()> {
    let bashrcFile = getUserHome().join(".bashrc");

    let mut httpProxyExists = false;
    let mut httpsProxyExists = false;

    if bashrcFile.exists() {
        if let Ok(content) = std::fs::read_to_string(&bashrcFile) {
            httpProxyExists = content.contains(&format!("export HTTP_PROXY={}", proxy_url));
            httpsProxyExists = content.contains(&format!("export HTTPS_PROXY={}", proxy_url));
        }
    }

    if !httpProxyExists || !httpsProxyExists {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&bashrcFile)?;

        use std::io::Write;

        writeln!(file, "\n# Proxy settings")?;

        if !httpProxyExists {
            writeln!(file, "export HTTP_PROXY={}", proxy_url)?;
            writeln!(file, "export http_proxy={}", proxy_url)?;
        }

        if !httpsProxyExists {
            writeln!(file, "export HTTPS_PROXY={}", proxy_url)?;
            writeln!(file, "export https_proxy={}", proxy_url)?;
        }

        println!("Bash proxy settings updated successfully");
        println!("Run 'source ~/.bashrc' to apply the changes to your current session");
    } else {
        println!("Bash proxy settings already exist, no changes made");
    }

    Ok(())
}

fn setZshProxy(proxy_url: &str) -> std::io::Result<()> {
    let zshrcFile = getUserHome().join(".zshrc");

    let mut httpProxyExists = false;
    let mut httpsProxyExists = false;

    if zshrcFile.exists() {
        if let Ok(content) = std::fs::read_to_string(&zshrcFile) {
            httpProxyExists = content.contains(&format!("export HTTP_PROXY={}", proxy_url));
            httpsProxyExists = content.contains(&format!("export HTTPS_PROXY={}", proxy_url));
        }
    }

    if !httpProxyExists || !httpsProxyExists {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&zshrcFile)?;

        use std::io::Write;

        writeln!(file, "\n# Proxy settings")?;

        if !httpProxyExists {
            writeln!(file, "export HTTP_PROXY={}", proxy_url)?;
            writeln!(file, "export http_proxy={}", proxy_url)?;
        }

        if !httpsProxyExists {
            writeln!(file, "export HTTPS_PROXY={}", proxy_url)?;
            writeln!(file, "export https_proxy={}", proxy_url)?;
        }

        println!("Zsh proxy settings updated successfully");
        println!("Run 'source ~/.zshrc' to apply the changes to your current session");
    } else {
        println!("Zsh proxy settings already exist, no changes made");
    }

    Ok(())
}

fn setAptProxy(proxy_url: &str) -> std::io::Result<()> {
    let apt_conf = PathBuf::from("/etc/apt/apt.conf");

    let mut proxy_http_exists = false;
    let mut proxy_https_exists = false;

    if apt_conf.exists() {
        if let Ok(content) = std::fs::read_to_string(&apt_conf) {
            proxy_http_exists =
                content.contains(&format!("Acquire::http::proxy \"{}\";", proxy_url));
            proxy_https_exists =
                content.contains(&format!("Acquire::https::proxy \"{}\";", proxy_url));
        }
    }

    if !proxy_http_exists || !proxy_https_exists {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&apt_conf)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    eprintln!("Permission denied: this operation requires root privileges");
                    eprintln!("Try running the program with sudo");
                }
                e
            })?;

        use std::io::Write;

        if !proxy_http_exists {
            writeln!(file, "Acquire::http::proxy \"{}\";", proxy_url)?;
        }

        if !proxy_https_exists {
            writeln!(file, "Acquire::https::proxy \"{}\";", proxy_url)?;
        }

        println!("APT proxy settings updated successfully");
    } else {
        println!("APT proxy settings already exist, no changes made");
    }

    Ok(())
}
