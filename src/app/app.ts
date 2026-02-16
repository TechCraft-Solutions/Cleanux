/* sys lib */
import { Component, signal, inject } from '@angular/core';
import { DOCUMENT } from '@angular/common';
import { RouterOutlet } from '@angular/router';

/* components */
import { SidebarComponent } from '@components/sidebar/sidebar.component';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, SidebarComponent],
  templateUrl: './app.html',
})
export class App {
  private document = inject(DOCUMENT);
  isDarkMode = signal(localStorage.getItem('theme') === 'dark');

  get isDark(): boolean {
    return this.document.body.classList.contains('dark');
  }

  toggleDarkMode() {
    this.isDarkMode.update((value) => !value);
    const theme = this.isDarkMode() ? 'dark' : 'light';
    localStorage.setItem('theme', theme);

    if (this.isDarkMode()) {
      this.document.body.classList.add('dark');
    } else {
      this.document.body.classList.remove('dark');
    }
  }

  constructor() {
    // Ensure dark mode is applied on start
    if (this.isDarkMode()) {
      this.document.body.classList.add('dark');
    } else {
      this.document.body.classList.remove('dark');
    }
  }
}
