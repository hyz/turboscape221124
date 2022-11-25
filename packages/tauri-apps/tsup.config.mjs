import { defineConfig } from 'tsup'

export default defineConfig(() => [
  {
    entry: ['src/*.ts'],
    outDir: 'dist',
    format: ['esm', 'cjs'],
    clean: true,
    minify: false,
    target: 'es2021',
    platform: 'node',
    dts: { resolve: true },
    splitting: false
  },
  {
    entry: { bundle: 'src/index.ts' },
    outDir: '../../core/tauri/scripts',
    format: ['iife'],
    globalName: '__TAURI_IIFE__',
    clean: false,
    minify: false,
    platform: 'node',
    dts: true,
    // esbuild `globalName` option generates `var __TAURI_IIFE__ = (() => {})()`
    // and var is not guaranted to assign to the global `window` object so we make sure to assign it
    footer: {
      js: 'window.__TAURI__ = __TAURI_IIFE__'
    },
    splitting: false
  }
])
