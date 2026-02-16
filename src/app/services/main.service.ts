/* sys lib */
import { Injectable, inject } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';

/* models */
import { Response, getData } from '@models/response.model';

@Injectable({
  providedIn: 'root',
})
export class MainService {
  constructor() { }

  /**
   * Invokes a Tauri command and returns the data T.
   * Handles Response status checks automatically.
   */
  async invoke<T>(
    command: string,
    args?: Record<string, unknown>,
    options: { suppressError?: boolean } = {}
  ): Promise<T> {
    try {
      const response = await invoke<Response>(command, args);
      if (response.status === 'success') {
        return getData<T>(response) as T;
      } else {
        throw new Error(response.message || `Operation failed: ${command}`);
      }
    } catch (error: any) {
      if (!options.suppressError) {
        console.error(`Error invoking command "${command}":`, error);
        const errorMessage =
          typeof error === 'string' ? error : error?.message || 'An unknown error occurred';
        console.error(errorMessage);
      }
      throw error;
    }
  }

  async getCacheFiles<T>(): Promise<T> {
    return await this.invoke<T>('getCacheFiles');
  }

  async getTrashFiles<T>(): Promise<T> {
    return await this.invoke<T>('getTrashFiles');
  }

  async getSystemLogs<T>(): Promise<T> {
    return await this.invoke<T>('getSystemLogs');
  }

  async getLargeFiles<T>(): Promise<T> {
    return await this.invoke<T>('getLargeFiles');
  }

  async getSystemServices<T>(): Promise<T> {
    return await this.invoke<T>('getSystemServices');
  }

  async getCacheSummary<T>(): Promise<T> {
    return await this.invoke<T>('getCacheSummary');
  }

  async getTrashSummary<T>(): Promise<T> {
    return await this.invoke<T>('getTrashSummary');
  }

  async getLogSummary<T>(): Promise<T> {
    return await this.invoke<T>('getLogSummary');
  }

  async getLargeFilesSummary<T>(): Promise<T> {
    return await this.invoke<T>('getLargeFilesSummary');
  }

  async clearSelectedCacheFiles<T>(paths: string[]): Promise<T> {
    return await this.invoke<T>('clearSelectedCacheFiles', { paths });
  }

  async clearSelectedTrashFiles<T>(paths: string[]): Promise<T> {
    return await this.invoke<T>('clearSelectedTrashFiles', { paths });
  }

  async clearSelectedLogFiles<T>(paths: string[]): Promise<T> {
    return await this.invoke<T>('clearSelectedLogFiles', { paths });
  }

  async clearSelectedLargeFiles<T>(paths: string[]): Promise<T> {
    return await this.invoke<T>('clearSelectedLargeFiles', { paths });
  }

  async clearTrash<T>(): Promise<T> {
    return await this.invoke<T>('clearTrash');
  }

  async clearCache<T>(): Promise<T> {
    return await this.invoke<T>('clearCache');
  }

  async clearAllLogs<T>(): Promise<T> {
    return await this.invoke<T>('clearAllLogs');
  }

  async clearAllLargeFiles<T>(): Promise<T> {
    return await this.invoke<T>('clearAllLargeFiles');
  }

  async stopService<T>(service: string): Promise<T> {
    return await this.invoke<T>('stopService', { service });
  }

  async stopSelectedServices<T>(services: string[]): Promise<T> {
    return await this.invoke<T>('stopSelectedServices', { services });
  }

  async previewFile<T>(path: string): Promise<T> {
    return await this.invoke<T>('previewFile', { path });
  }

  async openFile<T>(path: string, command?: string): Promise<T> {
    return await this.invoke<T>('openFile', { path, command: command || null });
  }

  async getAllServices<T>(): Promise<T> {
    return await this.invoke<T>('getAllServices');
  }

  async enableService<T>(service: string): Promise<T> {
    return await this.invoke<T>('enableService', { service });
  }

  async startService<T>(service: string): Promise<T> {
    return await this.invoke<T>('startService', { service });
  }

  async enableSelectedServices<T>(services: string[]): Promise<T> {
    return await this.invoke<T>('enableSelectedServices', { services });
  }
}
