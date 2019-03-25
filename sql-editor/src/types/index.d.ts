declare module WebAssembly {
    interface Memory {
        a: any;
    }
}

declare interface NSql {
    translate(arg0: string): string;

    translate_pgsql(arg0: string): string;

    translate_oracle(arg0: string): string;

    translate_mysql(arg0: string): string;

    translate_sql_server(arg0: string): string;

    translate_sqlite(arg0: string): string;
}

declare interface Window {
    n_sql: NSql;
}
