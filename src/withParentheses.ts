export type Context = Readonly<{
    precedence: number;
    associativity: 'left' | 'right' | null;
    position: 'left' | 'right' | null;
}>;

export const withParentheses = (
    child: Readonly<{ content: string; precedence: number }>,
    context: Context
): string => {
    const needsParentheses = (() => {
        if (child.precedence < context.precedence) {
            return true;
        }

        if (child.precedence > context.precedence) {
            return false;
        }

        if (context.associativity === 'left' && context.position === 'right') {
            return true;
        }

        if (context.associativity === 'right' && context.position === 'left') {
            return true;
        }

        return false;
    })();

    return needsParentheses ? `(${child.content})` : child.content;
};
