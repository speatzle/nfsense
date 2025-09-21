/// <reference types="vite/client" />
/// <reference types="unplugin-icons/types/vue" />
declare module '*.vue' {
  import type { DefineComponent } from 'vue';
  const component: DefineComponent<{}, {}, any>;
  export default component;
}
declare module 'simple-jsonrpc-js' {
  const mod: any; // TODO: Add proper type?
  export default mod;
}
