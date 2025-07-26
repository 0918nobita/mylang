import { expect, test } from 'bun:test';

import { generateExpression } from './generateExpression';

test('null リテラルのコードを生成する', () => {
    expect(generateExpression({ type: 'nullLiteral' })).toStrictEqual({
        content: 'null',
        precedence: 20,
    });
});

test('boolean リテラルのコードを生成する', () => {
    expect(
        generateExpression({ type: 'booleanLiteral', value: true })
    ).toStrictEqual({
        content: 'true',
        precedence: 20,
    });

    expect(
        generateExpression({ type: 'booleanLiteral', value: false })
    ).toStrictEqual({
        content: 'false',
        precedence: 20,
    });
});

test('演算子の優先順位に基づいて必要最低限の括弧を挿入する', () => {
    expect(
        generateExpression({
            type: 'binaryExpression',
            operator: '+',
            left: {
                type: 'binaryExpression',
                operator: '*',
                left: { type: 'numberLiteral', value: 2 },
                right: { type: 'numberLiteral', value: 3 },
            },
            right: { type: 'numberLiteral', value: 4 },
        })
    ).toStrictEqual({ content: '2*3+4', precedence: 12 });

    expect(
        generateExpression({
            type: 'binaryExpression',
            operator: '*',
            left: {
                type: 'binaryExpression',
                operator: '+',
                left: { type: 'numberLiteral', value: 2 },
                right: { type: 'numberLiteral', value: 3 },
            },
            right: { type: 'numberLiteral', value: 4 },
        })
    ).toStrictEqual({ content: '(2+3)*4', precedence: 13 });
});

test('演算子の結合性に基づいて必要最低限の括弧を挿入する', () => {
    expect(
        generateExpression({
            type: 'binaryExpression',
            operator: '+',
            left: { type: 'numberLiteral', value: 2 },
            right: {
                type: 'binaryExpression',
                operator: '+',
                left: { type: 'numberLiteral', value: 3 },
                right: { type: 'numberLiteral', value: 4 },
            },
        })
    ).toStrictEqual({ content: '2+(3+4)', precedence: 12 });

    expect(
        generateExpression({
            type: 'binaryExpression',
            operator: '*',
            left: { type: 'numberLiteral', value: 2 },
            right: {
                type: 'binaryExpression',
                operator: '*',
                left: { type: 'numberLiteral', value: 3 },
                right: { type: 'numberLiteral', value: 4 },
            },
        })
    ).toStrictEqual({ content: '2*(3*4)', precedence: 13 });
});

test('関数呼び出しのコードを生成する', () => {
    expect(
        generateExpression({
            type: 'callExpression',
            callee: { type: 'identifier', name: 'print' },
            arguments: [],
            optional: false,
        })
    ).toStrictEqual({
        content: 'print()',
        precedence: 18,
    });

    expect(
        generateExpression({
            type: 'callExpression',
            callee: { type: 'identifier', name: 'print' },
            arguments: [],
            optional: true,
        })
    ).toStrictEqual({
        content: 'print?.()',
        precedence: 18,
    });

    expect(
        generateExpression({
            type: 'callExpression',
            callee: {
                type: 'callExpression',
                callee: { type: 'identifier', name: 'highOrderFunc' },
                arguments: [
                    { type: 'identifier', name: 'arg1' },
                    { type: 'identifier', name: 'arg2' },
                ],
                optional: false,
            },
            arguments: [{ type: 'identifier', name: 'arg3' }],
            optional: false,
        })
    ).toStrictEqual({
        content: 'highOrderFunc(arg1,arg2)(arg3)',
        precedence: 18,
    });
});

test('メンバアクセスのコードを生成する', () => {
    expect(
        generateExpression({
            type: 'memberExpression',
            object: { type: 'identifier', name: 'obj' },
            property: { type: 'identifier', name: 'prop' },
            computed: false,
            optional: false,
        })
    ).toStrictEqual({ content: 'obj.prop', precedence: 18 });

    expect(
        generateExpression({
            type: 'memberExpression',
            object: { type: 'identifier', name: 'console' },
            property: { type: 'identifier', name: 'log' },
            computed: false,
            optional: true,
        })
    ).toStrictEqual({ content: 'console?.log', precedence: 18 });

    expect(
        generateExpression({
            type: 'memberExpression',
            object: { type: 'identifier', name: 'console' },
            property: { type: 'stringLiteral', value: 'log' },
            computed: true,
            optional: false,
        })
    ).toStrictEqual({ content: 'console["log"]', precedence: 18 });

    expect(
        generateExpression({
            type: 'memberExpression',
            object: { type: 'identifier', name: 'console' },
            property: { type: 'stringLiteral', value: 'log' },
            computed: true,
            optional: true,
        })
    ).toStrictEqual({ content: 'console?.["log"]', precedence: 18 });
});
