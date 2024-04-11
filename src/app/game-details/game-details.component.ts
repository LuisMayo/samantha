import { ActivatedRoute, Router } from '@angular/router';
import { Component, OnInit, signal } from '@angular/core';

import { CommonModule } from '@angular/common';
import { invoke } from '@tauri-apps/api';

export interface Achievement {
  name: string;
  unlocked: boolean;
  description?: string;
  hidden: boolean;
  image: string;
}

@Component({
  selector: 'app-game-details',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './game-details.component.html',
  styleUrl: './game-details.component.css'
})
export class GameDetailsComponent implements OnInit {
  achievements = signal<Achievement[] | null>(null);
  constructor(private route: ActivatedRoute, private router: Router) {}

  ngOnInit(): void {
    const id = this.route.snapshot.paramMap.get('id');

    if(id == null) {
      this.router.navigate(['/']);
    } else {
      invoke<Achievement[]>("get_achievement_list", {appid: +id}).then((achievements) => {
        this.achievements.set(achievements);
      }).catch((e) => alert(e));
    }
  }
}
