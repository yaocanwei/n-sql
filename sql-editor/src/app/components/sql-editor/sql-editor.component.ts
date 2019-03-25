import {Component, ElementRef, OnDestroy, OnInit, Input, AfterViewInit, ViewChild} from '@angular/core';
import * as CodeMirror from 'codemirror';
import {Editor, Token} from 'codemirror';
import './addon/hint/show-hint';
import 'codemirror/addon/comment/comment';
import 'codemirror/addon/edit/closebrackets';
import 'codemirror/addon/edit/matchbrackets';
import './n-sql.mode';
import './n-sql.hint';
import {TooltipComponent} from './widgets/tooltip/tooltip.component';
const AUTO_COMPLETE_CHARS = /^[\\.a-zA-Z0-9_@(]$/;

declare module 'codemirror' {
    interface Editor {
        container: SqlEditorComponent;
    }
}

@Component({
    selector: 'sql-editor',
    template: '<tooltip [editor]="editor" #tooltip></tooltip>'
})
export class SqlEditorComponent implements OnInit, OnDestroy, AfterViewInit {
    @ViewChild('tooltip')
    private tooltip: TooltipComponent;
    public editor: Editor;

    @Input()
    public tables: any[];

    public value = 'select NVL(name, \'No Name\'), age from student skip 2 limit 3';

    getHintOptions(completeSingle: boolean = true): any {
        return {
            completeSingle: completeSingle,
            hint: (CodeMirror as any).hint['n-sql'],
            tables: this.tables
        };
    }
    constructor(private el: ElementRef) {
    }

    ngOnInit() {
        this.editor = CodeMirror(this.el.nativeElement, ({
            extraKeys: {
                'Cmd-Space': (cm: any) => cm.showHint(this.getHintOptions()),
                'Ctrl-Space': (cm: any) => cm.showHint(this.getHintOptions()),
                'Alt-Space': (cm: any) => cm.showHint(this.getHintOptions()),
                'Cmd-/': 'toggleComment',
                'Ctrl-/': 'toggleComment',
                'Cmd-P': this.showParameterInfo.bind(this),
                'Ctrl-P': this.showParameterInfo.bind(this),
            },
            autoCloseBrackets: true,
            matchBrackets: true,
            mode: 'text/n-sql'
        }) as any);
        console.log(CodeMirror.version);
        this.editor.setValue(this.value);
        this.editor.setOption('theme', 'solarized light');
        this.editor.on('keyup', this.onKeyUp.bind(this));
        this.editor.on('change', (o) => this.value = o.getValue());
        this.editor.container = this;
    }

    ngOnDestroy(): void {
        this.editor.off('keyup', this.onKeyUp.bind(this));
    }

    onKeyUp(cm: any, event: KeyboardEvent) {
        if (AUTO_COMPLETE_CHARS.test(event.key) || event.key === 'Backspace') {
            cm.showHint(this.getHintOptions(false));
        }
    }

    showParameterInfo() {
        // const mode = getMode(cm);
        const cur = Object.assign({}, this.editor.getCursor());
        let token: Token;

        while (cur.ch > 0) {
            token = this.editor.getTokenAt(cur);

            if (token.type === 'bracket' && token.string === '(') {
                cur.ch = token.start;
                token = this.editor.getTokenAt(cur);
                if (token.type === 'def') {
                    break;
                }
            }

            if (token.type === 'bracket' && token.string === ')') {
                while (true) {
                    cur.ch = token.start;
                    token = this.editor.getTokenAt(cur);
                    if (token.type === 'bracket' && token.string === '(') {
                        break;
                    }
                }
            }

            cur.ch = token.start;
        }
        if (token && token.type === 'def') {
            this.tooltip.show(token, this.editor.cursorCoords(cur));
        }
    }

    ngAfterViewInit(): void {
        // document.body.append(this.el.nativeElement);
    }
}
