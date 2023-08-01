#### "Висячая" запятая.

Например, следующий код выполнится с ошибкой:

```js
JSON.parse('[1, 2, 3, 4,]');
JSON.parse('{"foo": 1,}');
// SyntaxError JSON.parse: unexpected character
// at line 1 column 14 of the JSON data
```

#### Названия свойств должны быть указаны в двойных кавычках.

```js
JSON.parse("{'foo': 1}");
// SyntaxError: JSON.parse: expected property name or '}'
// at line 1 column 2 of the JSON data
```

#### "Ведущий" нуль или знаки десятичных дробей.

```js
JSON.parse('{"foo": 01}');
// SyntaxError: JSON.parse: expected ',' or '}' after property value
// in object at line 1 column 2 of the JSON data

JSON.parse('{"foo": 1.}');
// SyntaxError: JSON.parse: unterminated fractional number
// at line 1 column 2 of the JSON data
```

Благодаря модулю @tutu/stringify можно кодировать данные в строку и декодировать обратно без потерь. То есть, без какого-либо преобразования. Например, какие проблемы могут быть решены:

#### Преобразование `undefined`:

```js
JSON.parse(JSON.stringify({ test: undefined }));
// Вернёт {}
```

#### Преобразование NaN

```js
JSON.parse(JSON.stringify({ foo: NaN }));
// Вернёт "{\"foo\":null}"
```

#### Замена слешей при экранировании:

```js
JSON.parse(JSON.stringify({ 'fo\\"bar\\"r': 'test' }));
// Вернёт { 'fo\"bar\"r': 'test' }
```

#### Потеря "инстанса" объекта Date при декодировании:

```js
const res = JSON.parse(JSON.stringify({ test: new Date() })); // { test: '2022-08-16T13:04:28.698Z' }
res['test'] instanceof Date; // false

new Date() instanceof Date; // true
```

#### Отсутствие возможности кодировать функции:

```js
const foo = { f: () => 'test' };
JSON.stringify(foo);

// Вернёт {}
```

Ещё немного о синтаксических ограничениях JSON :)

```
SyntaxError: JSON.parse: unterminated string literal
SyntaxError: JSON.parse: bad control character in string literal
SyntaxError: JSON.parse: bad character in string literal
SyntaxError: JSON.parse: bad Unicode escape
SyntaxError: JSON.parse: bad escape character
SyntaxError: JSON.parse: unterminated string
SyntaxError: JSON.parse: no number after minus sign
SyntaxError: JSON.parse: unexpected non-digit
SyntaxError: JSON.parse: missing digits after decimal point
SyntaxError: JSON.parse: unterminated fractional number
SyntaxError: JSON.parse: missing digits after exponent indicator
SyntaxError: JSON.parse: missing digits after exponent sign
SyntaxError: JSON.parse: exponent part is missing a number
SyntaxError: JSON.parse: unexpected end of data
SyntaxError: JSON.parse: unexpected keyword
SyntaxError: JSON.parse: unexpected character
SyntaxError: JSON.parse: end of data while reading object contents
SyntaxError: JSON.parse: end of data when property name was expected
SyntaxError: JSON.parse: expected double-quoted property name
SyntaxError: JSON.parse: end of data after property name when ':' was expected
SyntaxError: JSON.parse: property names must be double-quoted strings
SyntaxError: JSON.parse: unexpected character
SyntaxError: JSON.parse: unexpected non-whitespace character after JSON data
```
