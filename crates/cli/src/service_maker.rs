use colored::*;
use include_dir::Dir;
use regex::Regex;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub struct ServiceMaker {
    skeleton_dir: Dir<'static>,
}

impl ServiceMaker {
    pub fn new(skeleton_dir: Dir<'static>) -> Self {
        ServiceMaker { skeleton_dir }
    }
    
    pub fn create_new_service(&self) -> Result<(), String> {
        // Get service name from user
        print!("Enter the new service name (snake_case): ");
        io::stdout().flush().unwrap();
        
        let mut service_name = String::new();
        io::stdin().read_line(&mut service_name)
            .map_err(|e| format!("Failed to read input: {}", e))?;
        
        let service_name = service_name.trim();
        
        // Validate service name
        let name_regex = Regex::new(r"^[a-z][a-z0-9_]*$").unwrap();
        if !name_regex.is_match(service_name) {
            return Err("❌ Invalid name. Use snake_case (e.g., my_service).".to_string());
        }
        
        // Check if service already exists
        let cargo_toml_path = PathBuf::from("./Cargo.toml");
        if cargo_toml_path.exists() {
            let content = fs::read_to_string(&cargo_toml_path)
                .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;
            
            if content.contains(&format!("\"crates/services/{}\"", service_name)) {
                return Err(format!("❌ Service '{}' already exists in Cargo.toml.", service_name));
            }
        }
        
        let upper_service_name = service_name.to_uppercase();
        
        println!("Creating service '{}'...", service_name);
        
        // Update Cargo.toml
        println!("Updating Cargo.toml members...");
        self.update_cargo_toml(service_name)?;
        
        // Update .env with new port
        println!("Adding port to .env...");
        let port = self.add_service_port(&upper_service_name)?;
        
        // Copy command scripts
        println!("Creating command scripts...");
        self.copy_skeleton_files(
            "commands/services/<new_service_name>",
            &format!("commands/services/{}", service_name),
            service_name,
            &upper_service_name,
            port
        )?;
        
        // Update services_manager
        println!("Updating services_manager...");
        self.update_services_manager(service_name, &upper_service_name)?;
        
        // Copy crate skeleton
        println!("Creating service crate...");
        self.copy_skeleton_files(
            "crates/services/<new_service_name>",
            &format!("crates/services/{}", service_name),
            service_name,
            &upper_service_name,
            port
        )?;
        
        // Copy docker skeleton
        println!("Creating Docker files...");
        self.copy_skeleton_files(
            "docker/<new_service_name>",
            &format!("docker/{}", service_name),
            service_name,
            &upper_service_name,
            port
        )?;
        
        println!("\n{} Service '{}' created successfully!", "✅".green(), service_name);
        
        Ok(())
    }
    
    fn update_cargo_toml(&self, service_name: &str) -> Result<(), String> {
        let cargo_path = PathBuf::from("./Cargo.toml");
        let content = fs::read_to_string(&cargo_path)
            .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;
        
        let new_entry = format!("    \"crates/services/{}\",", service_name);
        
        // Find members block and insert sorted
        let lines: Vec<&str> = content.lines().collect();
        let mut new_lines = Vec::new();
        let mut in_members = false;
        let mut members_entries = Vec::new();
        
        for (_i, line) in lines.iter().enumerate() {
            if line.contains("members") && line.contains("=") {
                in_members = true;
                new_lines.push(line.to_string());
            } else if in_members && line.trim() == "]" {
                // Add new entry and sort all members
                members_entries.push(new_entry.clone());
                members_entries.sort();
                
                // Add all sorted members
                for entry in members_entries.into_iter() {
                    new_lines.push(entry);
                }
                
                new_lines.push(line.to_string());
                in_members = false;
                members_entries = Vec::new(); // Reset for safety
            } else if in_members && line.trim().starts_with('"') {
                members_entries.push(line.to_string());
            } else {
                new_lines.push(line.to_string());
            }
        }
        
        fs::write(&cargo_path, new_lines.join("\n"))
            .map_err(|e| format!("Failed to write Cargo.toml: {}", e))?;
        
        Ok(())
    }
    
    fn add_service_port(&self, upper_service_name: &str) -> Result<u16, String> {
        let env_path = PathBuf::from("./.env");
        let content = fs::read_to_string(&env_path)
            .map_err(|e| format!("Failed to read .env: {}", e))?;
        
        // Find highest port
        let port_regex = Regex::new(r"SERVICE_[A-Z_]+_PORT=(\d+)").unwrap();
        let mut highest_port = 34699;
        
        for cap in port_regex.captures_iter(&content) {
            if let Ok(port) = cap[1].parse::<u16>() {
                if port > highest_port {
                    highest_port = port;
                }
            }
        }
        
        let new_port = highest_port + 1;
        
        // Append new port
        let mut content = content;
        if !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(&format!("SERVICE_{}_PORT={}\n", upper_service_name, new_port));
        
        fs::write(&env_path, content)
            .map_err(|e| format!("Failed to write .env: {}", e))?;
        
        Ok(new_port)
    }
    
    fn update_services_manager(&self, service_name: &str, upper_service_name: &str) -> Result<(), String> {
        let lib_path = PathBuf::from("crates/common/services_manager/src/lib.rs");
        if !lib_path.exists() {
            return Err("services_manager lib.rs not found".to_string());
        }
        
        let content = fs::read_to_string(&lib_path)
            .map_err(|e| format!("Failed to read lib.rs: {}", e))?;
        
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        // Find enum ServicesName and add new entry
        let mut enum_index = None;
        let mut enum_end_index = None;
        for (i, line) in lines.iter().enumerate() {
            if line.contains("pub enum ServicesName") {
                enum_index = Some(i);
            } else if enum_index.is_some() && line.trim() == "}" {
                enum_end_index = Some(i);
                break;
            }
        }
        
        if let (Some(start), Some(end)) = (enum_index, enum_end_index) {
            // Collect existing entries
            let mut entries = Vec::new();
            for i in (start + 1)..end {
                let line = lines[i].trim();
                if !line.is_empty() && line != "{" {
                    entries.push(lines[i].clone());
                }
            }
            entries.push(format!("    {},", upper_service_name));
            entries.sort();
            
            // Replace entries
            lines.splice((start + 2)..end, entries);
        }
        
        // Find match self block and add new entry
        let mut match_index = None;
        let mut match_end_index = None;
        for (i, line) in lines.iter().enumerate() {
            if line.contains("match self {") {
                match_index = Some(i);
            } else if match_index.is_some() && line.trim() == "}" {
                match_end_index = Some(i);
                break;
            }
        }
        
        if let (Some(start), Some(end)) = (match_index, match_end_index) {
            // Collect existing entries
            let mut entries = Vec::new();
            for i in (start + 1)..end {
                let line = lines[i].trim();
                if !line.is_empty() {
                    entries.push(lines[i].clone());
                }
            }
            entries.push(format!("            ServicesName::{} => \"{}\",", upper_service_name, service_name));
            entries.sort();
            
            // Replace entries
            lines.splice((start + 1)..end, entries);
        }
        
        fs::write(&lib_path, lines.join("\n"))
            .map_err(|e| format!("Failed to write lib.rs: {}", e))?;
        
        Ok(())
    }
    
    fn copy_skeleton_files(
        &self,
        skeleton_path: &str,
        target_path: &str,
        service_name: &str,
        upper_service_name: &str,
        port: u16
    ) -> Result<(), String> {
        // Extract files from embedded directory
        self.extract_and_process_dir(
            skeleton_path,
            target_path,
            service_name,
            upper_service_name,
            port
        )
    }
    
    fn extract_and_process_dir(
        &self,
        skeleton_path: &str,
        target_path: &str,
        service_name: &str,
        upper_service_name: &str,
        port: u16
    ) -> Result<(), String> {
        // Navigate to the skeleton directory
        let mut current_dir = &self.skeleton_dir;
        for part in skeleton_path.split('/') {
            if let Some(dir) = current_dir.get_dir(part) {
                current_dir = dir;
            } else {
                return Err(format!("Skeleton path not found: {}", skeleton_path));
            }
        }
        
        // Create target directory
        fs::create_dir_all(target_path)
            .map_err(|e| format!("Failed to create directory {}: {}", target_path, e))?;
        
        // Process all files and directories
        self.process_dir_contents(current_dir, Path::new(target_path), service_name, upper_service_name, port)?;
        
        Ok(())
    }
    
    fn process_dir_contents(
        &self,
        dir: &Dir<'static>,
        target_path: &Path,
        service_name: &str,
        upper_service_name: &str,
        port: u16
    ) -> Result<(), String> {
        // Process files
        for file in dir.files() {
            let file_name = file.path().file_name()
                .ok_or("Invalid file name")?
                .to_str()
                .ok_or("Invalid UTF-8 in file name")?;
            
            let target_file = target_path.join(file_name);
            
            // Get file content and replace placeholders
            let content = file.contents_utf8()
                .ok_or_else(|| format!("Failed to read file as UTF-8: {:?}", file.path()))?;
            
            let processed_content = content
                .replace("<new_service_name>", service_name)
                .replace("<NEW_SERVICE_NAME>", upper_service_name)
                .replace("<new_service_port>", &port.to_string());
            
            fs::write(&target_file, processed_content)
                .map_err(|e| format!("Failed to write file {:?}: {}", target_file, e))?;
            
            // Make .sh files executable
            if file_name.ends_with(".sh") {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = fs::metadata(&target_file).unwrap().permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(&target_file, perms)
                        .map_err(|e| format!("Failed to set permissions: {}", e))?;
                }
            }
        }
        
        // Process subdirectories
        for subdir in dir.dirs() {
            let dir_name = subdir.path().file_name()
                .ok_or("Invalid directory name")?
                .to_str()
                .ok_or("Invalid UTF-8 in directory name")?;
            
            let target_subdir = target_path.join(dir_name);
            fs::create_dir_all(&target_subdir)
                .map_err(|e| format!("Failed to create directory {:?}: {}", target_subdir, e))?;
            
            self.process_dir_contents(subdir, &target_subdir, service_name, upper_service_name, port)?;
        }
        
        Ok(())
    }
}