# N-SQL Functions

> N-SQL has many built-in functions.

## N-SQL String Functions

Function | Description
-- | --
[CONCAT](/#/functions?id=concat) | Adds two or more strings together
[LEFT](/#/functions?id=left) | Extracts a number of characters from a string (starting from left)
[LENGTH](/#/functions?id=length) | Returns the length of a string
[LOWER](/#/functions?id=lower) | Converts a string to lower-case
[LTRIM](/#/functions?id=ltrim) | Removes leading spaces from a string
[REPLACE](/#/functions?id=replace) | Replaces all occurrences of a substring within a string, with a new substring
[REVERSE](/#/functions?id=reverse) | Reverses a string and returns the result.
[RIGHT](/#/functions?id=right) | Extracts a number of characters from a string (starting from right)
[RTRIM](/#/functions?id=rtrim) | Removes trailing spaces from a string
[SUBSTR](/#/functions?id=substr) | Extracts a substring from a string (starting at any position).
[TRIM](/#/functions?id=trim) | Removes leading and trailing spaces (or other specified characters) from a string
[UPPER](/#/functions?id=upper) | Converts a string to upper-case

## N-SQL Math/Numeric Functions

Function | Description
-- | --
[ABS](/#/functions?id=abs) | Returns the absolute value of a number
[AVG](/#/functions?id=avg) | Returns the average value of an expression
[COUNT](/#/functions?id=count) | Returns the number of records returned by a select query
[CEIL](/#/functions?id=ceil) | Returns the smallest integer value that is >= to a number
[CEILING](/#/functions?id=ceil) | This function is equal to the CEIL() function.
[COS](/#/functions?id=cos) | Returns the cosine of a number
[DENSE_RANK](/#/functions?id=dense_rank) | Returns the rank of each row within a result set partition, with no gaps in the ranking values.
[FLOOR](/#/functions?id=floor) | Returns the largest integer value that is <= to a number
[LOG](/#/functions?id=log) | Returns the natural logarithm of a number, or the logarithm of a number to a specified base
[LOG10](/#/functions?id=log10) | Returns the natural logarithm of a number to base 10
[MAX](/#/functions?id=max) | Returns the maximum value in a set of values
[MEDIAN](/#/functions?id=median) | Retuns the median of a series of numbers
[MIN](/#/functions?id=min) | Returns the minimum value in a set of values
[PERCENT](/#/functions?id=percentile_cont) | This function is equal to the PERCENTILE_CONT() function.
[PERCENTILE](/#/functions?id=percentile_cont) | This function is equal to the PERCENTILE_CONT() function.
[PERCENTILE_CONT](/#/functions?id=percentile_cont) | Returns an interpolated value in An inverse distribution function that assumes a continuous distribution model.
[PERCENTILE_DISC](/#/functions?id=percentile_disc) | Returns an element from An inverse distribution function that assumes a discrete distribution model
[POW](/#/functions?id=pow) | Returns the value of a number raised to the power of another number
[POWER](/#/functions?id=power) | Returns the value of a number raised to the power of another number
[RANK](/#/functions?id=rank) | Returns the rank of a value in a group of values
[ROUND](/#/functions?id=round) | Rounds a number to a specified number of decimal places
[SIGN](/#/functions?id=sign) | Returns the sign of a number
[SIN](/#/functions?id=sin) | Returns the sine of a number
[SQRT](/#/functions?id=sqrt) | Returns the square of a number
[STDDEV](/#/functions?id=stddev) | Returns the population standard deviation of expression.
[SUM](/#/functions?id=sum) | Calculates the sum of a set of values
[TAN](/#/functions?id=tan) | Returns the tangent of a number.
[VARIANCE](/#/functions?id=variance) |  Returns the population standard variance of an expression.

## N-SQL Date Functions

Function | Description
-- | --
[DAY](/#/functions?id=day) | Returns the day of the month (from 1 to 31) for a specified date.
[DAY_ADD](/#/functions?id=day_add) | Adds a day or serveral days interval to a date and then returns the date.
[DAY_DIFF](/#/functions?id=day_diff) | Returns the number of days between two date values.
[DAY_SUB](/#/functions?id=day_sub) | Subtracts a day or serveral days interval from a date and then returns the date.
[HOUR](/#/functions?id=hour) | Returns the hour part for a given date
[HOUR_ADD](/#/functions?id=hour_add) | Adds an hour or serveral hours interval to a date and then returns the date.
[HOUR_DIFF](/#/functions?id=hour_diff) | Returns the number of hours between two date values.
[HOUR_SUB](/#/functions?id=hour_sub) | Subtracts an hour or serveral hours interval to a date and then returns the date.
[MINUTE](/#/functions?id=minute) | Returns the minute part of a time/datetime
[MINUTE_ADD](/#/functions?id=minute_add) | Adds a minute or serveral minutes interval to a date and then returns the date.
[MINUTE_DIFF](/#/functions?id=minute_diff) | Returns the number of minutes between two date values.
[MINUTE_SUB](/#/functions?id=minute_sub) | Subtracts a minute or serveral minutes interval to a date and then returns the date.
[MONTH](/#/functions?id=month) | Returns the month part for a specified date (a number from 1 to 12)
[MONTH_ADD](/#/functions?id=month_add) | Adds a month or serveral months interval to a date and then returns the date.
[MONTH_DIFF](/#/functions?id=month_diff) | Returns the month of hours between two date values.
[MONTH_SUB](/#/functions?id=month_sub) | Subtracts a month or serveral months interval to a date and then returns the date.
[NOW](http://localhost:5000/#/functions?id=now) | Returns the current date and time
QUARTER | Returns the quarter number for a given date.
[SECOND](/#/functions?id=second) | Returns the seconds part of a time/datetime
[SECOND_ADD](/#/functions?id=second_add) | Adds a second or serveral seconds interval to a date and then returns the date.
[SECOND_DIFF](/#/functions?id=second_diff) | Returns the number of seconds between two date values.
[SECOND_SUB](/#/functions?id=second_sub) | Subtracts a second or serveral seconds interval to a date and then returns the date.
WEEK | Returns the week number for a given date
WEEK_ADD | Adds a week or serveral weeks interval to a date and then returns the date.
WEEK_DIFF | Returns the number of weeks between two date values.
WEEK_SUB | Subtracts a week or serveral weeks interval to a date and then returns the date.
[YEAR](/#/functions?id=year) | Returns the year part for a specified date
[YEAR_ADD](/#/functions?id=year_add) | Adds a year or serveral years interval to a date and then returns the date.
[YEAR_DIFF](/#/functions?id=year_diff) | Returns the number of years between two date values.
[YEAR_SUB](/#/functions?id=year_sub) | Subtracts a year or serveral years interval to a date and then returns the date.

## N-SQL Advanced Functions

Function | Description
-- | --
[COALESCE](/#/functions?id=coalesce) | Return the first non-null value in a list
[DECODE](/#/functions?id=decode) | Used to provide if-then-else type of logic to SQL.
IFNULL | Returns a specified value if the expression is NULL.
NULLIF | Returns NULL if two expressions are equal, otherwise it returns the first expression.
[NVL](/#/functions?id=nvl) | Replace NULL value with another value
[FORMAT](/#/functions?id=format)| Returns a value with the specified format

## Details Of Functions

### ABS

> The ABS() function returns the absolute value of a number.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax
  
  `abs(:number)`

### AVG

> The AVG() function returns the average value of a numeric column

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax
  
  `avg([:number])`

### CEIL

> The CEIL() function returns the smallest integer value that is bigger than or equal to a number.

**Note**: This function is equal to the CEILING() function.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$
- Syntax
  
  1. `ceil(:number)`
  2. `ceiling(:number)`

### COALESCE

> The COALESCE() function returns the first non-null value in a list.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax
  
  `coalesce(:any++)`

### CONCAT

> Adds two or more strings together

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax
  
  `concat(:string+)`

### COS

> The COS() function returns the cosine of a number.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax
  
  `cos(:number)`

### COUNT

> The COUNT() function returns the number of rows that matches a specified criteria.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax
  
  `count([:number])`

### DAY

> The DAY() function returns the day of the month (from 1 to 31) for a specified date.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite |

- Syntax
  
  `day(:datetime)`

### DAY_ADD

> The DAY_ADD function adds a day or serveral days interval to a date and then returns the date.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax
  
  `day_add(:datetime, :int)`

### DAY_DIFF

> The DAY_DIFF() function returns the number of days between two date values.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax
  
  `day_diff(start: datetime, end: datetime)`

### DAY_SUB

> The DAY_SUB function subtracts a day or serveral days interval from a date and then returns the date.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax
  
  `day_sub(:datetime, :int)`

### DECODE

> The DECODE() function is used to provide if-then-else type of logic to SQL.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax
  
  `decode(:any, :any, :any+)`

### DENSE_RANK

> DENSE_RANK computes the rank of a row in an ordered group of rows and returns the rank as a NUMBER. The ranks are consecutive integers beginning with 1. The largest rank value is the number of unique values returned by the query. Rank values are not skipped in the event of ties. Rows with equal values for the ranking criteria receive the same rank. This function is useful for top-N and bottom-N reporting.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite |

- Syntax

  `dense_rank([:number] | :number, ASC | DESC)`

### FLOOR

> The FLOOR() function returns the largest integer value that is smaller than or equal to a number.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax

  `floor(:number)`

### FORMAT

> The FORMAT() function formats a value with the specified format

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Template Patterns for Date/Time Formatting
  Pattern | Description
  -- | --
  HH | hour of day (01-12)
  HH12 | hour of day (01-12)
  HH24 | hour of day (00-23)
  MI | minute (00-59)
  SS | second (00-59)
  YYYY | year (4 or more digits)
  YY | last 2 digits of year
  MONTH | full upper case month name (blank-padded to 9 chars)
  Month | full capitalized month name (blank-padded to 9 chars)
  month | full lower case month name (blank-padded to 9 chars)
  MON | abbreviated upper case month name (3 chars in English, localized lengths vary)
  Mon | abbreviated capitalized month name (3 chars in English, localized lengths vary)
  mon | abbreviated lower case month name (3 chars in English, localized lengths vary)
  MM | month number (01-12)
  DAY | full upper case day name (blank-padded to 9 chars)
  Day | full capitalized day name (blank-padded to 9 chars)
  day | full lower case day name (blank-padded to 9 chars)
  DY | abbreviated upper case day name (3 chars in English, localized lengths vary)
  Dy | abbreviated capitalized day name (3 chars in English, localized lengths vary)
  dy | abbreviated lower case day name (3 chars in English, localized lengths vary)
  DD | day of month (01-31)

- Syntax

  `format(:datetime, fmt:string)`

### HOUR

> The HOUR() function returns the hour of the month (from 0 to 23) for a specified date.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite |

- Syntax
  
  `hour(:datetime)`

### HOUR_ADD

> The HOUR_ADD() function adds an hour or serveral hours interval to a date and then returns the date.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax
  
  `hour_add(:datetime, :int)`

### HOUR_DIFF

> The HOUR_DIFF() returns the number of hours between two date values.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax
  
  `hour_diff(start: datetime, end: datetime)`

### HOUR_SUB

> The HOUR_SUB() function subtracts an hour or serveral hours interval to a date and then returns the date.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax
  
  `hour_sub(:datetime, :int)`

### LEFT

> Extracts a number of characters from a string (starting from left)

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax
  
  `left(:string, length:int)`

### LENGTH

> The LENGTH function returns the length of a string

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax
  
  `length(:string)`

### LOG

> The LOG() function returns the natural logarithm of a specified number, or the logarithm of the number to the specified base.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite |

- Syntax
  
 1. `log(:number)`
 2. `log(base: number, :number)`

### LOG10

> The LOG10() function returns the natural logarithm of a number to base-10.

**Note**: See also the [LOG()](/#/functions?id=log) function.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{Orange} {\checkmark}$
  Oracle | $\color{Orange} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite |

- Syntax
  
   `log10(:number)`

### LTRIM

> The LTRIM() function removes leading spaces from a string.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | Only Support trim whitespace
  SQlite | $\color{green} {\checkmark}$
- Syntax

  1. `ltrim(:string, trimText: string?)`
  2. `trim_start(:string, trimText: string?)`

### LOWER

> Converts a string to lower-case

**Note**: Also look at the [UPPER()](/#/functions?id=upper) function.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax
  
  `lower(:string)`

### MAX

> The MAX() function returns the largest value of the selected column.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax
  
  `max([:number])`

### MEDIAN

> The MEDIAN() function calculates the median of a series of numbers and returns it.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL |
  Sql Server | $\color{Orange} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax
  
  `median([:number])`

### MIN

> The MIN() function returns the smallest value of the selected column.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax
  
  `min([:number])`

### MINUTE

> The MINUTE() function returns the minute of the hour (from 0 to 59) for a specified date.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite |

- Syntax
  
  `minute(:datetime)`

### MINUTE_ADD

> The MINUTE_ADD function adds an hour or serveral hours interval to a date and then returns the date.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax
  
  `minute_add(:datetime, :int)`

### MINUTE_DIFF

> The MINUTE_DIFF() returns the number of minutes between two date values.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax
  
  `minute_diff(start: datetime, end: datetime)`

### MINUTE_SUB

> The MINUTE_SUB() function subtracts a minute or serveral minutes interval to a date and then returns the date.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax
  
  `minute_sub(:datetime, :int)`

### MONTH

> The MONTH() function returns the month part for a specified date (a number from 1 to 12)

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite |

- Syntax
  
  `month(:datetime)`

### MONTH_ADD

> The MONTH_ADD() function adds a month or serveral months interval to a date and then returns the date.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax
  
  `month_add(:datetime)`

### MONTH_DIFF

> The MONTH_DIFF() function returns the month of hours between two date values.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax
  
  `month_diff(start: datetime, end: datetime)`

### MONTH_SUB

> The MONTH_SUB() function Subtracts a month or serveral months interval to a date and then returns the date.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax
  
  `month_sub(:datetime)`

### NOW

> The NOW() function returns the current date and time.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax

  `now()`

### NVL

> The NVL() function replace NULL value with another value

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax

  `nvl(:any, replace:any)`

### PERCENTILE_CONT

> The PERCENTILE_CONT function is an inverse distribution function that assumes a continuous distribution model. It takes a percentile value and a sort specification, and returns an interpolated value that would fall into that percentile value with respect to the sort specification. Nulls are ignored in the calculation.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL |
  Sql Server | $\color{orange} {\checkmark}$
  SQlite |

- Syntax

  1. `percent([:number], p:float)`
  2. `percent([:number], p:float, asc | desc)`
  3. `percentile([:number], p:float)`
  4. `percentile([:number], p:float, asc | desc)`
  5. `percentile_cont([:number], p:float)`
  6. `percentile_cont([:number], p:float, asc | desc)`

### PERCENTILE_DISC

> The PERCENTILE_DISC() function is an inverse distribution function that assumes a discrete distribution model. It takes a percentile value and a sort specification and returns an element from the set. Nulls are ignored in the calculation.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL |
  Sql Server | $\color{orange} {\checkmark}$
  SQlite |

- Syntax

  1. `percentile_dist([:number], p:float)`
  2. `percentile_dist([:number], p:float, asc | desc)`

### POW

> The POW() function returns the value of a number raised to the power of another number.

**Note**: This function is equal to the [POWER()](/#/functions?id=power) function.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax

  `pow(x:number, y:number)`

### POWER

> The POWER() function returns the value of a number raised to the power of another number.

**Note**: This function is equal to the [POW()](/#/functions?id=pow) function.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax

  `power(x:number, y:number)`

### RANK

> RANK calculates the rank of a value in a group of values. The return type is NUMBER.
- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite |

- Syntax

  `rank([number]|number, ASC | DESC)`

### REPLACE

> The REPLACE() function replaces all occurrences of a substring within a string, with a new substring.

  Note: The search is case-insensitive.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax

  `replace(:string, old: string, new: string)`

### REVERSE

> The REVERSE() function reverses a string and returns the result.

- Support
   Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax

  `reverse(:string)`

### RIGHT

> The RIGHT() function extracts a number of characters from a string (starting from right).

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax
  
  `right(:string, length:int)`

### ROUND

> The ROUND() function rounds a number to a specified number of decimal places.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax
  
  `round(:number, :decimals)`

### RTRIM

> The TRIM() function removes the space character OR other specified characters from the start or end of a string.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | Only Support trim whitespace
  SQlite | $\color{green} {\checkmark}$

- Syntax

  1. `rtrim(:string, trimText: string?)`
  2. `trim_end(:string, trimText: string?)`

### SECOND

> The SECOND() function returns the seconds part of a time/datetime.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite |

- Syntax

   `second(:datetime)`

### SECOND_ADD

> The SECOND_ADD() function adds a second or serveral seconds interval to a date and then returns the date.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax

   `second_add(:datetime)`

### SECOND_DIFF

> The SECOND_DIFF() function returns the number of seconds between two date values.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax

   `second_diff(start: datetime, end: datetime)`

### SECOND_SUB

> The SECOND_SUB() function subtracts a second or serveral seconds interval to a date and then returns the date.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

- Syntax

   `second_sub(:datetime)`

### SIGN

> The SIGN() function returns the sign of a number.
- Note

  This function will return one of the following:
  - If number > 0, it returns 1
  - If number = 0, it returns 0
  - If number < 0, it returns -1

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax

   `sign(:number)`

### SIN

> The SIN() function returns the sine of a number.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax

   `sin(:number)`

### SQRT

> The SQRT() function returns the square of a number.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax

   `sqrt(:number)`

### STDDEV

> The STDDEV() function returns the population standard deviation of expression. It returns NULL if no matching rows found.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite |

- Syntax

   1. `stddev([:number])`
   2. `stddev(all | distinct [:number])`

### SUBSTR

> The SUBSTR() function extracts a substring from a string (starting at any position)

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax
  
  `substr(:string, start: int, length:int)`

### SUM

> The SUM() function returns the total sum of a numeric column.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax

   `sum([:number])`

### TAN

> The TAN() function returns the tangent of a number.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

  - Syntax

   `tan(:number)`

### TRIM

> Removes leading and trailing spaces (or other specified characters) from a string

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | Only Support trim whitespace
  SQlite | $\color{green} {\checkmark}$

- Syntax

   `trim(:string, trimText: string?)`

### UPPER

> The UPPER() function converts a string to upper-case.

**Note**: Also look at the [LOWER()](/#/functions?id=lower) function.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax

  `upper(:string)`

### VARIANCE

> The VARIANCE() function returns the population standard variance of an expression.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

- Syntax

  `variance([:number])`

### YEAR

> The YEAR() function returns the year part for a specified date.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{Orange} {\checkmark}$
  Oracle | $\color{Orange} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite |

- Syntax

  `year(:datetime)`

### YEAR_ADD

> The YEAR_ADD() fucntion adds a year or serveral years interval to a date and then returns the date.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{Orange} {\checkmark}$
  Oracle | $\color{Orange} {\checkmark}$
  MySQL | $\color{Orange} {\checkmark}$
  Sql Server | $\color{Orange} {\checkmark}$
  SQlite | $\color{Orange} {\checkmark}$

- Syntax

  `year_add(:datetime, :int)`

### YEAR_DIFF

> The YEAR_DIFF() function returns the number of years between two date values.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{Orange} {\checkmark}$
  Oracle | $\color{Orange} {\checkmark}$
  MySQL | $\color{Orange} {\checkmark}$
  Sql Server | $\color{Orange} {\checkmark}$
  SQlite | $\color{Orange} {\checkmark}$

- Syntax

  `year_diff(start:datetime, end: datetime)`

### YEAR_SUB

> The YEAR_SUB() function subtracts a year or serveral years interval to a date and then returns the date.

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{Orange} {\checkmark}$
  Oracle | $\color{Orange} {\checkmark}$
  MySQL | $\color{Orange} {\checkmark}$
  Sql Server | $\color{Orange} {\checkmark}$
  SQlite | $\color{Orange} {\checkmark}$

- Syntax

  `year_sub(:datetime, :int)`