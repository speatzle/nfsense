import { defineConfig } from 'vite';
import { fileURLToPath, URL } from 'node:url';

import VueDevTools from 'vite-plugin-vue-devtools';
import VueMacros from 'vue-macros/vite';
import Vue from '@vitejs/plugin-vue';
import AutoImport from 'unplugin-auto-import/vite';
import Components from 'unplugin-vue-components/vite';
import Icons from 'unplugin-icons/vite';
import IconsResolver from 'unplugin-icons/resolver';
import Pages from 'vite-plugin-pages';

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    VueDevTools(),
    VueMacros({
      reactivityTransform: true,
      plugins: {
        vue: Vue(),
      },
    }),
    AutoImport({
      include: [/\.[jt]sx?$/, /\.vue$/, /\.vue\?vue/, /\.md$/],
      imports: [
        'vue',
        'vue-router',
        'vue-i18n',
        'vue/macros',
        '@vueuse/core',
        '@vueuse/head',
        {
          from: 'vue-toast-notification',
          imports: ['useToast'],
        },
      ],
      dts: 'src/generated/auto-imports.d.ts',
      dtsMode: 'overwrite',
      dirs: ['src/composables'],
      vueTemplate: true,
    }),
    Components({
      extensions: ['vue'],
      include: [/\.vue$/, /\.vue\?vue/],
      dts: 'src/generated/components.d.ts',
      resolvers: [
        IconsResolver(),
        (componentName: string) => {
          if (componentName === 'FocusTrap') return { name: 'FocusTrap', from: 'focus-trap-vue' };
        },
      ],
      types: [
        {
          from: 'focus-trap-vue',
          names: ['FocusTrap'],
        },
      ],
    }),
    Icons({}),
    Pages({
      extensions: ['vue', 'md'],
    }),
  ],

  // Easy Imports (Keep in sync with tsconfig.json!)
  resolve: {
    alias: {
      '~': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
});
