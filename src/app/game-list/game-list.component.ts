import { Component, signal } from '@angular/core';
import { Game } from '../types';
import { invoke } from '@tauri-apps/api';
import { RouterModule } from '@angular/router';

@Component({
  selector: 'app-game-list',
  standalone: true,
  imports: [RouterModule],
  templateUrl: './game-list.component.html',
  styleUrl: './game-list.component.css'
})
export class GameListComponent {
  games = signal<Game[] | null>(null);

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
