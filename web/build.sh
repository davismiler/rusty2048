#!/bin/bash

# 构建WASM
echo "Building WASM..."
cd ../core
wasm-pack build --target web --out-dir ../web/pkg
cd ../web

# 安装依赖（如果还没有安装）
if [ ! -d "node_modules" ]; then
    echo "Installing dependencies..."
    npm install
fi

# 启动开发服务器
echo "Starting development server..."
npm run dev
