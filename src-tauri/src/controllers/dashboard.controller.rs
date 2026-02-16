/* services */
use crate::services::dashboard_service::DashboardService;

/* models */
use crate::models::ResponseModel;

#[allow(non_snake_case)]
pub struct DashboardController {
  dashboardService: DashboardService,
}

#[allow(non_snake_case)]
impl DashboardController {
  pub fn new() -> Self {
    Self {
      dashboardService: DashboardService,
    }
  }

  pub fn getRunningServices(&self) -> Result<ResponseModel, ResponseModel> {
    self.dashboardService.getRunningServices()
  }

  pub fn getCacheSummary(&self) -> Result<ResponseModel, ResponseModel> {
    self.dashboardService.getCacheSummary()
  }

  pub fn getTrashSummary(&self) -> Result<ResponseModel, ResponseModel> {
    self.dashboardService.getTrashSummary()
  }

  pub fn getLogSummary(&self) -> Result<ResponseModel, ResponseModel> {
    self.dashboardService.getLogSummary()
  }

  pub fn getLargeFilesSummary(&self) -> Result<ResponseModel, ResponseModel> {
    self.dashboardService.getLargeFilesSummary()
  }
}
