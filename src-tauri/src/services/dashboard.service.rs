/* sys lib */
use std::fs;
use std::path::Path;
use std::process::Command;

/* models */
use crate::models::{
  DataValue, ResponseModel, ResponseStatus, ScanSummaryModel, SystemServiceModel,
};

/* helpers */
use rayon::prelude::*;
use serde_json::json;
use walkdir::WalkDir;

pub struct DashboardService;

#[allow(non_snake_case)]
impl DashboardService {
  pub fn getRunningServices(&self) -> Result<ResponseModel, ResponseModel> {
    let output = Command::new("systemctl")
      .args([
        "list-units",
        "--type=service",
        "--state=running",
        "--no-pager",
        "--plain",
      ])
      .output()
      .map_err(|e| format!("Failed to run systemctl: {}", e))?;

    if !output.status.success() {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: String::from_utf8_lossy(&output.stderr).to_string(),
        data: DataValue::String("".to_string()),
      });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut services = Vec::new();

    for line in stdout.lines().skip(1) {
      let parts: Vec<&str> = line.split_whitespace().collect();
      if parts.len() >= 4 {
        let name = parts[0].to_string();
        let status = parts[3].to_string();
        let description = if parts.len() > 4 {
          parts[4..].join(" ")
        } else {
          "No description".to_string()
        };
        services.push(SystemServiceModel {
          name,
          description,
          status: status.clone(),
          isRunning: status == "running",
        });
      }
    }

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "Running services retrieved successfully".to_string(),
      data: DataValue::Array(
        services
          .into_iter()
          .map(|s| serde_json::to_value(s).unwrap_or(json!({})))
          .collect(),
      ),
    })
  }

  pub fn getCacheSummary(&self) -> Result<ResponseModel, ResponseModel> {
    let cacheDir = dirs::cache_dir().ok_or("Cache directory not found")?;

    let (totalSize, fileCount) = WalkDir::new(cacheDir)
      .max_depth(4)
      .into_iter()
      .filter_map(|e| e.ok())
      .filter(|e| e.file_type().is_file())
      .take(2000)
      .collect::<Vec<_>>()
      .into_par_iter()
      .filter_map(|entry| fs::metadata(entry.path()).ok())
      .fold(
        || (0u64, 0usize),
        |mut acc, meta| {
          acc.0 += meta.len();
          acc.1 += 1;
          acc
        },
      )
      .reduce(|| (0u64, 0usize), |a, b| (a.0 + b.0, a.1 + b.1));

    let summary = ScanSummaryModel {
      totalSize,
      fileCount,
    };

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "Cache summary retrieved successfully".to_string(),
      data: DataValue::Object(serde_json::to_value(summary).unwrap_or(json!({}))),
    })
  }

  pub fn getTrashSummary(&self) -> Result<ResponseModel, ResponseModel> {
    let home = dirs::home_dir().ok_or("Home directory not found")?;
    let trashDir = home.join(".local/share/Trash/files");
    let mut totalSize = 0;
    let mut fileCount = 0;

    if let Ok(entries) = fs::read_dir(&trashDir) {
      for entry in entries.flatten() {
        if let Ok(meta) = fs::metadata(entry.path()) {
          totalSize += meta.len();
          fileCount += 1;
        }
      }
    }

    let summary = ScanSummaryModel {
      totalSize,
      fileCount,
    };

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "Trash summary retrieved successfully".to_string(),
      data: DataValue::Object(serde_json::to_value(summary).unwrap_or(json!({}))),
    })
  }

  pub fn getLogSummary(&self) -> Result<ResponseModel, ResponseModel> {
    let logDir = Path::new("/var/log");

    let (totalSize, fileCount) = WalkDir::new(logDir)
      .max_depth(2)
      .into_iter()
      .filter_map(|e| e.ok())
      .filter(|e| e.file_type().is_file())
      .take(500)
      .collect::<Vec<_>>()
      .into_par_iter()
      .filter_map(|entry| fs::metadata(entry.path()).ok())
      .fold(
        || (0u64, 0usize),
        |mut acc, meta| {
          acc.0 += meta.len();
          acc.1 += 1;
          acc
        },
      )
      .reduce(|| (0u64, 0usize), |a, b| (a.0 + b.0, a.1 + b.1));

    let summary = ScanSummaryModel {
      totalSize,
      fileCount,
    };

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "Log summary retrieved successfully".to_string(),
      data: DataValue::Object(serde_json::to_value(summary).unwrap_or(json!({}))),
    })
  }

  pub fn getLargeFilesSummary(&self) -> Result<ResponseModel, ResponseModel> {
    let home = dirs::home_dir().ok_or("Home directory not found")?;
    let threshold = 100 * 1024 * 1024;

    let dirsToScan = vec![
      home.join("Downloads"),
      home.join("Documents"),
      home.join("Videos"),
      home.join("Pictures"),
      home.join("Desktop"),
    ];

    let (totalSize, fileCount) = dirsToScan
      .into_par_iter()
      .map(|dir| {
        if !dir.exists() {
          return (0u64, 0usize);
        }
        WalkDir::new(dir)
          .max_depth(3)
          .into_iter()
          .filter_map(|e| e.ok())
          .filter(|e| e.file_type().is_file())
          .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            if metadata.len() > threshold {
              Some(metadata.len())
            } else {
              None
            }
          })
          .fold((0u64, 0usize), |acc, size| (acc.0 + size, acc.1 + 1))
      })
      .reduce(|| (0u64, 0usize), |a, b| (a.0 + b.0, a.1 + b.1));

    let summary = ScanSummaryModel {
      totalSize,
      fileCount,
    };

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "Large files summary retrieved successfully".to_string(),
      data: DataValue::Object(serde_json::to_value(summary).unwrap_or(json!({}))),
    })
  }
}
