import { Routes } from "@angular/router";
import { GameListComponent } from "./game-list/game-list.component";
import { GameDetailsComponent } from "./game-details/game-details.component";

export const routes: Routes = [
    {
        path: '',
        component: GameListComponent
    },
    {
        path: 'game/:id',
        component: GameDetailsComponent
    }
];
