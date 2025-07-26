import { generate } from 'astring';
import type { Program } from 'estree';

const program: Program = {
    type: 'Program',
    sourceType: 'module',
    body: [
        {
            type: 'ExpressionStatement',
            expression: {
                type: 'CallExpression',
                callee: {
                    type: 'MemberExpression',
                    object: {
                        type: 'Identifier',
                        name: 'console',
                    },
                    property: {
                        type: 'Literal',
                        value: 'log',
                    },
                    computed: true,
                    optional: true,
                },
                arguments: [
                    {
                        type: 'BinaryExpression',
                        operator: '+',
                        left: {
                            type: 'Literal',
                            value: 3,
                        },
                        right: {
                            type: 'Literal',
                            value: 4,
                        },
                    },
                ],
                optional: false,
            },
        },
    ],
};

console.log(generate(program));
