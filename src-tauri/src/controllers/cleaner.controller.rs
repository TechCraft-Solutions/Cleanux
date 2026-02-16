/* services */
use crate::services::cleaner_service::CleanerService;

/* models */
use crate::models::ResponseModel;

#[allow(non_snake_case)]
pub struct CleanerController {
  cleanerService: CleanerService,
}

#[allow(non_snake_case)]
impl CleanerController {
  pub fn new() -> Self {
    Self {
      cleanerService: CleanerService,
    }
  }

  pub fn getCacheFiles(&self) -> Result<ResponseModel, ResponseModel> {
    self.cleanerService.getCacheFiles()
  }

  pub fn getTrashFiles(&self) -> Result<ResponseModel, ResponseModel> {
    self.cleanerService.getTrashFiles()
  }

  pub fn getSystemLogs(&self) -> Result<ResponseModel, ResponseModel> {
    self.cleanerService.getSystemLogs()
  }

  pub fn getLargeFiles(&self) -> Result<ResponseModel, ResponseModel> {
    self.cleanerService.getLargeFiles()
  }

  pub fn previewFile(&self, path: String) -> Result<ResponseModel, ResponseModel> {
    self.cleanerService.previewFile(path)
  }

  pub fn clearSelectedCacheFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    self.cleanerService.clearSelectedCacheFiles(paths)
  }

  pub fn clearSelectedTrashFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    self.cleanerService.clearSelectedTrashFiles(paths)
  }

  pub fn clearSelectedLogFiles(&self, paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
    self.cleanerService.clearSelectedLogFiles(paths)
  }

  pub fn clearSelectedLargeFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    self.cleanerService.clearSelectedLargeFiles(paths)
  }

  pub fn clearTrash(&self) -> Result<ResponseModel, ResponseModel> {
    self.cleanerService.clearTrash()
  }

  pub fn clearCache(&self) -> Result<ResponseModel, ResponseModel> {
    self.cleanerService.clearCache()
  }

  pub fn clearAllLogs(&self) -> Result<ResponseModel, ResponseModel> {
    self.cleanerService.clearAllLogs()
  }

  pub fn clearAllLargeFiles(&self) -> Result<ResponseModel, ResponseModel> {
    self.cleanerService.clearAllLargeFiles()
  }
}
