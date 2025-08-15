import { defineConfig } from 'vite'

export default defineConfig({
  server: {
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
  optimizeDeps: {
    exclude: ['../pkg/rusty2048_web.js']
  },
  build: {
    target: 'esnext',
    rollupOptions: {
      external: ['../pkg/rusty2048_web.js']
    }
  }
})
