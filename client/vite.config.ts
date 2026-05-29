import { defineConfig } from "vite";
import { fileURLToPath, URL } from "node:url";

import VueDevTools from "vite-plugin-vue-devtools";
import ReactiveVue from "unplugin-reactive-vue/vite";
import Vue from "@vitejs/plugin-vue";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import Icons from "unplugin-icons/vite";
import IconsResolver from "unplugin-icons/resolver";

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    ReactiveVue({}),
    VueDevTools(),
    Vue(),
    AutoImport({
      include: [/\.[jt]sx?$/, /\.vue$/, /\.vue\?vue/, /\.md$/],
      imports: [
        "vue",
        "vue-router",
        "vue-i18n",
        "@vueuse/core",
        "@vueuse/head",
        {
          "vue-toast-notification": ["useToast"],
          "~/util": ["equals", "isNullish", "variantOf", "atPath", "syncModel"],
          "~/plugins": ["usePlugins"],
          "~/api": ["apiCall"],
          "~/composables/modals": ["useModals"],
        },
        { from: "./src/util", imports: ["Index", "MaybeIndex"], type: true },
      ],
      dts: "src/generated/auto-imports.d.ts",
      dtsMode: "overwrite",
      dirs: ["src/composables"],
      vueTemplate: true,
    }),
    Components({
      extensions: ["vue"],
      include: [/\.vue$/, /\.vue\?vue/],
      dts: "src/generated/components.d.ts",
      resolvers: [
        IconsResolver(),
        (comp: string) => {
          if (comp === "FocusTrap") return { name: "FocusTrap", from: "focus-trap-vue" };
        },
      ],
      types: [{ from: "focus-trap-vue", names: ["FocusTrap"] }],
    }),
    Icons({}),
  ],

  // Easy Imports (Keep in sync with tsconfig.json!)
  resolve: { alias: { "~": fileURLToPath(new URL("./src", import.meta.url)) } },
});
