import type * as jsAst from './jsAst';

export const compile = (source: string): jsAst.Expression => {
    return {
        type: 'callExpression',
        callee: {
            type: 'memberExpression',
            object: { type: 'identifier', name: 'console' },
            property: { type: 'identifier', name: 'log' },
            computed: false,
            optional: false,
        },
        arguments: [{ type: 'stringLiteral', value: source }],
        optional: false,
    };
};
