# Tauri + SvelteKit

This template should help get you started developing with Tauri and SvelteKit in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

Your system is missing dependencies (or they do not exist in $PATH):
╭────────────────────┬─────────────────────────────────────────────────────╮
│ Node.js            │ Visit https://nodejs.org/                           │
├────────────────────┼─────────────────────────────────────────────────────┤
│ webkit2gtk & rsvg2 │ Visit https://tauri.app/guides/prerequisites/#linux │
╰────────────────────┴─────────────────────────────────────────────────────╯

Make sure you have installed the prerequisites for your OS: https://tauri.app/start/prerequisites/, then run:
  cd dashboard
  npm install
  npm run tauri android init

For Desktop development, run:
  npm run tauri dev

For Android development, run:
  npm run tauri android dev

## Installing Rerun:

```
npm i @rerun-io/web-viewer
npm i vite-plugin-top-level-await
npm i vite-plugin-wasm
```

Then you need to copy the wasm rerun module to node_modules/.vite/deps/

You also need to enable the plugins above in the vite.config.ts file.
