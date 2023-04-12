import { defineConfig } from 'vite';
import Vue from '@vitejs/plugin-vue';
import Pages from 'vite-plugin-pages';
import Markdown from 'vite-plugin-vue-markdown';

import Components from 'unplugin-vue-components/vite';
import Icons from 'unplugin-icons/vite';
import IconsResolver from 'unplugin-icons/resolver';
import I18N from '@intlify/unplugin-vue-i18n/vite';
import Macros from 'unplugin-vue-macros/vite';
import AutoImport from 'unplugin-auto-import/vite';

import Shiki from 'markdown-it-shiki';
import LinkAttributes from 'markdown-it-link-attributes';

// https://vitejs.dev/config/
export default defineConfig({
  server: {
    "proxy": {
      "/api": "http://localhost:8080",
      "/login": "http://localhost:8080",
      "/logout": "http://localhost:8080",
      "/session": "http://localhost:8080",
      "/ws": {
        target: "ws://localhost:8080",
        ws: true,
      },
    },
  },
  plugins: [
    Macros({
      plugins: {
        vue: Vue({
          include: [/\.vue$/, /\.md$/],
          reactivityTransform: true,
        }),
      },
    }),
    Pages({
      extensions: ['vue', 'md'],
    }),
    Markdown({
      wrapperClasses: 'prose prose-sm m-auto text-left',
      headEnabled: true,
      markdownItSetup(md) {
        md.use(Shiki, {
          theme: {
            light: 'vitesse-light',
            dark: 'vitesse-dark',
          },
        });
        md.use(LinkAttributes, {
          matcher: (link: string) => /^https?:\/\//.test(link),
          attrs: {
            target: '_blank',
            rel: 'noopener',
          },
        });
      },
    }),
    Components({
      extensions: ['vue', 'md'],
      include: [/\.vue$/, /\.vue\?vue/, /\.md$/],
      dts: 'src/generated/components.d.ts',
      resolvers: [
        IconsResolver(),
        (componentName) => {
          if (componentName === 'FocusTrap')
            return { name: 'FocusTrap', from: 'focus-trap-vue' };
        },
      ],
      types: [{
        from: 'focus-trap-vue',
        names: ['FocusTrap'],
      }],
    }),
    Icons({
    }),
    I18N({
      runtimeOnly: true,
      compositionOnly: true,
      fullInstall: true,
      include: ['src/locales'],
    }),
    AutoImport({
      include: [
        /\.[tj]sx?$/,
        /\.vue$/, /\.vue\?vue/,
        /\.md$/,
      ],
      imports: [
        'vue',
        'vue-router',
        'vue-i18n',
        'vue/macros',
        '@vueuse/core',
        '@vueuse/head',
        {
          from: "vue-toast-notification",
          imports: ["useToast"],
        },
      ],
      dts: 'src/generated/auto-imports.d.ts',
      dirs: ['src/composables'],
      vueTemplate: true,
    }),
  ],
});
