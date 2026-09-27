import { defineConfig } from 'vite';
import preact from '@preact/preset-vite';
import { viteSingleFile } from 'vite-plugin-singlefile';
import { VitePWA } from 'vite-plugin-pwa';

export default defineConfig(({ mode }) => ({
  base: './',
  plugins: [
    preact(),
    ...(mode === 'single' ? [viteSingleFile({ removeViteModuleLoader: true })] : []),
    ...(mode === 'pwa' ? [VitePWA({
      registerType: 'autoUpdate',
      workbox: { globPatterns: ['**/*.{js,css,html,svg,json,png}'], maximumFileSizeToCacheInBytes: 6 * 1024 * 1024 },
      manifest: {
        name: 'CDL Workshop CA', short_name: 'CDL Workshop', description: 'Learn and pass the California CDL General Knowledge and Combination tests.',
        theme_color: '#0b5d3b', background_color: '#f4f6f1', display: 'standalone', start_url: './',
        icons: [{ src: 'icon.svg', sizes: 'any', type: 'image/svg+xml', purpose: 'any maskable' }],
      },
    })] : []),
  ],
  define: { __BUILD_MODE__: JSON.stringify(mode) },
  build: { outDir: mode === 'single' ? 'dist-single' : 'dist', assetsInlineLimit: 100000000, target: 'es2020' },
  test: { include: ['tests/unit/**/*.test.ts'], environment: 'node' },
}));
