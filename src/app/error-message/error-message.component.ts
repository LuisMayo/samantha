import { Component, Inject } from '@angular/core';
import { MAT_DIALOG_DATA, MatDialogModule } from '@angular/material/dialog';

@Component({
  selector: 'app-error-message',
  standalone: true,
  imports: [MatDialogModule],
  templateUrl: './error-message.component.html',
  styleUrl: './error-message.component.css'
})
export class ErrorMessageComponent {
  constructor(@Inject(MAT_DIALOG_DATA) public data: {error: string}) {}
}
