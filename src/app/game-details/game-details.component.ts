import { ActivatedRoute, Router } from '@angular/router';
import { Component, OnInit, signal } from '@angular/core';

import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTableModule } from '@angular/material/table';
import { invoke } from '@tauri-apps/api';

export interface Achievement {
  name: string;
  screen_name?: string;
  unlocked: boolean;
  toBeUnlocked: boolean;
  description?: string;
  hidden: boolean;
  image: string;
}

@Component({
  selector: 'app-game-details',
  standalone: true,
  imports: [CommonModule, MatProgressSpinnerModule, MatTableModule, FormsModule],
  templateUrl: './game-details.component.html',
  styleUrl: './game-details.component.css'
})
export class GameDetailsComponent implements OnInit {
  readonly COLUMNS = ['icon', 'screen_name', 'description', 'currently_unlocked', 'toBeUnlocked']
  achievements = signal<Achievement[] | null>(null);
  constructor(private route: ActivatedRoute, private router: Router) { }

  ngOnInit(): void {
    const id = this.route.snapshot.paramMap.get('id');

    if (id == null) {
      this.router.navigate(['/']);
    } else {
      invoke<Omit<Achievement, 'toBeUnlocked'>[]>("get_achievement_list", { appid: +id }).then((achievements) => {
        this.achievements.set(achievements.map(ach => ({...ach, toBeUnlocked: ach.unlocked})));
      }).catch((e) => alert(e));
    }
  }

  getChangedAchievements() {
    return this.achievements()?.filter(ach => ach.unlocked != ach.toBeUnlocked);
  }

  isAnyAchievementchanged() {
    return (this.getChangedAchievements() || []).length > 0;
  }
}
