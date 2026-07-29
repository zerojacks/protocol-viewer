import { defineConfig } from 'vite';
import basicSsl from '@vitejs/plugin-basic-ssl';

export default defineConfig({
    base: '/protocol-viewer/',  // GitHub Pages 部署路径
    plugins: [
        basicSsl()  // 开发时启用 HTTPS（GPUI WebGL 需要）
    ],
    server: {
        port: 3000,
        https: true,
    },
    build: {
        target: 'esnext',
        outDir: 'dist',
    },
});
