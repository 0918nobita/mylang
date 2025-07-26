import { expect, test } from 'bun:test';

import { withParentheses } from './withParentheses';

test('withParentheses', () => {
    // (2 + 3) * 7
    expect(
        withParentheses(
            {
                content: '2 + 3',
                precedence: 12,
            },
            {
                precedence: 13,
                associativity: 'left',
                position: 'left',
            }
        )
    ).toBe('(2 + 3)');

    // 3 * 4 + 5
    expect(
        withParentheses(
            {
                content: '3 * 4',
                precedence: 13,
            },
            {
                precedence: 12,
                associativity: 'left',
                position: 'left',
            }
        )
    ).toBe('3 * 4');

    // 2 * 3 * 4
    expect(
        withParentheses(
            {
                content: '2 * 3',
                precedence: 13,
            },
            {
                precedence: 13,
                associativity: 'left',
                position: 'left',
            }
        )
    ).toBe('2 * 3');

    // 2 * (3 * 4)
    expect(
        withParentheses(
            {
                content: '3 * 4',
                precedence: 13,
            },
            {
                precedence: 13,
                associativity: 'left',
                position: 'right',
            }
        )
    ).toBe('(3 * 4)');
});
