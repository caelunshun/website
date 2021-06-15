/// <reference types="@sveltejs/kit" />

declare module "*.svg?component" {
  export {SvelteComponentDev as default} from "svelte/internal";
}
