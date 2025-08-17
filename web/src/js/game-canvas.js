// canvas-manager.js
export class CanvasManager {
    constructor() {
        this.worker = null;
        this.canvas = null;
        this.offscreenCanvas = null;
        this.isInitialized = false;
        this.pendingMessages = [];
        this.animationDuration = 120;
        this.useWorker = true; // 是否使用 Worker
        this.fallbackRenderer = null; // 回退渲染器
    }

    async init() {
        // 创建 canvas 元素
        const container = document.querySelector('.canvas-container');
        this.canvas = document.createElement('canvas');
        this.canvas.id = 'gameCanvas';
        this.canvas.style.borderRadius = '12px';
        this.canvas.style.cursor = 'grab';
        this.canvas.style.imageRendering = 'crisp-edges';
        this.canvas.style.imageRendering = '-webkit-optimize-contrast';
        
        // 替换旧 canvas
        const old = document.getElementById('gameCanvas');
        if (old && old !== this.canvas) old.replaceWith(this.canvas);
        else container.appendChild(this.canvas);

        // 获取容器尺寸
        const bounds = container.getBoundingClientRect();
        const width = bounds.width || container.clientWidth || container.offsetWidth;
        const height = bounds.height || container.clientHeight || container.offsetHeight || width;

        // 获取设备像素比
        const devicePixelRatio = window.devicePixelRatio || 1;
        
        // 设置 canvas 的 CSS 尺寸
        this.canvas.style.width = width + 'px';
        this.canvas.style.height = height + 'px';
        
        // 设置 canvas 的实际尺寸（考虑设备像素比）
        this.canvas.width = width * devicePixelRatio;
        this.canvas.height = height * devicePixelRatio;

        // 创建 Worker
        this.worker = new Worker(new URL('./renderer.worker.js', import.meta.url), { type: 'module' });
        
        // 设置消息监听
        this.worker.onmessage = (e) => this.handleWorkerMessage(e);
        
        // 创建 OffscreenCanvas
        this.offscreenCanvas = this.canvas.transferControlToOffscreen();
        
        // 初始化 Worker
        await this.initWorker(width, height, devicePixelRatio);
        
        // 设置 resize 监听
        window.addEventListener('resize', () => this.handleResize());
    }

    async initWorker(width, height, devicePixelRatio) {
        return new Promise((resolve) => {
            const messageHandler = (e) => {
                if (e.data.type === 'initComplete') {
                    this.worker.removeEventListener('message', messageHandler);
                    this.isInitialized = true;
                    resolve();
                }
            };
            
            this.worker.addEventListener('message', messageHandler);
            
            this.worker.postMessage({
                type: 'init',
                data: {
                    offscreenCanvas: this.offscreenCanvas,
                    width: width * devicePixelRatio,
                    height: height * devicePixelRatio,
                    devicePixelRatio
                }
            }, [this.offscreenCanvas]);
        });
    }

    handleWorkerMessage(e) {
        const { type, data } = e.data;
        
        switch (type) {
            case 'initComplete':
                // Worker初始化完成
                break;
                
            case 'animationComplete':
                // 动画完成
                break;
                
            case 'isAnimatingResponse':
                // 动画状态响应
                break;
                
            default:
                // 忽略未知消息类型
                break;
        }
    }

    handleResize() {
        if (!this.isInitialized) return;
        
        const container = document.querySelector('.canvas-container');
        const bounds = container.getBoundingClientRect();
        const width = bounds.width || container.clientWidth || container.offsetWidth;
        const height = bounds.height || container.clientHeight || container.offsetHeight || width;
        
        // 获取设备像素比
        const devicePixelRatio = window.devicePixelRatio || 1;
        
        // 设置 canvas 的 CSS 尺寸
        this.canvas.style.width = width + 'px';
        this.canvas.style.height = height + 'px';
        
        // 通知 Worker 调整尺寸（Worker会处理实际的canvas尺寸调整）
        this.worker.postMessage({
            type: 'resize',
            data: {
                width: width * devicePixelRatio,
                height: height * devicePixelRatio,
                devicePixelRatio
            }
        });
    }

    actuate(tiles) {
        if (!this.isInitialized) return;
        
        this.worker.postMessage({
            type: 'drawBoardWithAnimation',
            data: { animTiles: tiles }
        });
    }

    drawBoardRaw(tiles) {
        if (!this.isInitialized) return;
        
        this.worker.postMessage({
            type: 'drawBoardRaw',
            data: { tiles }
        });
    }

    // 添加动画支持
    drawBoardWithAnimation(animTiles) {
        if (!this.isInitialized) return;
        
        // 使用主线程的requestAnimationFrame来同步Worker动画
        this.startAnimationSync();
        
        this.worker.postMessage({
            type: 'drawBoardWithAnimation',
            data: { animTiles }
        });
    }

    // 主线程动画同步
    startAnimationSync() {
        if (this.animationSyncId) return; // 避免重复启动
        
        const syncAnimation = () => {
            // 检查Worker是否还在动画中
            this.isAnimatingNow().then(animating => {
                if (animating) {
                    // 继续同步
                    this.animationSyncId = requestAnimationFrame(syncAnimation);
                } else {
                    // 动画完成，停止同步
                    this.animationSyncId = null;
                }
            }).catch(() => {
                // 如果检查失败，停止同步
                this.animationSyncId = null;
            });
        };
        
        this.animationSyncId = requestAnimationFrame(syncAnimation);
    }

    // 停止动画同步
    stopAnimationSync() {
        if (this.animationSyncId) {
            cancelAnimationFrame(this.animationSyncId);
            this.animationSyncId = null;
        }
    }

    getCanvas() { 
        return this.canvas; 
    }

    updateTileColors(theme) {
        if (!this.isInitialized) return;
        
        this.worker.postMessage({
            type: 'updateTileColors',
            data: { theme }
        });
    }

    isAnimatingNow() { 
        return new Promise((resolve) => {
            const messageHandler = (e) => {
                if (e.data.type === 'isAnimatingResponse') {
                    this.worker.removeEventListener('message', messageHandler);
                    resolve(e.data.data?.animating || false);
                }
            };
            
            this.worker.addEventListener('message', messageHandler);
            
            this.worker.postMessage({
                type: 'isAnimating'
            });
        });
    }

    // 清理资源
    destroy() {
        // 停止动画同步
        this.stopAnimationSync();
        
        if (this.worker) {
            this.worker.terminate();
            this.worker = null;
        }
        this.isInitialized = false;
    }
}
