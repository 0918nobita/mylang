export type NullLiteral = {
    type: 'nullLiteral';
};

export type BooleanLiteral = {
    type: 'booleanLiteral';
    value: boolean;
};

export type NumberLiteral = {
    type: 'numberLiteral';
    value: number;
};

export type StringLiteral = {
    type: 'stringLiteral';
    value: string;
};

export type Literal =
    | NullLiteral
    | BooleanLiteral
    | NumberLiteral
    | StringLiteral;

export type Identifier = {
    type: 'identifier';
    name: string;
};

export type CallExpression = {
    type: 'callExpression';
    callee: Expression;
    arguments: Expression[];
    optional: boolean;
};

export type MemberExpression = {
    type: 'memberExpression';
    object: Expression;
    property: Expression;
    computed: boolean;
    optional: boolean;
};

export type Expression =
    | Literal
    | Identifier
    | CallExpression
    | MemberExpression;
