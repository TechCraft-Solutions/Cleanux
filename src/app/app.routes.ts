/* sys lib */
import { Routes } from '@angular/router';

export const routes: Routes = [
  { path: '', redirectTo: 'dashboard', pathMatch: 'full' },
  { 
    path: 'dashboard', 
    loadComponent: () => import('@views/dashboard/dashboard.view').then(m => m.DashboardView) 
  },
  { 
    path: 'cleaner', 
    loadComponent: () => import('@views/cleaner/cleaner.view').then(m => m.CleanerView) 
  },
  { 
    path: 'system', 
    loadComponent: () => import('@views/system/system.view').then(m => m.SystemView) 
  },
  { 
    path: 'large-files', 
    loadComponent: () => import('@views/large-files/large-files.view').then(m => m.LargeFilesView) 
  },
  { 
    path: 'settings', 
    loadComponent: () => import('@views/settings/settings.view').then(m => m.SettingsView) 
  },
  // Add more routes as needed
];
