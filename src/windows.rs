#![allow(non_snake_case)]

use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::fs::{File, OpenOptions};
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
    println!("Proxy Manager for Windows");
    println!("Usage:");
    println!("  rustprox --set host:port    Set proxy for all services");
    println!("  rustprox --unset            Remove all proxy settings");
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
    if let Some(profile) = env::var_os("USERPROFILE") {
        return PathBuf::from(profile);
    }
    
    let drive = env::var("HOMEDRIVE").unwrap_or_else(|_| String::from("C:"));
    let path = env::var("HOMEPATH").unwrap_or_else(|_| String::from("\\Users\\Default"));
    
    PathBuf::from(format!("{}{}", drive, path))
}

fn setAllProxies(proxyUrl: &str) -> io::Result<()> {
    println!("Setting proxies for all services...");
    
    if let Err(e) = setWindowsSystemProxy(proxyUrl) {
        eprintln!("Failed to set Windows system proxy: {}", e);
    }
    
    if let Err(e) = setNpmProxy(proxyUrl) {
        eprintln!("Failed to set NPM proxy: {}", e);
    }
    
    if let Err(e) = setGitProxy(proxyUrl) {
        eprintln!("Failed to set Git proxy: {}", e);
    }
    
    if let Err(e) = setEnvVarsProxy(proxyUrl) {
        eprintln!("Failed to set environment variables: {}", e);
    }
    
    println!("Proxy settings applied successfully");
    Ok(())
}

fn unsetAllProxies() -> io::Result<()> {
    println!("Removing proxies from all services...");
    
    if let Err(e) = unsetWindowsSystemProxy() {
        eprintln!("Failed to unset Windows system proxy: {}", e);
    }
    
    if let Err(e) = unsetNpmProxy() {
        eprintln!("Failed to unset NPM proxy: {}", e);
    }
    
    if let Err(e) = unsetGitProxy() {
        eprintln!("Failed to unset Git proxy: {}", e);
    }
    
    if let Err(e) = unsetEnvVarsProxy() {
        eprintln!("Failed to unset environment variables: {}", e);
    }
    
    println!("Proxy settings removed successfully");
    Ok(())
}

fn checkProxyStatus() {
    println!("Checking proxy status for various services:");
    
    checkWindowsSystemProxy();
    checkNpmProxy();
    checkGitProxy();
    checkEnvVarsProxy();
}

fn setWindowsSystemProxy(proxyUrl: &str) -> io::Result<()> {
    println!("Setting Windows system proxy to: {}", proxyUrl);
    
    let parts: Vec<&str> = proxyUrl.split("://").collect();
    let actualUrl = if parts.len() > 1 { parts[1] } else { proxyUrl };
    
    let status = Command::new("reg")
        .args(&["add", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings", 
               "/v", "ProxyEnable", "/t", "REG_DWORD", "/d", "1", "/f"])
        .status()?;
    
    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to enable proxy"));
    }
    
    let status = Command::new("reg")
        .args(&["add", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings", 
               "/v", "ProxyServer", "/t", "REG_SZ", "/d", actualUrl, "/f"])
        .status()?;
    
    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to set proxy server"));
    }
    
    println!("Windows system proxy enabled successfully");
    Ok(())
}

fn unsetWindowsSystemProxy() -> io::Result<()> {
    println!("Disabling Windows system proxy");
    
    let status = Command::new("reg")
        .args(&["add", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings", 
               "/v", "ProxyEnable", "/t", "REG_DWORD", "/d", "0", "/f"])
        .status()?;
    
    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to disable proxy"));
    }
    
    println!("Windows system proxy disabled successfully");
    Ok(())
}

fn checkWindowsSystemProxy() {
    let output = Command::new("reg")
        .args(&["query", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings", 
               "/v", "ProxyEnable"])
        .output();
    
    let enabled = match output {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            output_str.contains("0x1")
        },
        Err(_) => false
    };
    
    let server = if enabled {
        let output = Command::new("reg")
            .args(&["query", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings", 
                  "/v", "ProxyServer"])
            .output();
        
        match output {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                if output_str.contains("REG_SZ") {
                    let parts: Vec<&str> = output_str.split("REG_SZ").collect();
                    if parts.len() > 1 {
                        parts[1].trim().to_string()
                    } else {
                        "Unknown".to_string()
                    }
                } else {
                    "Not set".to_string()
                }
            },
            Err(_) => "Error".to_string()
        }
    } else {
        "Disabled".to_string()
    };
    
    println!("Windows system proxy: {}", if enabled { format!("Enabled ({})", server) } else { "Disabled".to_string() });
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

fn setEnvVarsProxy(proxyUrl: &str) -> io::Result<()> {
    println!("Setting system environment variables for proxy");
    
    let setHttpProxy = Command::new("setx")
        .args(&["/M", "HTTP_PROXY", proxyUrl])
        .status()?;
    
    let setHttpsProxy = Command::new("setx")
        .args(&["/M", "HTTPS_PROXY", proxyUrl])
        .status()?;
    
    if setHttpProxy.success() && setHttpsProxy.success() {
        println!("Environment variables set successfully");
        println!("Note: Changes will take effect after restarting applications or system");
    } else {
        eprintln!("Failed to set environment variables. Run as Administrator and try again.");
    }
    
    Ok(())
}

fn unsetEnvVarsProxy() -> io::Result<()> {
    println!("Removing system environment variables for proxy");
    
    let deleteHttpProxy = Command::new("reg")
        .args(&["delete", "HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment", 
                "/v", "HTTP_PROXY", "/f"])
        .status()?;
    
    let deleteHttpsProxy = Command::new("reg")
        .args(&["delete", "HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment", 
                "/v", "HTTPS_PROXY", "/f"])
        .status()?;
    
    if deleteHttpProxy.success() && deleteHttpsProxy.success() {
        println!("Environment variables removed successfully");
        println!("Note: Changes will take effect after restarting applications or system");
    } else {
        eprintln!("Failed to remove environment variables. Run as Administrator and try again.");
    }
    
    Ok(())
}

fn checkEnvVarsProxy() {
    let http_proxy = env::var("HTTP_PROXY").unwrap_or_default();
    let https_proxy = env::var("HTTPS_PROXY").unwrap_or_default();
    
    println!("Environment variables:");
    println!("  HTTP_PROXY: {}", if http_proxy.is_empty() { "Not set" } else { "Set" });
    println!("  HTTPS_PROXY: {}", if https_proxy.is_empty() { "Not set" } else { "Set" });
}