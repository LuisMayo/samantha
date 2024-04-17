import { Component, computed, signal } from '@angular/core';

import { Game } from '../types';
import {MatCardModule} from '@angular/material/card';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { RouterModule } from '@angular/router';
import { invoke } from '@tauri-apps/api';

@Component({
  selector: 'app-game-list',
  standalone: true,
  imports: [RouterModule, MatProgressSpinnerModule, MatCardModule],
  templateUrl: './game-list.component.html',
  styleUrl: './game-list.component.css'
})
export class GameListComponent {
  games = signal<Game[] | null>(null);
  sortedGames = computed(() => this.games()?.sort((a, b) => (a.name > b.name) ? 1 : ((b.name > a.name) ? -1 : 0)))

  ngOnInit(): void {
    // TODO esto debe ir por tiempo y usuario?
    const ownedGames = localStorage.getItem('games');
    if (ownedGames) {
      this.games.set(JSON.parse(ownedGames));
    } else {
      invoke<Game[]>("locate_games").then((ownedGames) => {
        localStorage.setItem('games', JSON.stringify(ownedGames));
        this.games.set(ownedGames);
      }).catch((e) => console.error(e));
    }
  }
}
