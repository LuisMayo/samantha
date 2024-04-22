import { Component, Inject } from '@angular/core';
import {
  MAT_DIALOG_DATA,
  MatDialogModule,
} from '@angular/material/dialog';

@Component({
  selector: 'app-confirmation-popup',
  standalone: true,
  imports: [MatDialogModule],
  templateUrl: './confirmation-popup.component.html',
  styleUrl: './confirmation-popup.component.css'
})
export class ConfirmationPopupComponent {
  constructor(@Inject(MAT_DIALOG_DATA) public data: {onAccept: () => void}) { }
}
