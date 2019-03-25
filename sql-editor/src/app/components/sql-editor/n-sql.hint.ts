import * as CodeMirror from 'codemirror';

declare module 'codemirror' {
    function showParameterInfo(cm: CodeMirror.Editor, hinter?: HintFunction, options?: ShowHintOptions);
}

let tooltipWidget: TooltipWidget;


class TooltipWidget {
    tooltip: HTMLDivElement;
    constructor(private cm: CodeMirror.Editor, private token: CodeMirror.Token, cursor: CodeMirror.Position) {
        this.tooltip = document.createElement('div');
        this.tooltip.innerText = `${token.string}函数的帮助`;
        const pos = cm.cursorCoords(cursor);
        const left = pos.left, top = pos.bottom, below = true;
        this.tooltip.style.left = left + 'px';
        this.tooltip.style.top = top + 'px';
        this.tooltip.style.position = 'absolute';
        this.tooltip.style.backgroundColor = 'chartreuse';
    }

    show() {
        console.log('显示函数帮助');
        document.body.appendChild(this.tooltip);

        this.cm.on('blur', () => this.close());
    }

    close() {
        this.tooltip.parentNode.removeChild(this.tooltip);
        this.cm.off('blur', () => this.close());
        tooltipWidget = undefined;
    }
}

let tables;
let defaultTable;
let keywords;
let functions;
let identifierQuote;
const CONS = {
    QUERY_DIV: ';',
    ALIAS_KEYWORD: 'AS'
};
const Pos = CodeMirror.Pos, cmpPos = CodeMirror.cmpPos;

function isArray(val) { return Object.prototype.toString.call(val) === '[object Array]'; }

function getMode(editor) {
    let mode = editor.doc.modeOption;
    if (mode === 'sql') {
        mode = 'text/x-sql';
    }
    return (CodeMirror as any).resolveMode(mode);
}

function getKeywords(editor) {
    return getMode(editor).keywords;
}

function getFunctions(editor) {
    return getMode(editor).functions;
}

function getIdentifierQuote(editor) {
    let mode = editor.doc.modeOption;
    if (mode === 'sql') {
        mode = 'text/x-sql';
    }
    return (CodeMirror as any).resolveMode(mode).identifierQuote || '`';
}

function getText(item): string {
    return typeof item === 'string' ? item : item.text;
}

function wrapTable(name, value) {
    if (isArray(value)) value = { columns: value };
    if (!value.text) value.text = name;
    return value;
}

function parseTables(input) {
    const result = {};
    if (isArray(input)) {
        for (let i = input.length - 1; i >= 0; i--) {
            const item = input[i];
            result[getText(item).toUpperCase()] = wrapTable(getText(item), item);
        }
    } else if (input) {
        for (const name in input) result[name.toUpperCase()] = wrapTable(name, input[name]);
    }
    return result;
}

function getTable(name) {
    return tables[name.toUpperCase()];
}

function shallowClone(object) {
    const result = {};
    for (const key in object) {
        if (object.hasOwnProperty(key)) result[key] = object[key];
    }
    return result;
}

function match(string, word) {
    const len = string.length;
    const sub = getText(word).substr(0, len);
    return string.toUpperCase() === sub.toUpperCase();
}

function addMatches(result, search, wordlist, formatter) {
    if (isArray(wordlist)) {
        for (let i = 0; i < wordlist.length; i++) {
            if (match(search, wordlist[i])) result.push(formatter(wordlist[i]));
        }
    } else {
        for (const word in wordlist) {
            if (wordlist.hasOwnProperty(word)) {
                let val = wordlist[word];
                if (!val || val === true || val.param) {
                    val = word;
                } else {
                    val = val.displayText ? { text: val.text, displayText: val.displayText } : val.text;
                }
                if (match(search, val)) result.push(formatter(val));
            }
        }
    }
}

function cleanName(name) {
    // Get rid name from identifierQuote and preceding dot(.)
    if (name.charAt(0) === '.') {
        name = name.substr(1);
    }
    // replace doublicated identifierQuotes with single identifierQuotes
    // and remove single identifierQuotes
    const nameParts = name.split(identifierQuote + identifierQuote);
    for (let i = 0; i < nameParts.length; i++) {
        nameParts[i] = nameParts[i].replace(new RegExp(identifierQuote, 'g'), '');
    }
    return nameParts.join(identifierQuote);
}

function insertIdentifierQuotes(name) {
    const nameParts = getText(name).split('.');
    for (let i = 0; i < nameParts.length; i++) {
        nameParts[i] = identifierQuote +
            // doublicate identifierQuotes
            nameParts[i].replace(new RegExp(identifierQuote, 'g'), identifierQuote + identifierQuote) +
            identifierQuote;
    }
    const escaped = nameParts.join('.');
    if (typeof name === 'string') return escaped;
    name = shallowClone(name);
    name.text = escaped;
    return name;
}

function nameCompletion(cur, token, result, editor) {
    // Try to complete table, column names and return start position of completion
    let useIdentifierQuotes = false;
    const nameParts = [];
    let start = token.start;
    let cont = true;
    while (cont) {
        cont = (token.string.charAt(0) === '.');
        useIdentifierQuotes = useIdentifierQuotes || (token.string.charAt(0) === identifierQuote);

        start = token.start;
        nameParts.unshift(cleanName(token.string));

        token = editor.getTokenAt(Pos(cur.line, token.start));
        if (token.string === '.') {
            cont = true;
            token = editor.getTokenAt(Pos(cur.line, token.start));
        }
    }

    // Try to complete table names
    let string = nameParts.join('.');
    addMatches(result, string, tables, (w) => useIdentifierQuotes ? insertIdentifierQuotes(w) : w);

    // Try to complete columns from defaultTable
    addMatches(result, string, defaultTable, (w) => useIdentifierQuotes ? insertIdentifierQuotes(w) : w);

    // Try to complete columns
    string = nameParts.pop();
    let table = nameParts.join('.');

    let alias = false;
    const aliasTable = table;
    // Check if table is available. If not, find table by Alias
    if (!getTable(table)) {
        const oldTable = table;
        table = findTableByAlias(table, editor);
        if (table !== oldTable) alias = true;
    }

    let columns = getTable(table);
    if (columns && columns.columns) {
        columns = columns.columns;
    }

    if (columns) {
        addMatches(result, string, columns, function (w) {
            let tableInsert = table;
            if (alias === true) tableInsert = aliasTable;
            if (typeof w === 'string') {
                w = tableInsert + '.' + w;
            } else {
                w = shallowClone(w);
                w.text = tableInsert + '.' + w.text;
            }
            w.className = 'CodeMirror-hint-column';
            return useIdentifierQuotes ? insertIdentifierQuotes(w) : w;
        });
    }

    return start;
}

function eachWord(lineText, f) {
    const words = lineText.split(/\s+/);
    for (let i = 0; i < words.length; i++) {
        if (words[i]) f(words[i].replace(/[,;]/g, ''));
    }
}

function findTableByAlias(alias, editor) {
    const doc = editor.doc;
    const fullQuery = doc.getValue();
    const aliasUpperCase = alias.toUpperCase();
    let previousWord = '';
    let table = '';
    const separator = [];
    let validRange = {
        start: Pos(0, 0),
        end: Pos(editor.lastLine(), editor.getLineHandle(editor.lastLine()).length)
    };

    // add separator
    let indexOfSeparator = fullQuery.indexOf(CONS.QUERY_DIV);
    while (indexOfSeparator !== -1) {
        separator.push(doc.posFromIndex(indexOfSeparator));
        indexOfSeparator = fullQuery.indexOf(CONS.QUERY_DIV, indexOfSeparator + 1);
    }
    separator.unshift(Pos(0, 0));
    separator.push(Pos(editor.lastLine(), editor.getLineHandle(editor.lastLine()).text.length));

    // find valid range
    let prevItem = null;
    const current = editor.getCursor();
    for (let i = 0; i < separator.length; i++) {
        if ((prevItem == null || cmpPos(current, prevItem) > 0) && cmpPos(current, separator[i]) <= 0) {
            validRange = { start: prevItem, end: separator[i] };
            break;
        }
        prevItem = separator[i];
    }

    if (validRange.start) {
        const query = doc.getRange(validRange.start, validRange.end, false);

        for (let i = 0; i < query.length; i++) {
            const lineText = query[i];
            eachWord(lineText, (word) => {
                const wordUpperCase = word.toUpperCase();
                if (wordUpperCase === aliasUpperCase && getTable(previousWord)) {
                    table = previousWord;
                }
                if (wordUpperCase !== CONS.ALIAS_KEYWORD) {
                    previousWord = word;
                }
            });
            if (table) break;
        }
    }
    return table;
}

let hintTooltip: HTMLDivElement;
let theme: string;
const handlers = {
    'select': [
        (data, element: HTMLLIElement) => {
            if (!hintTooltip) {
                hintTooltip = document.createElement('div');
                hintTooltip.className = 'CodeMirror-hint-tooltip ' + theme;
                document.body.appendChild(hintTooltip);
            }
            hintTooltip.style.top = `${element.parentElement.offsetTop + element.offsetTop}px`;
            hintTooltip.style.left = `${element.parentElement.offsetLeft + element.clientWidth}px`;
            if (data.detail) {
                hintTooltip.innerText = data.detail.description || data.text + '函数说明，样例。。。。';
            } else {
                hintTooltip.innerText = data.text + '函数说明，样例。。。。';
            }
        }
    ],
    'close': [
        (data, element: HTMLLIElement) => {
            if (hintTooltip) {
                document.body.removeChild(hintTooltip);
                hintTooltip = undefined;
            }
        }
    ]
};

function functionRender(element: HTMLLIElement, self, data) {
    if (data.detail) {
        element.innerText = data.detail.text;
        if (data.detail.param) {
            const span = document.createElement('span');
            span.innerText = data.detail.param;
            span.className = 'CodeMirror-hint-function-param';
            element.appendChild(span);
        }
    }
}

function pick(cm: CodeMirror.Editor, self, data): void {
    let text = getText(self);
    let from: CodeMirror.Position = self.from || data.from;
    let to: CodeMirror.Position = self.to || data.to;
    if (self.className === 'CodeMirror-hint-keyword') {
        text += ' ';
    }
    cm.replaceRange(text, from, to, 'complete');
    if (self.className === 'CodeMirror-hint-function') {
        const line = cm.getLine(to.line);
        const p: number = to.ch + 1;
        if (line.charAt(p) !== '(') {
            const length = text.length;
            from = {line: from.line, ch: from.ch + length};
            to = from;
            cm.replaceRange('()', from, to, 'complete');
            if (self.detail && self.detail.param !== '()') {
                cm.execCommand('goCharLeft');
            }
        }

    }
}

CodeMirror.registerHelper('hint', 'n-sql', (editor, options) => {
    theme = editor.options.theme;
    tables = parseTables(options && options.tables);
    const defaultTableName = options && options.defaultTable;
    const disableKeywords = options && options.disableKeywords;
    defaultTable = defaultTableName && getTable(defaultTableName);
    keywords = getKeywords(editor);
    functions = getFunctions(editor);
    identifierQuote = getIdentifierQuote(editor);

    if (defaultTableName && !defaultTable) {
        defaultTable = findTableByAlias(defaultTableName, editor);
    }

    defaultTable = defaultTable || [];

    if (defaultTable.columns) {
        defaultTable = defaultTable.columns;
    }

    const cur = editor.getCursor();

    const token = editor.getTokenAt(cur);
    if (token.type === 'string' || token.type === 'comment') {
        return undefined;
    }
    const result = [];
    const line = editor.getLine(cur.line);
    let start, end, search;
    if (token.end > cur.ch) {
        token.end = cur.ch;
        token.string = token.string.slice(0, cur.ch - token.start);
    }

    if (token.string.match(/^[.`"\w@]\w*$/)) {
        search = token.string;
        start = token.start;
        end = token.end;
    } else {
        start = end = cur.ch;
        search = '';
    }

    if (search === '' && (window.event && (window.event as KeyboardEvent).key === 'Backspace')) {
        return undefined;
    }

    while (end < line.length && /\w/.test(line.charAt(end)))++end;

    if (search.charAt(0) === '.' || search.charAt(0) === identifierQuote) {
        start = nameCompletion(cur, token, result, editor);
    } else {
        addMatches(result, search, defaultTable, (w) => ({ text: w, className: 'CodeMirror-hint-table CodeMirror-hint-default-table' }));
        addMatches(
            result,
            search,
            tables,
            (w) => {
                if (typeof w === 'object') {
                    w.className = 'CodeMirror-hint-table';
                } else {
                    w = { text: w, className: 'CodeMirror-hint-table' };
                }
                return w;
            }
        );
        if (!disableKeywords) {
            addMatches(result, search, keywords, (w) => ({ text: w, pick: pick, className: 'CodeMirror-hint-keyword' }));
        }

        addMatches(result, search, functions,
            (w) => ({ text: w.text, pick: pick, className: 'CodeMirror-hint-function', detail: w, render: functionRender }));
    }

    return {
        list: result, from: Pos(cur.line, start), to: Pos(cur.line, end),
        _handlers: handlers
    };
});


CodeMirror.commands.showParameterInfo = CodeMirror.showParameterInfo;
