module.exports = {
  // 基本設定
  printWidth: 100,
  tabWidth: 2,
  useTabs: false,
  semi: true,
  singleQuote: true,
  quoteProps: 'as-needed',

  // 末尾カンマ（ES5: オブジェクト・配列のみ）
  trailingComma: 'es5',

  // スペース
  bracketSpacing: true,
  arrowParens: 'always',

  // 改行
  endOfLine: 'lf',

  // TypeScript
  parser: 'typescript',

  // ファイル別設定
  overrides: [
    {
      files: '*.json',
      options: {
        parser: 'json',
      },
    },
    {
      files: '*.md',
      options: {
        parser: 'markdown',
        proseWrap: 'preserve',
      },
    },
    {
      files: '*.yml',
      options: {
        parser: 'yaml',
      },
    },
  ],
};
