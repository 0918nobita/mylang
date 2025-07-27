export type NullLiteral = Readonly<{
    type: 'nullLiteral';
}>;

export type BooleanLiteral = Readonly<{
    type: 'booleanLiteral';
    value: boolean;
}>;

export type NumberLiteral = Readonly<{
    type: 'numberLiteral';
    value: number;
}>;

export type StringLiteral = Readonly<{
    type: 'stringLiteral';
    value: string;
}>;

export type Literal =
    | NullLiteral
    | BooleanLiteral
    | NumberLiteral
    | StringLiteral;

export type Identifier = Readonly<{
    type: 'identifier';
    name: string;
}>;

export type BinaryExpression = Readonly<{
    type: 'binaryExpression';
    operator: '+' | '*';
    left: Expression;
    right: Expression;
}>;

export type CallExpression = Readonly<{
    type: 'callExpression';
    callee: Expression;
    arguments: Expression[];
    optional: boolean;
}>;

export type MemberExpression = Readonly<{
    type: 'memberExpression';
    object: Expression;
    property: Expression;
    computed: boolean;
    optional: boolean;
}>;

export type Expression =
    | Literal
    | Identifier
    | BinaryExpression
    | CallExpression
    | MemberExpression;
