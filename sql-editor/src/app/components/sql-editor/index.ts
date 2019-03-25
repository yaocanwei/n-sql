import {SqlEditorComponent} from './sql-editor.component';
import {CommonModule} from '@angular/common';
import {NgModule} from '@angular/core';
import {TooltipComponent} from './widgets/tooltip/tooltip.component';


@NgModule({
    imports: [
        CommonModule
    ],
    declarations: [
        SqlEditorComponent, TooltipComponent
    ],
    exports: [
        SqlEditorComponent
    ]
})
export class SqlEditorModule {
}
