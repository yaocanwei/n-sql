import {ChangeDetectorRef, Component, HostBinding, Input, OnInit} from '@angular/core';
import {Editor, Token} from 'codemirror';
import * as CodeMirror from 'codemirror';

@Component({
    selector: 'tooltip',
    template: `<span>{{text}}</span>`,
    // styleUrls: ['./tooltip.component.less'],
    styles: [`
        :host {
            position: absolute;
            z-Index : 11;
            margin-Left : 7px;
            overflow: hidden;
            list-style: none;
            padding: 2px;
            box-shadow: 2px 3px 5px rgba(0,0,0,.2);
            border-radius: 3px;
            border: 1px solid silver;
            background: white;
            font-size: 90%;
            font-family: monospace;
            max-height: 20em;
            overflow-y: auto;
        }
    `]
})
export class TooltipComponent implements OnInit {
    @HostBinding('style.display') display = 'none';
    // @HostBinding('style.position') relative = 'absolute';
    @HostBinding('style.left.px') left = 20;
    @HostBinding('style.top.px') top = 20;
    @Input()
    editor: Editor;
    mode;
    text: string;

    activityFunc: () => void;

    get visible(): boolean {
        return this.display !== 'none';
    }

    constructor() {
    }

    ngOnInit() {
        this.mode = this.getMode();
    }

    cursorActivity() {
        this.close();
    }

    public show(token: Token, position: { left: number; right?: number; top: number; bottom: number; }) {
        this.display = 'block';
        this.left = position.left;
        this.top = position.bottom;
        this.editor.on('cursorActivity', this.activityFunc = () => this.cursorActivity());

        if (token.type === 'def') {
            const f = this.mode.functions.find(o => o.text === token.string);
            if (f) {
                this.text = `${f.text}${f.param}`;
            }
        }
    }

    public close() {
        this.display = 'none';
        this.editor.off('cursorActivity', this.activityFunc);
    }


    private getMode() {
        let mode = (this.editor as any).doc.modeOption;
        if (mode === 'sql') {
            mode = 'text/x-sql';
        }
        return (CodeMirror as any).resolveMode(mode);
    }
}
