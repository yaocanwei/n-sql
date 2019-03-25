import { Component, OnInit, ElementRef, ViewChild } from '@angular/core';
import * as CodeMirror from 'codemirror';
import { Editor } from 'codemirror';
import 'codemirror/mode/javascript/javascript';
import 'codemirror/addon/edit/closebrackets.js';

declare module 'codemirror' {
    interface EditorConfiguration {
        autoCloseBrackets?: boolean;
    }
}
@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.less']
})
export class AppComponent implements OnInit {
    constructor() {
        // this.api = 'https://yish.org/api';
        this.tables = [
            {
                text: 'student',
                displayText: '学生表',
                columns: [
                    {
                        text: '[年龄{23}]',
                        displayText: '年龄'
                    },
                    {
                        text: '[名字{3}]',
                        displayText: '名字'
                    }
                ]
            },
            {
                text: 'score',
                displayText: '成绩表',
                columns: [
                    {
                        text: 'a8', displayText: '数学'
                    },
                    {
                        text: 'c8', displayText: '语文'
                    }
                ]
            }
        ];
    }
    @ViewChild('jsonEditor')
    jsonEditorEl: ElementRef;
    jsonEditor: Editor;
    api: string;

    public tables: any[];

    results: Array<{ db: string, sql: string, elapse: number }> = [];
    ngOnInit(): void {
        this.jsonEditor = CodeMirror(this.jsonEditorEl.nativeElement, {
            mode: 'application/json',
            autoCloseBrackets: true
        });

        this.jsonEditor.setOption('theme', 'solarized light');

        setTimeout(() => {
            this.jsonEditor.setValue(JSON.stringify(this.tables, undefined, 2));
        }, 100);

        this.jsonEditor.on('change', (o) => this.tables = JSON.parse(o.getValue()));
    }
    async toSql(sql: string): Promise<string> {
        if (this.api) {
            const result = await fetch(`${this.api}/translate/ToSql`, {
                method: 'post', body: `"${sql}"`,
                headers: {
                    'Content-Type': 'application/json'
                }
            });
            return await result.text();
        } else {
            return window.n_sql.translate(sql);
        }
    }
    async translateToSql(sql: string) {
        await this.putResult('N-SQL', sql, this.toSql.bind(this));
    }
    async toNpgsql(sql: string): Promise<string> {
        if (this.api) {
            const result = await fetch(`${this.api}/translate/ToNpgsql`, {
                method: 'post', body: `"${sql}"`,
                headers: {
                    'Content-Type': 'application/json'
                }
            });
            return await result.text();
        } else {
            return window.n_sql.translate_pgsql(sql);
        }
    }
    async translateToNpgsql(sql: string) {
        await this.putResult('PostgreSQL', sql, this.toNpgsql.bind(this));
    }
    async toOracle(sql: string): Promise<string> {
        if (this.api) {
            const result = await fetch(`${this.api}/translate/ToOracle`, {
                method: 'post', body: `"${sql}"`,
                headers: {
                    'Content-Type': 'application/json'
                }
            });
            return await result.text();
        } else {
            return window.n_sql.translate_oracle(sql);
        }
    }
    async translateToOracle(sql: string) {
        await this.putResult('Oracle', sql, this.toOracle.bind(this));
    }
    async toMySQL(sql: string): Promise<string> {
        if (this.api) {
            const result = await fetch(`${this.api}/translate/ToMySQL`, {
                method: 'post', body: `"${sql}"`,
                headers: {
                    'Content-Type': 'application/json'
                }
            });
            return await result.text();
        } else {
            return window.n_sql.translate_mysql(sql);
        }
    }
    async translateToMySQL(sql: string) {
        await this.putResult('MySQL', sql, this.toMySQL.bind(this));
    }
    async toSqlServer(sql: string): Promise<string> {
        if (this.api) {
            const result = await fetch(`${this.api}/translate/ToSqlServer`, {
                method: 'post', body: `"${sql}"`,
                headers: {
                    'Content-Type': 'application/json'
                }
            });
            return await result.text();
        } else {
            return window.n_sql.translate_sql_server(sql);
        }
    }
    async translateToSqlServer(sql: string) {
        await this.putResult('SqlServer', sql, this.toSqlServer.bind(this));
    }
    async toSQLite(sql: string): Promise<string> {
        if (this.api) {
            const result = await fetch(`${this.api}/translate/ToSQLite`, {
                method: 'post', body: `"${sql}"`,
                headers: {
                    'Content-Type': 'application/json'
                }
            });
            return await result.text();
        } else {
            return window.n_sql.translate_sqlite(sql);
        }
    }

    async translateToSQLite(sql: string) {
        await this.putResult('SQLite', sql, this.toSQLite.bind(this));
    }

    async putResult(t: string, sql: string, translator: (sql: string) => Promise<string>) {
        let time = Date.now();
        try {
            sql = await translator(sql);
        } finally {
            time = Date.now() - time;
        }
        this.results.push({
            db: t,
            sql, elapse: time
        });
    }

}
