/* services */
use crate::services::system_service::SystemService;

/* models */
use crate::models::ResponseModel;

#[allow(non_snake_case)]
pub struct SystemController {
  systemService: SystemService,
}

#[allow(non_snake_case)]
impl SystemController {
  pub fn new() -> Self {
    Self {
      systemService: SystemService,
    }
  }

  pub fn stopService(&self, service: &str) -> Result<ResponseModel, ResponseModel> {
    self.systemService.stopService(service)
  }

  pub fn stopSelectedServices(
    &self,
    services: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    self.systemService.stopSelectedServices(services)
  }

  pub fn openFile(
    &self,
    path: &str,
    command: Option<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    self.systemService.openFile(path, command)
  }

  pub fn getAllServices(&self) -> Result<ResponseModel, ResponseModel> {
    self.systemService.getAllServices()
  }

  pub fn enableService(&self, service: &str) -> Result<ResponseModel, ResponseModel> {
    self.systemService.enableService(service)
  }

  pub fn startService(&self, service: &str) -> Result<ResponseModel, ResponseModel> {
    self.systemService.startService(service)
  }

  pub fn enableSelectedServices(
    &self,
    services: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    self.systemService.enableSelectedServices(services)
  }
}
