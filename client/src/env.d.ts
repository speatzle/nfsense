/// <reference types="vite/client" />
/// <reference types="unplugin-icons/types/vue" />
/// <reference types="vite-plugin-pages/client" />
/// <reference types="unplugin-reactive-vue/types" />
declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<object, object, any>;
  export default component;
}
declare module "simple-jsonrpc-js" {
  // TODO: Add proper type?
  const mod: any;
  export default mod;
}
