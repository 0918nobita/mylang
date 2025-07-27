import { expect, test } from 'bun:test';

import { compile } from './compiler';

test('ソースコードを JS の AST にコンパイルする', () => {
    expect(compile('Hello, world!')).toStrictEqual({
        type: 'callExpression',
        callee: {
            type: 'memberExpression',
            object: { type: 'identifier', name: 'console' },
            property: { type: 'identifier', name: 'log' },
            computed: false,
            optional: false,
        },
        arguments: [{ type: 'stringLiteral', value: 'Hello, world!' }],
        optional: false,
    });
});
