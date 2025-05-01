#![allow(non_snake_case)]

use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::{self, Write};

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
            
            let httpProxy = format!("http://{}:{}", host, port);
            
            println!("Setting proxy to: {}", httpProxy);
            
            setAllProxies(&httpProxy).unwrap_or_else(|e| {
                eprintln!("Error setting some proxies: {}", e);
            });
        },
        "--unset" => {
            println!("Removing all proxy settings");
            
            unsetAllProxies().unwrap_or_else(|e| {
                eprintln!("Error unsetting some proxies: {}", e);
            });
        },
        "--status" => {
            println!("Checking current proxy settings");
            checkProxyStatus();
        },
        "--help" => {
            printUsage();
        },
        _ => {
            println!("Unknown command: {}", command);
            printUsage();
        }
    }
}

fn printUsage() {
    println!("Proxy Manager for macOS");
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

fn getUserHome() -> PathBuf {
    if let Some(home) = env::var_os("HOME") {
        return PathBuf::from(home);
    }
    
    PathBuf::from("/Users/")
}

fn setAllProxies(proxyUrl: &str) -> io::Result<()> {
    println!("Setting proxies for all services...");
    
    if let Err(e) = setMacSystemProxy(proxyUrl) {
        eprintln!("Failed to set macOS system proxy: {}", e);
    }
    
    if let Err(e) = setNpmProxy(proxyUrl) {
        eprintln!("Failed to set NPM proxy: {}", e);
    }
    
    if let Err(e) = setGitProxy(proxyUrl) {
        eprintln!("Failed to set Git proxy: {}", e);
    }
    
    if let Err(e) = setBashProxy(proxyUrl) {
        eprintln!("Failed to set Bash proxy: {}", e);
    }
    
    if let Err(e) = setZshProxy(proxyUrl) {
        eprintln!("Failed to set Zsh proxy: {}", e);
    }
    
    println!("Proxy settings applied successfully");
    Ok(())
}

fn unsetAllProxies() -> io::Result<()> {
    println!("Removing proxies from all services...");
    
    if let Err(e) = unsetMacSystemProxy() {
        eprintln!("Failed to unset macOS system proxy: {}", e);
    }
    
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
    
    println!("Proxy settings removed successfully");
    Ok(())
}

fn checkProxyStatus() {
    println!("Checking proxy status for various services:");
    
    checkMacSystemProxy();
    checkNpmProxy();
    checkGitProxy();
    checkShellProxy();
}

fn setMacSystemProxy(proxyUrl: &str) -> io::Result<()> {
    println!("Setting macOS system proxy to: {}", proxyUrl);
    
    let parts: Vec<&str> = proxyUrl.split("://").collect();
    let protocol = if parts.len() > 1 { parts[0] } else { "http" };
    
    let hostPort = if parts.len() > 1 { parts[1] } else { proxyUrl };
    let hostPortParts: Vec<&str> = hostPort.split(':').collect();
    
    let host = hostPortParts[0];
    let port = if hostPortParts.len() > 1 {
        hostPortParts[1].parse::<u16>().unwrap_or(8080)
    } else {
        8080
    };
    
    let networkInterfaces = Command::new("networksetup")
        .args(&["-listallnetworkservices"])
        .output()?;
    
    let interfacesStr = String::from_utf8_lossy(&networkInterfaces.stdout);
    let interfaces: Vec<&str> = interfacesStr.lines()
        .skip(1)  // Skip header line
        .collect();
    
    for interface in interfaces {
        let webProxy = Command::new("networksetup")
            .args(&["-setwebproxy", interface, host, &port.to_string(), "on"])
            .status()?;
            
        let secureWebProxy = Command::new("networksetup")
            .args(&["-setsecurewebproxy", interface, host, &port.to_string(), "on"])
            .status()?;
        
        if webProxy.success() && secureWebProxy.success() {
            println!("Set proxy for network interface: {}", interface);
        } else {
            eprintln!("Failed to set proxy for network interface: {}", interface);
        }
    }
    
    println!("macOS system proxy settings updated");
    println!("Note: Some applications may need to be restarted to apply the settings");
    
    Ok(())
}

fn unsetMacSystemProxy() -> io::Result<()> {
    println!("Disabling macOS system proxy");
    
    let networkInterfaces = Command::new("networksetup")
        .args(&["-listallnetworkservices"])
        .output()?;
    
    let interfacesStr = String::from_utf8_lossy(&networkInterfaces.stdout);
    let interfaces: Vec<&str> = interfacesStr.lines()
        .skip(1)  // Skip header line
        .collect();
    
    for interface in interfaces {
        let webProxy = Command::new("networksetup")
            .args(&["-setwebproxystate", interface, "off"])
            .status()?;
            
        let secureWebProxy = Command::new("networksetup")
            .args(&["-setsecurewebproxystate", interface, "off"])
            .status()?;
        
        if webProxy.success() && secureWebProxy.success() {
            println!("Disabled proxy for network interface: {}", interface);
        } else {
            eprintln!("Failed to disable proxy for network interface: {}", interface);
        }
    }
    
    println!("macOS system proxy settings disabled");
    println!("Note: Some applications may need to be restarted to apply the settings");
    
    Ok(())
}

fn checkMacSystemProxy() {
    let networkInterfaces = Command::new("networksetup")
        .args(&["-listallnetworkservices"])
        .output();
    
    match networkInterfaces {
        Ok(output) => {
            let interfacesStr = String::from_utf8_lossy(&output.stdout);
            let interfaces: Vec<&str> = interfacesStr.lines()
                .skip(1)  // Skip header line
                .collect();
            
            let mut isProxyEnabled = false;
            
            for interface in interfaces {
                let webProxy = Command::new("networksetup")
                    .args(&["-getwebproxy", interface])
                    .output();
                
                if let Ok(proxy) = webProxy {
                    let proxyStr = String::from_utf8_lossy(&proxy.stdout);
                    if proxyStr.contains("Enabled: Yes") {
                        isProxyEnabled = true;
                        println!("System proxy ({}): Enabled", interface);
                    }
                }
            }
            
            if !isProxyEnabled {
                println!("System proxy: Not set");
            }
        },
        Err(_) => {
            println!("System proxy: Failed to check");
        }
    }
}

fn setNpmProxy(proxyUrl: &str) -> io::Result<()> {
    println!("Setting NPM proxy to: {}", proxyUrl);
    
    let npmConfig = getUserHome().join(".npmrc");
    
    let mut proxyExists = false;
    let mut httpsProxyExists = false;
    
    if npmConfig.exists() {
        if let Ok(content) = std::fs::read_to_string(&npmConfig) {
            proxyExists = content.contains(&format!("proxy={}", proxyUrl));
            httpsProxyExists = content.contains(&format!("https-proxy={}", proxyUrl));
        }
    }
    
    if !proxyExists || !httpsProxyExists {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&npmConfig)?;
        
        if !proxyExists {
            writeln!(file, "proxy={}", proxyUrl)?;
        }
        
        if !httpsProxyExists {
            writeln!(file, "https-proxy={}", proxyUrl)?;
        }
        
        println!("NPM proxy settings updated successfully");
    } else {
        println!("NPM proxy settings already exist, no changes made");
    }
    
    Ok(())
}

fn unsetNpmProxy() -> io::Result<()> {
    println!("Removing NPM proxy settings");
    
    let npmConfig = getUserHome().join(".npmrc");
    
    if npmConfig.exists() {
        let content = std::fs::read_to_string(&npmConfig)?;
        let newContent = content
            .lines()
            .filter(|line| !line.contains("proxy=") && !line.contains("https-proxy="))
            .collect::<Vec<_>>()
            .join("\n");
        
        std::fs::write(&npmConfig, newContent)?;
        println!("NPM proxy settings removed successfully");
    } else {
        println!("NPM config file does not exist, nothing to remove");
    }
    
    Ok(())
}

fn checkNpmProxy() {
    let npmConfig = getUserHome().join(".npmrc");
    
    println!("NPM proxy: {}", 
        if npmConfig.exists() && 
           std::fs::read_to_string(&npmConfig)
               .map(|c| c.contains("proxy="))
               .unwrap_or(false) {
            "Set"
        } else {
            "Not set"
        }
    );
}

fn setGitProxy(proxyUrl: &str) -> io::Result<()> {
    println!("Setting Git proxy to: {}", proxyUrl);
    
    let httpStatus = Command::new("git")
        .args(&["config", "--global", "http.proxy", proxyUrl])
        .status()?;
    
    let httpsStatus = Command::new("git")
        .args(&["config", "--global", "https.proxy", proxyUrl])
        .status()?;
    
    if httpStatus.success() && httpsStatus.success() {
        println!("Git proxy settings updated successfully");
    } else {
        eprintln!("Failed to update Git proxy settings");
    }
    
    Ok(())
}

fn unsetGitProxy() -> io::Result<()> {
    println!("Removing Git proxy settings");
    
    let httpStatus = Command::new("git")
        .args(&["config", "--global", "--unset", "http.proxy"])
        .status()?;
    
    let httpsStatus = Command::new("git")
        .args(&["config", "--global", "--unset", "https.proxy"])
        .status()?;
    
    if httpStatus.success() && httpsStatus.success() {
        println!("Git proxy settings removed successfully");
    } else {
        println!("Failed to remove Git proxy settings or settings didn't exist");
    }
    
    Ok(())
}

fn checkGitProxy() {
    let output = Command::new("git")
        .args(&["config", "--global", "--get", "http.proxy"])
        .output();
    
    println!("Git proxy: {}", 
        if let Ok(output) = output {
            if output.stdout.is_empty() {
                "Not set"
            } else {
                "Set"
            }
        } else {
            "Not available"
        }
    );
}

fn setBashProxy(proxyUrl: &str) -> io::Result<()> {
    let bashrcFile = getUserHome().join(".bash_profile");
    
    let mut httpProxyExists = false;
    let mut httpsProxyExists = false;
    
    if bashrcFile.exists() {
        if let Ok(content) = std::fs::read_to_string(&bashrcFile) {
            httpProxyExists = content.contains(&format!("export HTTP_PROXY={}", proxyUrl));
            httpsProxyExists = content.contains(&format!("export HTTPS_PROXY={}", proxyUrl));
        }
    }
    
    if !httpProxyExists || !httpsProxyExists {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&bashrcFile)?;
        
        writeln!(file, "\n# Proxy settings")?;
        
        if !httpProxyExists {
            writeln!(file, "export HTTP_PROXY={}", proxyUrl)?;
            writeln!(file, "export http_proxy={}", proxyUrl)?;
        }
        
        if !httpsProxyExists {
            writeln!(file, "export HTTPS_PROXY={}", proxyUrl)?;
            writeln!(file, "export https_proxy={}", proxyUrl)?;
        }
        
        println!("Bash proxy settings updated successfully");
        println!("Run 'source ~/.bash_profile' to apply the changes to your current session");
    } else {
        println!("Bash proxy settings already exist, no changes made");
    }
    
    Ok(())
}

fn unsetBashProxy() -> io::Result<()> {
    let bashrcFile = getUserHome().join(".bash_profile");
    
    if bashrcFile.exists() {
        let content = std::fs::read_to_string(&bashrcFile)?;
        
        let newContent = content
            .lines()
            .filter(|line| !line.contains("export HTTP_PROXY=") && 
                           !line.contains("export http_proxy=") &&
                           !line.contains("export HTTPS_PROXY=") && 
                           !line.contains("export https_proxy=") &&
                           !line.contains("# Proxy settings"))
            .collect::<Vec<_>>()
            .join("\n");
        
        std::fs::write(&bashrcFile, newContent)?;
        println!("Bash proxy settings removed successfully");
        println!("Run 'source ~/.bash_profile' to apply the changes to your current session");
    } else {
        println!("Bash profile file does not exist, nothing to remove");
    }
    
    Ok(())
}

fn setZshProxy(proxyUrl: &str) -> io::Result<()> {
    let zshrcFile = getUserHome().join(".zshrc");
    
    let mut httpProxyExists = false;
    let mut httpsProxyExists = false;
    
    if zshrcFile.exists() {
        if let Ok(content) = std::fs::read_to_string(&zshrcFile) {
            httpProxyExists = content.contains(&format!("export HTTP_PROXY={}", proxyUrl));
            httpsProxyExists = content.contains(&format!("export HTTPS_PROXY={}", proxyUrl));
        }
    }
    
    if !httpProxyExists || !httpsProxyExists {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&zshrcFile)?;
        
        writeln!(file, "\n# Proxy settings")?;
        
        if !httpProxyExists {
            writeln!(file, "export HTTP_PROXY={}", proxyUrl)?;
            writeln!(file, "export http_proxy={}", proxyUrl)?;
        }
        
        if !httpsProxyExists {
            writeln!(file, "export HTTPS_PROXY={}", proxyUrl)?;
            writeln!(file, "export https_proxy={}", proxyUrl)?;
        }
        
        println!("Zsh proxy settings updated successfully");
        println!("Run 'source ~/.zshrc' to apply the changes to your current session");
    } else {
        println!("Zsh proxy settings already exist, no changes made");
    }
    
    Ok(())
}

fn unsetZshProxy() -> io::Result<()> {
    let zshrcFile = getUserHome().join(".zshrc");
    
    if zshrcFile.exists() {
        let content = std::fs::read_to_string(&zshrcFile)?;
        
        let newContent = content
            .lines()
            .filter(|line| !line.contains("export HTTP_PROXY=") && 
                           !line.contains("export http_proxy=") &&
                           !line.contains("export HTTPS_PROXY=") && 
                           !line.contains("export https_proxy=") &&
                           !line.contains("# Proxy settings"))
            .collect::<Vec<_>>()
            .join("\n");
        
        std::fs::write(&zshrcFile, newContent)?;
        println!("Zsh proxy settings removed successfully");
        println!("Run 'source ~/.zshrc' to apply the changes to your current session");
    } else {
        println!("Zsh config file does not exist, nothing to remove");
    }
    
    Ok(())
}

fn checkShellProxy() {
    let bashProfile = getUserHome().join(".bash_profile");
    println!("Bash proxy: {}", 
        if bashProfile.exists() && 
           std::fs::read_to_string(&bashProfile)
               .map(|c| c.contains("export HTTP_PROXY="))
               .unwrap_or(false) {
            "Set"
        } else {
            "Not set"
        }
    );
    
    let zshrc = getUserHome().join(".zshrc");
    println!("Zsh proxy: {}", 
        if zshrc.exists() && 
           std::fs::read_to_string(&zshrc)
               .map(|c| c.contains("export HTTP_PROXY="))
               .unwrap_or(false) {
            "Set"
        } else {
            "Not set"
        }
    );
}