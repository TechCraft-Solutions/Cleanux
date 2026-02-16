/* sys lib */
import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable } from 'rxjs';

/* env */
import { environment } from '@env/environment';

const httpOptions = {
  headers: new HttpHeaders({
    'Content-Type': 'application/json',
  }),
};

@Injectable({
  providedIn: 'root',
})
export class AboutService {
  constructor(private http: HttpClient) { }

  gitRepoName: string = environment.gitRepoName;
  githubUser: string = environment.githubUser;

  getDate(version: string): Observable<any> {
    return this.http.get<any>(
      `https://api.github.com/repos/${this.githubUser}/${this.gitRepoName}/releases/tags/v${version}`,
      httpOptions
    );
  }

  checkUpdate(): Observable<any> {
    return this.http.get<any>(
      `https://api.github.com/repos/${this.githubUser}/${this.gitRepoName}/releases/latest`,
      httpOptions
    );
  }
}
