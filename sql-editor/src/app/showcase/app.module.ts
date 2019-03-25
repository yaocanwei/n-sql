import {BrowserModule} from '@angular/platform-browser';
import {BrowserAnimationsModule} from '@angular/platform-browser/animations';
import {NgModule} from '@angular/core';

import {AppRoutingModule} from './app-routing.module';
import {AppComponent} from './app.component';
import {ButtonModule} from 'primeng/button';
import {FlexLayoutModule} from '@angular/flex-layout';
import {ListboxModule} from 'primeng/listbox';
import {ToolbarModule} from 'primeng/toolbar';
import {TabViewModule} from 'primeng/tabview';
import {ScrollPanelModule} from 'primeng/scrollpanel';
import {CardModule} from 'primeng/card';
import {SqlEditorModule} from '../components/sql-editor';


@NgModule({
    declarations: [
        AppComponent
    ],
    imports: [
        BrowserModule, BrowserAnimationsModule, SqlEditorModule, ButtonModule, FlexLayoutModule,
        AppRoutingModule, ListboxModule, ToolbarModule, TabViewModule, ScrollPanelModule,
        CardModule
    ],
    providers: [],
    bootstrap: [AppComponent]
})
export class AppModule {
}
