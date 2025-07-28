export const parse = (source: string) => {
    const segmenter = new Intl.Segmenter();

    type Acc = {
        currentPos: { line: number; column: number };
        chars: Array<{ c: string; pos: { line: number; column: number } }>;
    };

    const { chars } = Array.from(segmenter.segment(source))
        .map((segment) => segment.segment)
        .reduce<Acc>(
            ({ currentPos, chars }, item) => {
                if (item === '\n') {
                    return {
                        currentPos: { line: currentPos.line + 1, column: 0 },
                        chars: [...chars, { c: '\n', pos: currentPos }],
                    };
                }

                return {
                    currentPos: {
                        line: currentPos.line,
                        column: currentPos.column + 1,
                    },
                    chars: [...chars, { c: item, pos: currentPos }],
                };
            },
            { currentPos: { line: 0, column: 0 }, chars: [] }
        );

    console.log(chars);
};
