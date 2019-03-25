
import * as CodeMirror from 'codemirror';

export interface FunctionDefinition {
    param: string;
    description: string;
}

// these keywords are used by all SQL dialects (however, a mode can still overwrite it)
// noinspection TsLint
const sqlKeywords = `alter and as asc between by count create delete desc distinct drop from group having in insert into is join like
not on or order select set table union update values where limit skip`;

// turn a space-separated list into an array
function set(str) {
    const obj = {}, words: string[] = str.split(' ');
    for (let i = 0; i < words.length; ++i) {
        obj[words[i].trim()] = true;
    }
    return obj;
}

export let keywords = [];
export let functions = [];

class State {
    context;
    indent;
    tokenize: (stream, state: State) => any;
    constructor(tokenize: (stream, state: State) => any, context = null) {
        this.tokenize = tokenize;
        this.context = context;
    }
    p = 666666;
}

CodeMirror.defineMode('sql', (config, parserConfig): any => {
    const client = parserConfig.client || {},
        atoms = parserConfig.atoms || { 'false': true, 'true': true, 'null': true },
        builtin = parserConfig.builtin || {},
        operatorChars = parserConfig.operatorChars || /^[*+\-%<>!=&|~^]/,
        support = parserConfig.support || {},
        hooks = parserConfig.hooks || {},
        dateSQL = parserConfig.dateSQL || { 'date': true, 'time': true, 'timestamp': true },
        backslashStringEscapes = parserConfig.backslashStringEscapes !== false,
        brackets = parserConfig.brackets || /^[\{}\(\)\[\]]/,
        punctuation = parserConfig.punctuation || /^[;.,:]/;

    keywords = parserConfig.keywords || {};
    functions = parserConfig.functions;
    function tokenBase(stream, state: State) {
        const ch = stream.next();

        // call hooks from the mime type
        if (hooks[ch]) {
            const result = hooks[ch](stream, state);
            if (result !== false) {
                return result;
            }
        }

        if (support.hexNumber &&
            ((ch === '0' && stream.match(/^[xX][0-9a-fA-F]+/))
                || (ch === 'x' || ch === 'X') && stream.match(/^'[0-9a-fA-F]+'/))) {
            // hex
            return 'number';
        } else if (support.binaryNumber &&
            (((ch === 'b' || ch === 'B') && stream.match(/^'[01]+'/))
                || (ch === '0' && stream.match(/^b[01]+/)))) {
            // bitstring
            return 'number';
        } else if (ch.charCodeAt(0) > 47 && ch.charCodeAt(0) < 58) {
            // numbers
            stream.match(/^[0-9]*(\.[0-9]+)?([eE][-+]?[0-9]+)?/);
            return 'number';
        } else if (ch === '?' && (stream.eatSpace() || stream.eol() || stream.eat(';'))) {
            return 'variable-3';
        } else if (ch === '\'' || (ch === '"' && support.doubleQuote)) {
            state.tokenize = tokenLiteral(ch);
            return state.tokenize(stream, state);
        } else if ((((support.nCharCast && (ch === 'n' || ch === 'N'))
            || (support.charsetCast && ch === '_' && stream.match(/[a-z][a-z0-9]*/i)))
            && (stream.peek() === '\'' || stream.peek() === '"'))) {
            return 'keyword';
        } else if (support.commentSlashSlash && ch === '/' && stream.eat('/')) {
            // 1-line comment
            stream.skipToEnd();
            return 'comment';
        } else if ((support.commentHash && ch === '#')
            || (ch === '-' && stream.eat('-') && (!support.commentSpaceRequired || stream.eat(' ')))) {
            // 1-line comments
            // ref: https://kb.askmonty.org/en/comment-syntax/
            stream.skipToEnd();
            return 'comment';
        } else if (ch === '/' && stream.eat('*')) {
            // multi-line comments
            // ref: https://kb.askmonty.org/en/comment-syntax/
            state.tokenize = tokenComment(1);
            return state.tokenize(stream, state);
        } else if (ch === '.') {
            // .1 for 0.1
            if (support.zerolessFloat && stream.match(/^(?:\d+(?:e[+-]?\d+)?)/i)) {
                return 'number';
            }
            if (stream.match(/^\.+/)) {
                return null;
            }
            if (support.ODBCdotTable && stream.match(/^[\w\d_]+/)) {
                return 'variable-2';
            }
        } else if (operatorChars.test(ch)) {
            // operators
            stream.eatWhile(operatorChars);
            return 'operator';
        } else if (brackets.test(ch)) {
            // brackets
            return 'bracket';
        } else if (punctuation.test(ch)) {
            // punctuation
            stream.eatWhile(punctuation);
            return 'punctuation';
        } else if (ch === '{' &&
            (stream.match(/^( )*(d|D|t|T|ts|TS)( )*'[^']*'( )*}/) || stream.match(/^( )*(d|D|t|T|ts|TS)( )*"[^"]*"( )*}/))) {

            return 'number';
        } else {
            stream.eatWhile(/^[_\w\d]/);
            const word: string = stream.current().toLowerCase();

            if (dateSQL.hasOwnProperty(word) && (stream.match(/^( )+'[^']*'/) || stream.match(/^( )+"[^"]*"/))) {
                return 'number';
            }
            if (atoms.hasOwnProperty(word)) {
                return 'atom';
            }
            if (builtin.hasOwnProperty(word)) {
                return 'builtin';
            }
            if (keywords.hasOwnProperty(word)) {
                return 'keyword';
            }
            if (functions.some(o => o.text === word)) {
                return 'def';
            }
            if (client.hasOwnProperty(word)) {
                return 'string-2';
            }
            return null;
        }
    }

    // 'string', with char specified in quote escaped by '\'
    function tokenLiteral(quote) {
        return function (stream, state) {
            let escaped = false, ch;
            while ((ch = stream.next()) != null) {
                if (ch === quote && !escaped) {
                    state.tokenize = tokenBase;
                    break;
                }
                escaped = backslashStringEscapes && !escaped && ch === '\\';
            }
            return 'string';
        };
    }

    function tokenComment(depth) {
        return function (stream, state) {
            const m = stream.match(/^.*?(\/\*|\*\/)/);
            if (!m) {
                stream.skipToEnd();
            } else if (m[1] === '/*') {
                state.tokenize = tokenComment(depth + 1);
            } else if (depth > 1) {
                state.tokenize = tokenComment(depth - 1);
            } else {
                state.tokenize = tokenBase;
            }
            return 'comment';
        };
    }

    function pushContext(stream, state: State, type) {

        state.context = {
            prev: state.context,
            indent: stream.indentation(),
            col: stream.column(),
            type: type
        };
    }

    function popContext(state: State) {
        state.indent = state.context.indent;
        state.context = state.context.prev;
    }

    return {
        startState: () => {
            return new State(tokenBase);
        },

        token: (stream, state: State) => {
            if (stream.sol()) {
                if (state.context && state.context.align == null) {
                    state.context.align = false;
                }
            }
            if (state.tokenize === tokenBase && stream.eatSpace()) {
                return null;
            }

            const style = state.tokenize(stream, state);
            if (style === 'comment') {
                return style;
            }

            if (state.context && state.context.align == null) {
                state.context.align = true;
            }

            const tok = stream.current();
            if (tok === '(') {
                pushContext(stream, state, ')');
            } else if (tok === '[') {
                pushContext(stream, state, ']');
            } else if (state.context && state.context.type === tok) {
                popContext(state);
            }
            return style;
        },

        indent: (state: State, textAfter) => {
            const cx = state.context;
            if (!cx) {
                return CodeMirror.Pass;
            }
            const closing = textAfter.charAt(0) === cx.type;
            if (cx.align) {
                return cx.col + (closing ? 0 : 1);
            } else {
                return cx.indent + (closing ? 0 : config.indentUnit);
            }
        },

        blockCommentStart: '/*',
        blockCommentEnd: '*/',
        lineComment: support.commentSlashSlash ? '//' : support.commentHash ? '#' : '--',
        closeBrackets: '()[]{}\'\'""``'
    };
});


(CodeMirror as any).defineMIME('text/n-sql', {
    name: 'sql',
    client: set('source'),
    keywords: set(sqlKeywords),
    builtin: set(`bigint int8 bigserial serial8 bit varying varbit boolean bool box bytea character char varchar cidr circle date
     double precision float8 inet integer int int4 interval json jsonb line lseg macaddr macaddr8 money numeric decimal path pg_lsn point
     polygon real float4 smallint int2 smallserial serial2 serial serial4 text time without zone with timetz timestamp timestamptz
      tsquery tsvector txid_snapshot uuid xml`),
    atoms: set('false true null unknown'),
    functions: [
        {
            text: 'abs',
            param: '(:number)',
            description: 'The ABS() function returns the absolute value of a number.',
        }, {
            text: 'avg',
            param: '([:number])',
            description: 'The AVG() function returns the average value of a numeric column'
        }, {
            text: 'ceil',
            param: '(:number)',
            description: 'The CEIL() function returns the smallest integer value that is bigger than or equal to a number.'
        }, {
            text: 'coalesce',
            param: '(:any++)',
            description: 'The COALESCE() function returns the first non-null value in a list.'
        }, {
            text: 'concat',
            param: '(:string+)',
            description: 'Adds two or more strings together'
        }, {
            text: 'cos',
            param: '(:number)',
            description: 'The COS() function returns the cosine of a number.'
        }, {
            text: `count`,
            param: '([:number])',
            description: 'The COUNT() function returns the number of rows that matches a specified criteria.'
        }, {
            text: 'day',
            param: '(:datetime)',
            description: 'The DAY() function returns the day of the month (from 1 to 31) for a specified date.'
        }, {
            text: 'day_add',
            param: '(:datetime, :int)',
            description: 'The DAY_ADD function adds a day or serveral days interval to a date and then returns the date.'
        }, {
            text: 'day_diff',
            param: '(start: datetime, end: number)',
            description: 'The DAY_DIFF() function returns the number of days between two date values.'
        }, {
            text: 'day_sub',
            param: '(:datetime, :int)',
            description: 'The DAY_SUB function subtracts a day or serveral days interval from a date and then returns the date.'
        }, {
            text: 'decode',
            param: '(:any+++)',
            description: 'The DECODE() function is used to provide if-then-else type of logic to SQL.'
        }, {
            text: 'dense_rank',
            param: '([:number] | :number, asc | desc)',
            description: 'The DENSE_RANK() function computes the rank of a row in an ordered group of rows and returns the rank as a NUMBER'
        }, {
            text: 'floor',
            param: '(:number)',
            description: 'The FLOOR() function returns the largest integer value that is smaller than or equal to a number.'
        }, {
            text: 'format',
            param: '(:datetime, fmt: string)',
            description: 'The FORMAT() function formats a value with the specified format'
        }, {
            text: 'hour',
            param: '(:datetime)',
            description: 'The HOUR() function returns the hour of the month (from 0 to 23) for a specified date.'
        }, {
            text: 'hour_add',
            param: '(:datetime, :int)',
            description: 'The HOUR_ADD() function adds an hour or serveral hours interval to a date and then returns the date.'
        }, {
            text: 'hour_diff',
            param: '(start: datetime, end: datetime)',
            description: 'The HOUR_DIFF() returns the number of hours between two date values.'
        }, {
            text: 'hour_sub',
            param: '(:datetime, :number)',
            description: 'The HOUR_SUB() function subtracts an hour or serveral hours interval to a date and then returns the date.'
        }, {
            text: 'left',
            param: '(:string, length:int)',
            description: 'Extracts a number of characters from a string (starting from left)'
        }, {
            text: 'length',
            param: '(:string)',
            description: 'The LENGTH function returns the length of a string'
        }, {
            text: 'log',
            param: '(base: number, :number)',
            // tslint:disable-next-line:max-line-length
            description: 'The LOG() function returns the natural logarithm of a specified number, or the logarithm of the number to the specified base.'
        }, {
            text: 'log10',
            param: '(:number)',
            description: 'The LOG10() function returns the natural logarithm of a number to base-10.'
        }, {
            text: 'ltrim',
            param: '(:string, trimText: string?)',
            description: 'The LTRIM() function removes leading spaces from a string.'
        }, {
            text: 'lower',
            param: '(:string)',
            description: 'Converts a string to lower-case'
        }, {
            text: 'max',
            param: '([:number])',
            description: 'The MAX() function returns the largest value of the selected column.'
        }, {
            text: 'median',
            param: '([:number])',
            description: 'The MEDIAN() function calculates the median of a series of numbers and returns it.'
        }, {
            text: 'min',
            param: '([:number])',
            description: 'The MIN() function returns the smallest value of the selected column.'
        }, {
            text: 'minute',
            param: '(:datetime)',
            description: 'The MINUTE() function returns the minute of the hour (from 0 to 59) for a specified date.'
        }, {
            text: 'minute_add',
            param: '(:datetime, :int)',
            description: 'The MINUTE_ADD function adds an hour or serveral hours interval to a date and then returns the date.'
        }, {
            text: 'minute_diff',
            param: '(start: datetime, end: datetime)',
            description: 'The MINUTE_DIFF() returns the number of minutes between two date values.'
        }, {
            text: 'minute_sub',
            param: '(:datetime, :int)',
            description: 'The MINUTE_SUB() function subtracts a minute or serveral minutes interval to a date and then returns the date.'
        }, {
            text: 'month',
            param: '(:datetime)',
            description: 'The MONTH() function returns the month part for a specified date (a number from 1 to 12)'
        }, {
            text: 'month_add',
            param: '(:datetime)',
            description: 'The MONTH_ADD() function adds a month or serveral months interval to a date and then returns the date.'
        }, {
            text: 'month_diff',
            param: '(start: datetime, end: datetime)',
            description: 'The MONTH_DIFF() function returns the month of hours between two date values.'
        }, {
            text: 'month_sub',
            param: '(:datetime)',
            description: 'The MONTH_SUB() function Subtracts a month or serveral months interval to a date and then returns the date.'
        }, {
            text: 'now',
            param: '()',
            description: 'The NOW() function returns the current date and time.'
        }, {
            text: `nvl`,
            param: '(:any, replaceValue: any)',
            description: 'The NVL() function replace NULL value with another value',
            example: 'nvl(:any, replaceValue)'
        }, {
            text: 'percent',
            param: '([:number], p:float, asc | desc)',
            description: 'The PERCENTILE_CONT function is an inverse distribution function that assumes a continuous distribution model. '
        }, {
            text: 'percentile',
            param: '([:number], p:float)',
            description: 'The PERCENTILE_CONT function is an inverse distribution function that assumes a continuous distribution model. '
        }, {
            text: 'percentile',
            param: '([:number], p:float, asc | desc)',
            description: 'The PERCENTILE_CONT function is an inverse distribution function that assumes a continuous distribution model. '
        }, {
            text: 'percentile_cont',
            param: '([:number], p:float, asc | desc)',
            description: 'The PERCENTILE_CONT function is an inverse distribution function that assumes a continuous distribution model. '
        }, {
            text: 'percentile_dist',
            param: '([:number], p:float, asc | desc)',
            description: 'The PERCENTILE_DISC() function is an inverse distribution function that assumes a discrete distribution model.'
        }, {
            text: 'pow',
            param: '(x:number, y:number)',
            description: 'The POW() function returns the value of a number raised to the power of another number.'
        }, {
            text: 'power',
            param: '(x:number, y:number)',
            description: 'The POW() function returns the value of a number raised to the power of another number.'
        }, {
            text: 'rank',
            param: '([number]|number, asc | desc)',
            description: 'RANK calculates the rank of a value in a group of values. The return type is NUMBER.'
        }, {
            text: 'replace',
            param: '(:string, old: string, new: string)',
            description: 'The REPLACE() function replaces all occurrences of a substring within a string, with a new substring.'
        }, {
            text: 'reverse',
            param: '(:string)',
            description: 'The REVERSE() function reverses a string and returns the result.'
        }, {
            text: 'right',
            param: '(:string, length:int)',
            description: 'The RIGHT() function extracts a number of characters from a string (starting from right).'
        }, {
            text: 'round',
            param: '(:number, :decimals)',
            description: 'The ROUND() function rounds a number to a specified number of decimal places.'
        }, {
            text: 'rtrim',
            param: '(:string, trimText: string?)',
            description: 'The TRIM() function removes the space character OR other specified characters from the start or end of a string.'
        }, {
            text: `second`,
            param: '(:datetime)',
            description: 'The SECOND() function returns the seconds part of a time/datetime.'
        }, {
            text: `second_add`,
            param: '(:datetime, :int)',
            description: 'The SECOND_ADD() function adds a second or serveral seconds interval to a date and then returns the date.'
        }, {
            text: `second_diff`,
            param: '(start: datetime, end: datetime)',
            description: 'The SECOND_DIFF() function returns the number of seconds between two date values.'
        }, {
            text: `second_sub`,
            param: '(start: datetime, :int)',
            description: 'The SECOND_SUB() function subtracts a second or serveral seconds interval to a date and then returns the date.'
        }, {
            text: `sign`,
            param: '(:number)',
            description: 'The SIGN() function returns the sign of a number.',
            example: 'sign(:number)'
        },
        {
            text: `sin`,
            param: '(:number)',
            description: 'The SIN() function returns the sine of a number.'
        },
        {
            text: `sqrt`,
            param: '(:number)',
            description: 'The SQRT() function returns the square of a number.'
        },
        {
            text: `stddev`,
            param: '([:numner])',
            // tslint:disable-next-line:max-line-length
            description: 'The STDDEV() function returns the population standard deviation of expression. It returns NULL if no matching rows found.'
        },
        {
            text: `substr`,
            param: '(:string, start: int, length: int)',
            description: 'The SUBSTR() function extracts a substring from a string (starting at any position)'
        },
        {
            text: `sum`,
            param: '([:number])',
            description: 'The SUM() function returns the total sum of a numeric column.'
        },
        {
            text: 'tan',
            param: '(:number)',
            description: 'The TAN() function returns the tangent of a number.'
        },
        {
            text: `trim`,
            param: '(:string, trimText: string?)',
            description: 'Removes leading and trailing spaces (or other specified characters) from a string'
        },
        {
            text: 'upper',
            param: '(:string)',
            description: 'The UPPER() function converts a string to upper-case.'
        },
        {
            text: 'variance',
            param: '([:number])',
            description: 'The VARIANCE() function returns the population standard variance of an expression.'
        },
        {
            text: 'year',
            param: '(:datetime)',
            description: 'The YEAR() function returns the year part for a specified date.'
        },
        {
            text: 'year_add',
            param: '(:datetime, :int)',
            description: 'The YEAR_ADD() fucntion adds a year or serveral years interval to a date and then returns the date.'
        },
        {
            text: 'year_diff',
            param: '(start:datetime, end: datetime)',
            description: 'The YEAR_DIFF() function returns the number of years between two date values.'
        },
        {
            text: 'year_sub',
            param: '(:datetime, :int)',
            description: 'The YEAR_SUB function subtracts a year or serveral years interval to a date and then returns the date.'
        }
    ],
    operatorChars: /^[*+\-%<>!=&|^\/#@?~]/,
    dateSQL: set('date time timestamp'),
    support: set('ODBCdotTable decimallessFloat zerolessFloat binaryNumber hexNumber nCharCast charsetCast')
});




