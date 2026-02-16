/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { MainService } from './main.service';

/* models */
import {
  SystemServiceItem,
  CacheFileItem,
  TrashFileItem,
  LogFileItem,
  LargeFileItem,
  ScanSummary,
} from '@models/system.model';

export type { SystemServiceItem, CacheFileItem, TrashFileItem, LogFileItem, LargeFileItem, ScanSummary } from '@models/system.model';

@Injectable({
  providedIn: 'root',
})
export class SystemService {
  private mainService = inject(MainService);

  async getSystemServices(): Promise<SystemServiceItem[]> {
    return await this.mainService.getSystemServices<SystemServiceItem[]>();
  }

  async getCacheFiles(): Promise<CacheFileItem[]> {
    return await this.mainService.getCacheFiles<CacheFileItem[]>();
  }

  async getTrashFiles(): Promise<TrashFileItem[]> {
    return await this.mainService.getTrashFiles<TrashFileItem[]>();
  }

  async getSystemLogs(): Promise<LogFileItem[]> {
    return await this.mainService.getSystemLogs<LogFileItem[]>();
  }

  async getLargeFiles(): Promise<LargeFileItem[]> {
    return await this.mainService.getLargeFiles<LargeFileItem[]>();
  }

  async clearSelectedCacheFiles(paths: string[]): Promise<string> {
    return await this.mainService.clearSelectedCacheFiles<string>(paths);
  }

  async clearSelectedTrashFiles(paths: string[]): Promise<string> {
    return await this.mainService.clearSelectedTrashFiles<string>(paths);
  }

  async clearSelectedLogFiles(paths: string[]): Promise<string> {
    return await this.mainService.clearSelectedLogFiles<string>(paths);
  }

  async clearSelectedLargeFiles(paths: string[]): Promise<string> {
    return await this.mainService.clearSelectedLargeFiles<string>(paths);
  }

  async stopSelectedServices(services: string[]): Promise<string> {
    return await this.mainService.stopSelectedServices<string>(services);
  }

  async clearTrash(): Promise<string> {
    return await this.mainService.clearTrash<string>();
  }

  async clearCache(): Promise<string> {
    return await this.mainService.clearCache<string>();
  }

  async clearAllLogs(): Promise<string> {
    return await this.mainService.clearAllLogs<string>();
  }

  async clearAllLargeFiles(): Promise<string> {
    return await this.mainService.clearAllLargeFiles<string>();
  }

  async stopService(service: string): Promise<string> {
    return await this.mainService.stopService<string>(service);
  }

  async getCacheSummary(): Promise<ScanSummary> {
    return await this.mainService.getCacheSummary<ScanSummary>();
  }

  async getTrashSummary(): Promise<ScanSummary> {
    return await this.mainService.getTrashSummary<ScanSummary>();
  }

  async getLogSummary(): Promise<ScanSummary> {
    return await this.mainService.getLogSummary<ScanSummary>();
  }

  async getLargeFilesSummary(): Promise<ScanSummary> {
    return await this.mainService.getLargeFilesSummary<ScanSummary>();
  }

  async getAllServices(): Promise<SystemServiceItem[]> {
    return await this.mainService.getAllServices<SystemServiceItem[]>();
  }

  async enableService(service: string): Promise<string> {
    return await this.mainService.enableService<string>(service);
  }

  async startService(service: string): Promise<string> {
    return await this.mainService.startService<string>(service);
  }

  async enableSelectedServices(services: string[]): Promise<string> {
    return await this.mainService.enableSelectedServices<string>(services);
  }
}
