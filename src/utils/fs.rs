//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: File system utility

use anyhow::Result;
use std::fs;

fn get_files_in_current_dir(dir: &str) -> Result<Vec<String>, std::io::Error> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path.file_name().unwrap().to_string_lossy().to_string());
        }
    }

    Ok(files)
}

pub fn get_yaml_files_in_dir(dir: &str) -> Result<Vec<String>> {
    let files = get_files_in_current_dir(dir)?;

    let yaml_files = files
        .iter()
        .filter(|file| file.ends_with(".yml") || file.ends_with(".yaml"))
        .map(|file| file.clone())
        .collect();

    Ok(yaml_files)
}

// 通用权限设置函数
#[cfg(unix)]
fn set_dir_permissions(path: &str, permissions: u32) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(permissions))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_dir_permissions(path: &str, permissions: u32) -> Result<()> {
    // Windows 或其他平台，跳过权限设置
    fs::Permissions::from_mode(permissions);
    Ok(())
}

/// 创建目录并设置权限
pub fn make_dir_with_permissions(dir: &str, permissions: u32) -> Result<()> {
    fs::create_dir_all(dir)?;
    set_dir_permissions(dir, permissions)?;
    Ok(())
}
