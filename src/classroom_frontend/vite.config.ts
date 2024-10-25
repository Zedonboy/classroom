import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
import environment from "vite-plugin-environment";
import dotenv from "dotenv";

dotenv.config({ path: "../../.env" });

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue(),
    environment("all", { prefix: "CANISTER_" }),
    environment("all", { prefix: "DFX_" }),
  ],
  resolve: {
    alias: {
      '@declaration': resolve('../declarations'),
      // 'quill-blot-formatter': 'quill-blot-formatter/dist/quill-blot-formatter.min.js'
    }
  },
  define: {
    'process.env': process.env,
    'global': "globalThis",
  },
})
