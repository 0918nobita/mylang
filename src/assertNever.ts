export const assertNever = (value: never): never => {
    console.error('Unexpected value:', value);
    throw new Error('Unreachable code');
};
