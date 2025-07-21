import * as ast from './ast';
import { assertNever } from './util';

type Context = {
    precedence: number;
    associativity: 'left' | 'right' | null;
    position: 'left' | 'right' | null;
};

const getPrecedence = (node: ast.Expression): number => {
    if (node.type === 'nullLiteral') {
        return 20;
    }

    if (node.type === 'booleanLiteral') {
        return 20;
    }

    if (node.type === 'numberLiteral') {
        return 20;
    }

    if (node.type === 'stringLiteral') {
        return 20;
    }

    if (node.type === 'identifier') {
        return 20;
    }

    if (node.type === 'callExpression') {
        return 18;
    }

    if (node.type === 'memberExpression') {
        return 18;
    }

    return assertNever(node);
};

const needsParentheses = (childNode: ast.Expression, ctx: Context): boolean => {
    const childPrecedence = getPrecedence(childNode);
    const parentPrecedence = ctx.precedence;

    if (childPrecedence < parentPrecedence) {
        return true;
    }

    if (childPrecedence > parentPrecedence) {
        return false;
    }

    // childPrecedence === parentPrecedence
    if (ctx.associativity === 'left' && ctx.position === 'right') {
        return true;
    }

    if (ctx.associativity === 'right' && ctx.position === 'left') {
        return true;
    }

    return false;
};

export const generate = (jsAst: ast.Expression): string => {
    if (jsAst.type === 'nullLiteral') {
        return 'null';
    }

    if (jsAst.type === 'booleanLiteral') {
        return jsAst.value ? 'true' : 'false';
    }

    if (jsAst.type === 'numberLiteral') {
        return String(jsAst.value);
    }

    if (jsAst.type === 'stringLiteral') {
        return '"' + jsAst.value + '"';
    }

    if (jsAst.type === 'identifier') {
        return jsAst.name;
    }

    if (jsAst.type === 'callExpression') {
        const callee = jsAst.callee;
        const args = jsAst.arguments;
        const optional = jsAst.optional;

        let combined = '(' + generate(callee) + ')';

        if (optional) {
            combined += '?.';
        }

        combined += '(';

        for (let i = 0; i < args.length; i++) {
            combined += generate(args[i]!);
            if (i < args.length - 1) {
                combined += ', ';
            }
        }

        combined += ')';

        return combined;
    }

    if (jsAst.type === 'memberExpression') {
        const object = jsAst.object;
        const property = jsAst.property;
        const computed = jsAst.computed;
        const optional = jsAst.optional;

        let combined = generate(object);

        if (optional) {
            combined += '?.';
        }

        if (computed) {
            combined += '[' + generate(property) + ']';
        } else {
            combined += generate(property);
        }

        return combined;
    }

    return assertNever(jsAst);
};
