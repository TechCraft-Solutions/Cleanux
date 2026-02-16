/* sys lib */
use std::fs;
use std::path::Path;

/* models */
use crate::models::{
  CacheFileModel, DataValue, LargeFileModel, LogFileModel, ResponseModel, ResponseStatus,
  TrashFileModel,
};

/* helpers */
use chrono::{DateTime, Local};
use rayon::prelude::*;
use serde_json::json;
use walkdir::WalkDir;

pub struct CleanerService;

#[allow(non_snake_case)]
impl CleanerService {
  pub fn getCacheFiles(&self) -> Result<ResponseModel, ResponseModel> {
    let cacheDir = dirs::cache_dir().ok_or("Cache directory not found")?;

    let files: Vec<CacheFileModel> = WalkDir::new(cacheDir)
      .max_depth(4)
      .into_iter()
      .filter_map(|e| e.ok())
      .filter(|e| e.file_type().is_file())
      .take(1000)
      .collect::<Vec<_>>()
      .into_par_iter()
      .filter_map(|entry| {
        let path = entry.path();
        let metadata = fs::metadata(path).ok()?;
        let modified: DateTime<Local> = metadata
          .modified()
          .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
          .into();
        Some(CacheFileModel {
          path: path.to_string_lossy().to_string(),
          size: metadata.len(),
          modified: modified.format("%Y-%m-%d %H:%M:%S").to_string(),
        })
      })
      .collect();

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "Cache files retrieved successfully".to_string(),
      data: DataValue::Array(
        files
          .into_iter()
          .map(|f| serde_json::to_value(f).unwrap_or(json!({})))
          .collect(),
      ),
    })
  }

  pub fn getTrashFiles(&self) -> Result<ResponseModel, ResponseModel> {
    let home = dirs::home_dir().ok_or("Home directory not found")?;
    let trashDir = home.join(".local/share/Trash/files");

    let mut trashFiles = Vec::new();

    if let Ok(entries) = fs::read_dir(&trashDir) {
      for entry in entries.flatten() {
        let path = entry.path();
        if let Ok(metadata) = fs::metadata(&path) {
          let deletedDate: DateTime<Local> = metadata
            .modified()
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
            .into();
          trashFiles.push(TrashFileModel {
            name: path
              .file_name()
              .unwrap_or_default()
              .to_string_lossy()
              .to_string(),
            path: path.to_string_lossy().to_string(),
            size: metadata.len(),
            deletedDate: deletedDate.format("%Y-%m-%d %H:%M:%S").to_string(),
          });
        }
      }
    }

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "Trash files retrieved successfully".to_string(),
      data: DataValue::Array(
        trashFiles
          .into_iter()
          .map(|f| serde_json::to_value(f).unwrap_or(json!({})))
          .collect(),
      ),
    })
  }

  pub fn getSystemLogs(&self) -> Result<ResponseModel, ResponseModel> {
    let logDir = Path::new("/var/log");

    let files: Vec<LogFileModel> = WalkDir::new(logDir)
      .max_depth(3)
      .into_iter()
      .filter_map(|e| e.ok())
      .filter(|e| e.file_type().is_file())
      .take(500)
      .collect::<Vec<_>>()
      .into_par_iter()
      .filter_map(|entry| {
        let path = entry.path();
        let metadata = fs::metadata(path).ok()?;
        let modified: DateTime<Local> = metadata
          .modified()
          .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
          .into();
        Some(LogFileModel {
          path: path.to_string_lossy().to_string(),
          size: metadata.len(),
          modified: modified.format("%Y-%m-%d %H:%M:%S").to_string(),
        })
      })
      .collect();

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "System logs retrieved successfully".to_string(),
      data: DataValue::Array(
        files
          .into_iter()
          .map(|f| serde_json::to_value(f).unwrap_or(json!({})))
          .collect(),
      ),
    })
  }

  pub fn getLargeFiles(&self) -> Result<ResponseModel, ResponseModel> {
    let home = dirs::home_dir().ok_or("Home directory not found")?;
    let threshold = 100 * 1024 * 1024;

    let dirsToScan = vec![
      home.join("Downloads"),
      home.join("Documents"),
      home.join("Videos"),
      home.join("Pictures"),
      home.join("Desktop"),
    ];

    let mut files: Vec<LargeFileModel> = dirsToScan
      .into_par_iter()
      .map(|dir| {
        if !dir.exists() {
          return Vec::new();
        }
        WalkDir::new(dir)
          .max_depth(3)
          .into_iter()
          .filter_map(|e| e.ok())
          .filter(|e| e.file_type().is_file())
          .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            if metadata.len() > threshold {
              let modified: DateTime<Local> = metadata
                .modified()
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                .into();
              Some(LargeFileModel {
                name: entry.file_name().to_string_lossy().to_string(),
                path: entry.path().to_string_lossy().to_string(),
                size: metadata.len(),
                modified: modified.format("%Y-%m-%d %H:%M:%S").to_string(),
              })
            } else {
              None
            }
          })
          .take(50)
          .collect::<Vec<_>>()
      })
      .flatten()
      .collect();

    files.sort_by(|a, b| b.size.cmp(&a.size));
    if files.len() > 200 {
      files.truncate(200);
    }

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "Large files retrieved successfully".to_string(),
      data: DataValue::Array(
        files
          .into_iter()
          .map(|f| serde_json::to_value(f).unwrap_or(json!({})))
          .collect(),
      ),
    })
  }

  pub fn clearSelectedCacheFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    let mut cleared = 0;
    let mut errors = Vec::new();

    for path in paths {
      if let Err(e) = fs::remove_file(&path) {
        errors.push(format!("{}: {}", path, e));
      } else {
        cleared += 1;
      }
    }

    if errors.is_empty() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Successfully cleared {} cache files", cleared),
        data: DataValue::String("".to_string()),
      })
    } else {
      Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!(
          "Cleared {} files, failed on: {}",
          cleared,
          errors.join("; ")
        ),
        data: DataValue::String("".to_string()),
      })
    }
  }

  pub fn clearSelectedTrashFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    let mut cleared = 0;
    let mut errors = Vec::new();

    for path in paths {
      if let Err(e) = fs::remove_file(&path) {
        errors.push(format!("{}: {}", path, e));
      } else {
        cleared += 1;
      }
    }

    if errors.is_empty() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Successfully cleared {} trash files", cleared),
        data: DataValue::String("".to_string()),
      })
    } else {
      Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!(
          "Cleared {} files, failed on: {}",
          cleared,
          errors.join("; ")
        ),
        data: DataValue::String("".to_string()),
      })
    }
  }

  pub fn clearSelectedLogFiles(&self, paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
    if paths.is_empty() {
      return Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: "No log files selected".to_string(),
        data: DataValue::String("".to_string()),
      });
    }

    // Batch all paths into a single pkexec command
    let mut cmd = std::process::Command::new("pkexec");
    cmd.arg("rm").arg("-f");
    for path in &paths {
      cmd.arg(path);
    }

    let output = cmd.output().map_err(|e| ResponseModel {
      status: ResponseStatus::Error,
      message: format!("Failed to run pkexec: {}", e),
      data: DataValue::String("".to_string()),
    })?;

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Successfully cleared {} log files", paths.len()),
        data: DataValue::String("".to_string()),
      })
    } else {
      Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!(
          "Failed to clear log files: {}",
          String::from_utf8_lossy(&output.stderr).trim()
        ),
        data: DataValue::String("".to_string()),
      })
    }
  }

  pub fn clearSelectedLargeFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    let mut cleared = 0;
    let mut errors = Vec::new();

    for path in paths {
      if let Err(e) = fs::remove_file(&path) {
        errors.push(format!("{}: {}", path, e));
      } else {
        cleared += 1;
      }
    }

    if errors.is_empty() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Successfully cleared {} large files", cleared),
        data: DataValue::String("".to_string()),
      })
    } else {
      Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!(
          "Cleared {} files, failed on: {}",
          cleared,
          errors.join("; ")
        ),
        data: DataValue::String("".to_string()),
      })
    }
  }

  pub fn clearTrash(&self) -> Result<ResponseModel, ResponseModel> {
    let home = dirs::home_dir().ok_or("Home directory not found")?;
    let trashDir = home.join(".local/share/Trash/files");
    match fs::read_dir(&trashDir) {
      Ok(entries) => {
        for entry in entries.flatten() {
          let path = entry.path();
          if path.is_file() {
            if let Err(e) = fs::remove_file(&path) {
              return Err(ResponseModel {
                status: ResponseStatus::Error,
                message: format!("Failed to remove {}: {}", path.display(), e),
                data: DataValue::String("".to_string()),
              });
            }
          }
        }
        Ok(ResponseModel {
          status: ResponseStatus::Success,
          message: "Trash cleared successfully".to_string(),
          data: DataValue::String("".to_string()),
        })
      }
      Err(e) => Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Failed to read trash: {}", e),
        data: DataValue::String("".to_string()),
      }),
    }
  }

  pub fn clearCache(&self) -> Result<ResponseModel, ResponseModel> {
    let cacheDir = dirs::cache_dir().ok_or("Cache directory not found")?;
    if cacheDir.exists() {
      match fs::remove_dir_all(&cacheDir) {
        Ok(_) => {
          // Re-create the empty directory
          let _ = fs::create_dir_all(&cacheDir);
          Ok(ResponseModel {
            status: ResponseStatus::Success,
            message: "Cache directory cleared successfully".to_string(),
            data: DataValue::String("".to_string()),
          })
        }
        Err(e) => Err(ResponseModel {
          status: ResponseStatus::Error,
          message: format!("Failed to clear cache directory: {}", e),
          data: DataValue::String("".to_string()),
        }),
      }
    } else {
      Ok(ResponseModel {
        status: ResponseStatus::Info,
        message: "No cache to clear".to_string(),
        data: DataValue::String("".to_string()),
      })
    }
  }

  pub fn clearAllLogs(&self) -> Result<ResponseModel, ResponseModel> {
    let logDir = Path::new("/var/log");

    let files: Vec<LogFileModel> = WalkDir::new(logDir)
      .max_depth(3)
      .into_iter()
      .filter_map(|e| e.ok())
      .filter(|e| e.file_type().is_file())
      .take(500)
      .collect::<Vec<_>>()
      .into_par_iter()
      .filter_map(|entry| {
        let path = entry.path();
        let metadata = fs::metadata(path).ok()?;
        let modified: DateTime<Local> = metadata
          .modified()
          .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
          .into();
        Some(LogFileModel {
          path: path.to_string_lossy().to_string(),
          size: metadata.len(),
          modified: modified.format("%Y-%m-%d %H:%M:%S").to_string(),
        })
      })
      .collect();

    if files.is_empty() {
      return Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: "No log files found to clear".to_string(),
        data: DataValue::String("0".to_string()),
      });
    }

    // Batch all paths into a single pkexec command
    let mut cmd = std::process::Command::new("pkexec");
    cmd.arg("rm").arg("-f");
    for log in &files {
      cmd.arg(&log.path);
    }

    let output = cmd.output().map_err(|e| ResponseModel {
      status: ResponseStatus::Error,
      message: format!("Failed to run pkexec: {}", e),
      data: DataValue::String("0".to_string()),
    })?;

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Cleared {} log files", files.len()),
        data: DataValue::String(files.len().to_string()),
      })
    } else {
      Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!(
          "Failed to clear logs: {}",
          String::from_utf8_lossy(&output.stderr).trim()
        ),
        data: DataValue::String("0".to_string()),
      })
    }
  }

  pub fn clearAllLargeFiles(&self) -> Result<ResponseModel, ResponseModel> {
    let home = dirs::home_dir().ok_or("Home directory not found")?;
    let threshold = 100 * 1024 * 1024;

    let dirsToScan = vec![
      home.join("Downloads"),
      home.join("Documents"),
      home.join("Videos"),
      home.join("Pictures"),
      home.join("Desktop"),
    ];

    let files: Vec<LargeFileModel> = dirsToScan
      .into_par_iter()
      .flat_map(|dir| {
        if !dir.exists() {
          return Vec::new();
        }
        WalkDir::new(dir)
          .max_depth(3)
          .into_iter()
          .filter_map(|e| e.ok())
          .filter(|e| e.file_type().is_file())
          .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            if metadata.len() > threshold {
              let modified: DateTime<Local> = metadata
                .modified()
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                .into();
              Some(LargeFileModel {
                name: entry.file_name().to_string_lossy().to_string(),
                path: entry.path().to_string_lossy().to_string(),
                size: metadata.len(),
                modified: modified.format("%Y-%m-%d %H:%M:%S").to_string(),
              })
            } else {
              None
            }
          })
          .take(50)
          .collect::<Vec<_>>()
      })
      .collect();

    let mut clearedCount = 0;
    for file in files {
      if fs::remove_file(file.path).is_ok() {
        clearedCount += 1;
      }
    }

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: format!("Cleared {} large files", clearedCount),
      data: DataValue::String(clearedCount.to_string()),
    })
  }

  pub fn previewFile(&self, path: String) -> Result<ResponseModel, ResponseModel> {
    let filePath = Path::new(&path);

    if !filePath.exists() {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: "File not found".to_string(),
        data: DataValue::String("".to_string()),
      });
    }

    let extension = filePath
      .extension()
      .and_then(|e| e.to_str())
      .unwrap_or("")
      .to_lowercase();

    let imageExtensions = vec!["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg", "ico"];
    let textExtensions = vec![
      "txt",
      "md",
      "json",
      "xml",
      "html",
      "css",
      "js",
      "ts",
      "rs",
      "py",
      "java",
      "c",
      "cpp",
      "h",
      "hpp",
      "go",
      "rb",
      "php",
      "sh",
      "bash",
      "zsh",
      "yaml",
      "yml",
      "toml",
      "ini",
      "cfg",
      "log",
      "conf",
      "properties",
      "env",
      "gitignore",
      "dockerignore",
      "editorconfig",
    ];

    let fileType = if imageExtensions.contains(&extension.as_str()) {
      "image"
    } else if textExtensions.contains(&extension.as_str()) {
      let metadata = fs::metadata(&path).ok();
      if let Some(meta) = metadata {
        if meta.len() < 1024 * 1024 {
          if fs::read(&path)
            .map(|bytes| {
              let valid_utf8 = String::from_utf8(bytes.clone()).is_ok();
              if valid_utf8 {
                let is_binary = bytes.iter().take(8000).any(|&b| b == 0);
                !is_binary
              } else {
                false
              }
            })
            .unwrap_or(false)
          {
            "text"
          } else {
            "binary"
          }
        } else {
          "text"
        }
      } else {
        "text"
      }
    } else {
      let metadata = fs::metadata(&path).ok();
      if let Some(meta) = metadata {
        if meta.len() < 1024 * 1024 {
          if fs::read(&path)
            .map(|bytes| {
              bytes
                .iter()
                .take(8000)
                .all(|&b| b == 0 || (b >= 32 && b < 127) || b == 9 || b == 10 || b == 13)
            })
            .unwrap_or(false)
          {
            "text"
          } else {
            "binary"
          }
        } else {
          "binary"
        }
      } else {
        "unknown"
      }
    };

    let name = filePath
      .file_name()
      .and_then(|n| n.to_str())
      .unwrap_or("unknown")
      .to_string();

    let responseData = match fileType {
      "image" => {
        let bytes = fs::read(&path).map_err(|e| ResponseModel {
          status: ResponseStatus::Error,
          message: format!("Failed to read file: {}", e),
          data: DataValue::String("".to_string()),
        })?;
        let base64 = base64_encode(&bytes);
        let mimeType = match extension.as_str() {
          "png" => "image/png",
          "gif" => "image/gif",
          "bmp" => "image/bmp",
          "webp" => "image/webp",
          "svg" => "image/svg+xml",
          _ => "image/jpeg",
        };
        let dataUrl = format!("data:{};base64,{}", mimeType, base64);
        json!({
          "name": name,
          "path": path,
          "type": "image",
          "imageUrl": dataUrl
        })
      }
      "text" => {
        let bytes = fs::read(&path).map_err(|e| ResponseModel {
          status: ResponseStatus::Error,
          message: format!("Failed to read file: {}", e),
          data: DataValue::String("".to_string()),
        })?;

        let content = String::from_utf8_lossy(&bytes).into_owned();
        let truncatedContent = if content.len() > 50000 {
          format!(
            "{}...\n\n[Content truncated - file too large]",
            &content[..50000]
          )
        } else {
          content
        };
        json!({
          "name": name,
          "path": path,
          "type": "text",
          "content": truncatedContent
        })
      }
      _ => json!({
        "name": name,
        "path": path,
        "type": fileType
      }),
    };

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "File preview retrieved".to_string(),
      data: DataValue::Object(responseData),
    })
  }
}

fn base64_encode(data: &[u8]) -> String {
  const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
  let mut result = String::new();

  for chunk in data.chunks(3) {
    let b0 = chunk[0] as usize;
    let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
    let b2 = chunk.get(2).copied().unwrap_or(0) as usize;

    result.push(CHARSET[b0 >> 2] as char);
    result.push(CHARSET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);

    if chunk.len() > 1 {
      result.push(CHARSET[((b1 & 0x0f) << 2) | (b2 >> 6)] as char);
    } else {
      result.push('=');
    }

    if chunk.len() > 2 {
      result.push(CHARSET[b2 & 0x3f] as char);
    } else {
      result.push('=');
    }
  }

  result
}
