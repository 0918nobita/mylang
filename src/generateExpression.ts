import type * as jsAst from './jsAst';

import { assertNever } from './assertNever';
import { withParentheses } from './withParentheses';

export const generateExpression = (
    expression: jsAst.Expression
): { content: string; precedence: number } => {
    if (expression.type === 'nullLiteral') {
        return { content: 'null', precedence: 20 };
    }

    if (expression.type === 'booleanLiteral') {
        return { content: String(expression.value), precedence: 20 };
    }

    if (expression.type === 'numberLiteral') {
        return { content: String(expression.value), precedence: 20 };
    }

    if (expression.type === 'stringLiteral') {
        return { content: `"${expression.value}"`, precedence: 20 };
    }

    if (expression.type === 'identifier') {
        return { content: expression.name, precedence: 20 };
    }

    if (expression.type === 'callExpression') {
        const callee = generateExpression(expression.callee);

        const calleeWithParen = withParentheses(callee, {
            precedence: 18,
            associativity: 'left',
            position: null,
        });

        let combined = calleeWithParen;

        if (expression.optional) {
            combined += '?.';
        }

        combined += '(';

        const args = expression.arguments;

        for (let i = 0; i < args.length; i++) {
            combined += generateExpression(args[i]!).content;

            if (i < args.length - 1) {
                combined += ',';
            }
        }

        combined += ')';

        return { content: combined, precedence: 18 };
    }

    if (expression.type === 'memberExpression') {
        const object = generateExpression(expression.object);

        let combined = withParentheses(object, {
            precedence: 18,
            associativity: 'left',
            position: null,
        });

        if (expression.optional) {
            combined += '?.';
        }

        const { property } = expression;

        if (expression.computed) {
            combined += `[${generateExpression(property).content}]`;
        } else {
            if (!expression.optional) {
                combined += `.`;
            }

            combined += generateExpression(property).content;
        }

        return { content: combined, precedence: 18 };
    }

    if (expression.type === 'binaryExpression') {
        const precedence = (() => {
            if (expression.operator === '+') return 12;
            if (expression.operator === '*') return 13;
            return assertNever(expression.operator);
        })();

        const associativity = 'left';

        const left = generateExpression(expression.left);
        const leftWithParen = withParentheses(left, {
            precedence,
            associativity,
            position: 'left',
        });

        const right = generateExpression(expression.right);
        const rightWithParen = withParentheses(right, {
            precedence,
            associativity,
            position: 'right',
        });

        return {
            content: `${leftWithParen}${expression.operator}${rightWithParen}`,
            precedence,
        };
    }

    return assertNever(expression);
};
