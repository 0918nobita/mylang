module.exports = {
    semi: true,
    overrides: [
        {
            files: ['*.js', '*.ts'],
            options: {
                singleQuote: true,
                trailingComma: 'es5',
                tabWidth: 4,
            },
        },
        {
            files: ['*.json'],
            options: {
                tabWidth: 2,
            },
        },
    ],
};
