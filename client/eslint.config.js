import vuePlugin from 'eslint-plugin-vue';
import vueParser from 'vue-eslint-parser';
import tsParser from '@typescript-eslint/parser';

export default [
  ...vuePlugin.configs['flat/recommended'],
  {
    languageOptions: {
      parser: vueParser,
      parserOptions: {
        parser: tsParser,
        sourceType: 'module',
      },
    },
    plugins: { vueParser },
    files: ['src/**/*.js', 'src/**/*.ts', 'src/**/*.vue'],
    rules: {
      'semi': [
        'error',
        'always',
      ],
      'object-curly-spacing': ['warn', 'always'],
      'comma-dangle': [
        'error',
        'always-multiline',
      ],
      'no-trailing-spaces': 'error',
      'quotes': [
        'warn',
        'single',
      ],
      'prefer-template': 'warn',
      'vue/multi-word-component-names': 'off',
      'vue/html-closing-bracket-spacing': 'off',
      'vue/html-self-closing': 'off',
      'vue/first-attribute-linebreak': 'off',
      'vue/max-attributes-per-line': 'off',
      'vue/html-closing-bracket-newline': 'off',
      'vue/no-dupe-keys': 'off',
      'vue/no-template-shadow': 'off',
      'vue/v-on-event-hyphenation': 'off',
      'vue/singleline-html-element-content-newline': 'off',
      'indent': [
        'error',
        2,
      ],
    },
  },
];
