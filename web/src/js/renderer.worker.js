class CanvasManager {
    constructor() {
        this.canvas = null;
        this.ctx = null;
        this.isAnimating = false;
        this.animationDuration = 120;
        this.animationFrameId = null;
        this.currentAnimations = [];
        this.animationStartTime = 0;
        this.devicePixelRatio = 1;
        this.mergeAnimations = new Map(); // 存储合并动画状态
        this.fontSize = 0; // 缓存字体大小
        this.fontSizeCache = new Map(); // 缓存不同数字的字体大小

        this.GRID_SIZE = 4;
        this.TILE_SIZE = 88;
        this.TILE_GAP = 12;
        this.CANVAS_W = 400;
        this.CANVAS_H = 400;
        this.GRID_ORIGIN_X = 0;
        this.GRID_ORIGIN_Y = 0;

        this.TILE_COLORS = {
            0: '#cdc1b4',
            2: '#eee4da', 4: '#ede0c8', 8: '#f2b179', 16: '#f59563', 32: '#f67c5f', 64: '#f65e3b',
            128: '#edcf72', 256: '#edcc61', 512: '#edc850', 1024: '#edc53f', 2048: '#edc22e'
        };
        this.TILE_TEXT_COLORS = {
            2: '#776e65', 4: '#776e65',
            8: '#f9f6f2', 16: '#f9f6f2', 32: '#f9f6f2', 64: '#f9f6f2',
            128: '#f9f6f2', 256: '#f9f6f2', 512: '#f9f6f2', 1024: '#f9f6f2', 2048: '#f9f6f2'
        };
    }

    init(offscreenCanvas, containerWidth, containerHeight, devicePixelRatio) {
        this.canvas = offscreenCanvas;
        this.ctx = this.canvas.getContext('2d');
        this.devicePixelRatio = devicePixelRatio || 1;
        
        // 设置 canvas 尺寸
        this.canvas.width = containerWidth;
        this.canvas.height = containerHeight;
        
        // 设置上下文缩放以匹配设备像素比
        this.ctx.scale(this.devicePixelRatio, this.devicePixelRatio);
        
        this.calculateCanvasSize(containerWidth / this.devicePixelRatio, containerHeight / this.devicePixelRatio);
        this.drawGrid();
    }

    calculateCanvasSize(containerWidth, containerHeight) {
        const w = containerWidth;
        const h = containerHeight || w;

        this.CANVAS_W = w;
        this.CANVAS_H = h;

        // 优化：使用位运算代替Math.round
        this.TILE_GAP = Math.max(8, (Math.min(w, h) * 0.03) | 0);

        const gridTotalW = w;
        const gridTotalH = h;
        const tileSizeW = (gridTotalW - (this.GRID_SIZE + 1) * this.TILE_GAP) / this.GRID_SIZE;
        const tileSizeH = (gridTotalH - (this.GRID_SIZE + 1) * this.TILE_GAP) / this.GRID_SIZE;

        // 优化：使用位运算代替Math.floor
        this.TILE_SIZE = Math.min(tileSizeW, tileSizeH) | 0;

        const actualGridW = this.GRID_SIZE * this.TILE_SIZE + (this.GRID_SIZE + 1) * this.TILE_GAP;
        const actualGridH = this.GRID_SIZE * this.TILE_SIZE + (this.GRID_SIZE + 1) * this.TILE_GAP;

        // 优化：使用位运算代替Math.round和Math.floor
        this.GRID_ORIGIN_X = ((w - actualGridW) / 2) | 0;
        this.GRID_ORIGIN_Y = ((h - actualGridH) / 2) | 0;
        
        // 更新缓存的字体大小
        this.fontSize = (this.TILE_SIZE * 0.5) | 0;
        
        // 清除字体大小缓存，因为tile大小改变了
        this.clearFontSizeCache();
    }

    drawGrid() {
        if (!this.ctx) return;
        
        // 清空画布
        this.ctx.fillStyle = '#bbada0';
        this.ctx.fillRect(0, 0, this.CANVAS_W, this.CANVAS_H);

        // 绘制网格 - 添加圆角
        this.ctx.fillStyle = '#cdc1b4';
        for (let row = 0; row < this.GRID_SIZE; row++) {
            for (let col = 0; col < this.GRID_SIZE; col++) {
                const { x, y } = this.positionToXY(col, row);
                this.drawRoundedRect(x, y, this.TILE_SIZE, this.TILE_SIZE, 6);
            }
        }
    }

    // 绘制圆角矩形
    drawRoundedRect(x, y, width, height, radius) {
        this.ctx.beginPath();
        this.ctx.moveTo(x + radius, y);
        this.ctx.lineTo(x + width - radius, y);
        this.ctx.quadraticCurveTo(x + width, y, x + width, y + radius);
        this.ctx.lineTo(x + width, y + height - radius);
        this.ctx.quadraticCurveTo(x + width, y + height, x + width - radius, y + height);
        this.ctx.lineTo(x + radius, y + height);
        this.ctx.quadraticCurveTo(x, y + height, x, y + height - radius);
        this.ctx.lineTo(x, y + radius);
        this.ctx.quadraticCurveTo(x, y, x + radius, y);
        this.ctx.closePath();
        this.ctx.fill();
    }

    drawBoardRaw(tiles) {
        if (!this.ctx) return;
        
        this.drawGrid();
        
        for (const tile of tiles) {
            this.drawTile(tile.value, tile.x, tile.y);
        }
    }

    // 添加动画支持
    drawBoardWithAnimation(animTiles) {
        if (!this.ctx) return;
        
        this.currentAnimations = animTiles;
        this.isAnimating = true;
        this.animationStartTime = performance.now();
        this.mergeAnimations.clear(); // 清除之前的合并动画
        
        // 检测合并动画
        for (const tile of animTiles) {
            // 检查是否有mergedFrom属性且长度大于1
            if (tile.mergedFrom && tile.mergedFrom.length > 1) {
                // 这是一个合并的tile，需要添加合并动画
                const key = `${tile.x},${tile.y}`;
                this.mergeAnimations.set(key, {
                    startTime: performance.now() + this.animationDuration, // 移动动画结束后开始
                    duration: 200, // 合并动画持续时间增加到200ms
                    scale: 1.0
                });
            }
        }
        
        this.animate();
    }

    animate() {
        if (!this.isAnimating || !this.ctx) return;
        
        const currentTime = performance.now();
        const elapsed = currentTime - this.animationStartTime;
        const progress = Math.min(elapsed / this.animationDuration, 1);
        
        // 使用缓动函数
        const easeProgress = this.easeOutCubic(progress);
        
        this.drawGrid();
        
        for (const tile of this.currentAnimations) {
            this.drawAnimatedTile(tile, easeProgress, currentTime);
        }
        
        // 检查是否还有合并动画在进行
        let hasActiveMergeAnimations = false;
        for (const [key, mergeAnim] of this.mergeAnimations) {
            const mergeElapsed = currentTime - mergeAnim.startTime;
            if (mergeElapsed < mergeAnim.duration) {
                hasActiveMergeAnimations = true;
                break;
            }
        }
        
        if (progress < 1 || hasActiveMergeAnimations) {
            // 继续动画（移动动画未完成或合并动画在进行）
            // 使用较短的间隔，让主线程的requestAnimationFrame来控制节奏
            this.animationFrameId = setTimeout(() => this.animate(), 8); // ~120fps
        } else {
            // 所有动画完成
            this.isAnimating = false;
            this.currentAnimations = [];
            this.mergeAnimations.clear();
            self.postMessage({ type: 'animationComplete', data: {} });
        }
    }

    // 缓动函数
    easeOutCubic(t) {
        return 1 - Math.pow(1 - t, 3);
    }

    // 合并动画的缩放函数（先压缩再放大回弹）
    getMergeScale(progress) {
        if (progress < 0.3) {
            // 前30%：压缩到0.8倍（撞击效果）
            return 0.8 + (0.2 * (progress / 0.3));
        } else if (progress < 0.7) {
            // 30%-70%：放大到1.3倍
            const expandProgress = (progress - 0.3) / 0.4;
            return 1.0 + (0.3 * expandProgress);
        } else {
            // 70%-100%：回弹到1.0倍
            const bounceProgress = (progress - 0.7) / 0.3;
            return 1.3 - (0.3 * this.easeOutBounce(bounceProgress));
        }
    }

    // 回弹缓动函数
    easeOutBounce(t) {
        if (t < 1 / 2.75) {
            return 7.5625 * t * t;
        } else if (t < 2 / 2.75) {
            return 7.5625 * (t -= 1.5 / 2.75) * t + 0.75;
        } else if (t < 2.5 / 2.75) {
            return 7.5625 * (t -= 2.25 / 2.75) * t + 0.9375;
        } else {
            return 7.5625 * (t -= 2.625 / 2.75) * t + 0.984375;
        }
    }

    drawAnimatedTile(tile, progress, currentTime) {
        if (!this.ctx) return;
        
        let x, y;
        let isMergedTile = false;
        let mergeScale = 1.0;
        
        // 检查是否是合并的tile
        if (tile.mergedFrom && tile.mergedFrom.length > 1) {
            isMergedTile = true;
            const key = `${tile.x},${tile.y}`;
            const mergeAnim = this.mergeAnimations.get(key);
            
            if (mergeAnim) {
                const mergeElapsed = currentTime - mergeAnim.startTime;
                if (mergeElapsed >= 0) { // 合并动画已开始
                    const mergeProgress = Math.min(mergeElapsed / mergeAnim.duration, 1);
                    mergeScale = this.getMergeScale(mergeProgress);
                }
            }
        }
        
        if (tile.previousPosition) {
            // 移动动画
            const startX = this.positionToXY(tile.previousPosition.x, tile.previousPosition.y).x;
            const startY = this.positionToXY(tile.previousPosition.x, tile.previousPosition.y).y;
            const endX = this.positionToXY(tile.x, tile.y).x;
            const endY = this.positionToXY(tile.x, tile.y).y;
            
            x = startX + (endX - startX) * progress;
            y = startY + (endY - startY) * progress;
        } else {
            // 新生动画
            const pos = this.positionToXY(tile.x, tile.y);
            x = pos.x;
            y = pos.y;
            
            if (tile.isNew) {
                // 新生tile的缩放动画
                const scale = progress * mergeScale; // 结合新生和合并缩放
                const scaledSize = this.TILE_SIZE * scale;
                const offsetX = (this.TILE_SIZE - scaledSize) / 2;
                const offsetY = (this.TILE_SIZE - scaledSize) / 2;
                
                this.ctx.save();
                this.ctx.translate(x + offsetX, y + offsetY);
                this.ctx.scale(scale, scale);
                this.drawRoundedRect(0, 0, this.TILE_SIZE, this.TILE_SIZE, 6);
                this.ctx.fillStyle = this.TILE_COLORS[tile.value] || '#cdc1b4';
                this.ctx.fill();
                
                if (tile.value > 0) {
                    this.ctx.fillStyle = this.TILE_TEXT_COLORS[tile.value] || '#776e65';
                    
                    // 使用自适应字体大小
                    const fontSize = this.getAdaptiveFontSize(tile.value);
                    this.ctx.font = `bold ${fontSize}px -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif`;
                    this.ctx.textAlign = 'center';
                    this.ctx.textBaseline = 'middle';
                    
                    // 启用字体平滑
                    this.ctx.imageSmoothingEnabled = true;
                    this.ctx.imageSmoothingQuality = 'high';
                    
                    // 绘制文字阴影以提高可读性
                    this.ctx.shadowColor = 'rgba(0, 0, 0, 0.1)';
                    this.ctx.shadowBlur = 1;
                    this.ctx.shadowOffsetX = 0;
                    this.ctx.shadowOffsetY = 1;
                    
                    this.ctx.fillText(
                        String(tile.value), 
                        this.TILE_SIZE / 2, 
                        this.TILE_SIZE / 2
                    );
                    
                    // 重置阴影
                    this.ctx.shadowColor = 'transparent';
                    this.ctx.shadowBlur = 0;
                    this.ctx.shadowOffsetX = 0;
                    this.ctx.shadowOffsetY = 0;
                }
                this.ctx.restore();
                return;
            }
        }
        
        // 如果是合并的tile，应用合并缩放
        if (isMergedTile && mergeScale !== 1.0) {
            const scaledSize = this.TILE_SIZE * mergeScale;
            const offsetX = (this.TILE_SIZE - scaledSize) / 2;
            const offsetY = (this.TILE_SIZE - scaledSize) / 2;
            
            this.ctx.save();
            this.ctx.translate(x + offsetX, y + offsetY);
            this.ctx.scale(mergeScale, mergeScale);
            
            // 绘制背景 - 使用圆角
            this.ctx.fillStyle = this.TILE_COLORS[tile.value] || '#cdc1b4';
            this.drawRoundedRect(0, 0, this.TILE_SIZE, this.TILE_SIZE, 6);
            
            // 绘制文字
            if (tile.value > 0) {
                this.ctx.fillStyle = this.TILE_TEXT_COLORS[tile.value] || '#776e65';
                
                // 使用自适应字体大小
                const fontSize = this.getAdaptiveFontSize(tile.value);
                this.ctx.font = `bold ${fontSize}px -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif`;
                this.ctx.textAlign = 'center';
                this.ctx.textBaseline = 'middle';
                
                // 启用字体平滑
                this.ctx.imageSmoothingEnabled = true;
                this.ctx.imageSmoothingQuality = 'high';
                
                // 绘制文字阴影以提高可读性
                this.ctx.shadowColor = 'rgba(0, 0, 0, 0.1)';
                this.ctx.shadowBlur = 1;
                this.ctx.shadowOffsetX = 0;
                this.ctx.shadowOffsetY = 1;
                
                this.ctx.fillText(
                    String(tile.value), 
                    this.TILE_SIZE / 2, 
                    this.TILE_SIZE / 2
                );
                
                // 重置阴影
                this.ctx.shadowColor = 'transparent';
                this.ctx.shadowBlur = 0;
                this.ctx.shadowOffsetX = 0;
                this.ctx.shadowOffsetY = 0;
            }
            
            this.ctx.restore();
        } else {
            // 普通tile，正常绘制
            this.ctx.fillStyle = this.TILE_COLORS[tile.value] || '#cdc1b4';
            this.drawRoundedRect(x, y, this.TILE_SIZE, this.TILE_SIZE, 6);
            
            // 绘制文字
            if (tile.value > 0) {
                this.ctx.fillStyle = this.TILE_TEXT_COLORS[tile.value] || '#776e65';
                
                // 使用自适应字体大小
                const fontSize = this.getAdaptiveFontSize(tile.value);
                this.ctx.font = `bold ${fontSize}px -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif`;
                this.ctx.textAlign = 'center';
                this.ctx.textBaseline = 'middle';
                
                // 启用字体平滑
                this.ctx.imageSmoothingEnabled = true;
                this.ctx.imageSmoothingQuality = 'high';
                
                // 绘制文字阴影以提高可读性
                this.ctx.shadowColor = 'rgba(0, 0, 0, 0.1)';
                this.ctx.shadowBlur = 1;
                this.ctx.shadowOffsetX = 0;
                this.ctx.shadowOffsetY = 1;
                
                this.ctx.fillText(
                    String(tile.value), 
                    x + this.TILE_SIZE / 2, 
                    y + this.TILE_SIZE / 2
                );
                
                // 重置阴影
                this.ctx.shadowColor = 'transparent';
                this.ctx.shadowBlur = 0;
                this.ctx.shadowOffsetX = 0;
                this.ctx.shadowOffsetY = 0;
            }
        }
    }

    drawTile(value, col, row) {
        if (!this.ctx) return;
        
        const { x, y } = this.positionToXY(col, row);
        
        // 绘制背景 - 使用圆角
        this.ctx.fillStyle = this.TILE_COLORS[value] || '#cdc1b4';
        this.drawRoundedRect(x, y, this.TILE_SIZE, this.TILE_SIZE, 6);
        
        // 绘制文字
        if (value > 0) {
            this.ctx.fillStyle = this.TILE_TEXT_COLORS[value] || '#776e65';
            
            // 使用自适应字体大小
            const fontSize = this.getAdaptiveFontSize(value);
            this.ctx.font = `bold ${fontSize}px -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif`;
            this.ctx.textAlign = 'center';
            this.ctx.textBaseline = 'middle';
            
            // 启用字体平滑
            this.ctx.imageSmoothingEnabled = true;
            this.ctx.imageSmoothingQuality = 'high';
            
            // 绘制文字阴影以提高可读性
            this.ctx.shadowColor = 'rgba(0, 0, 0, 0.1)';
            this.ctx.shadowBlur = 1;
            this.ctx.shadowOffsetX = 0;
            this.ctx.shadowOffsetY = 1;
            
            this.ctx.fillText(
                String(value), 
                x + this.TILE_SIZE / 2, 
                y + this.TILE_SIZE / 2
            );
            
            // 重置阴影
            this.ctx.shadowColor = 'transparent';
            this.ctx.shadowBlur = 0;
            this.ctx.shadowOffsetX = 0;
            this.ctx.shadowOffsetY = 0;
        }
    }

    positionToXY(col, row) {
        const x = this.GRID_ORIGIN_X + this.TILE_GAP + col * (this.TILE_SIZE + this.TILE_GAP);
        const y = this.GRID_ORIGIN_Y + this.TILE_GAP + row * (this.TILE_SIZE + this.TILE_GAP);
        return { x: x | 0, y: y | 0 };
    }

    resize(containerWidth, containerHeight, devicePixelRatio) {
        if (!this.canvas) return;
        
        this.devicePixelRatio = devicePixelRatio || 1;
        this.canvas.width = containerWidth;
        this.canvas.height = containerHeight;
        
        // 重新设置上下文缩放
        this.ctx.scale(this.devicePixelRatio, this.devicePixelRatio);
        
        this.calculateCanvasSize(containerWidth / this.devicePixelRatio, containerHeight / this.devicePixelRatio);
        this.drawGrid();
    }

    updateTileColors(theme) {
        if (theme.tile_colors) {
            theme.tile_colors.forEach((hex, idx) => {
                // 优化：使用位运算代替Math.pow(2, idx)
                const val = 1 << idx;
                this.TILE_COLORS[val] = hex;
            });
        }
    }

    isAnimatingNow() { 
        return this.isAnimating; 
    }

    // 计算自适应字体大小
    getAdaptiveFontSize(value) {
        // 检查缓存
        if (this.fontSizeCache.has(value)) {
            return this.fontSizeCache.get(value);
        }
        
        const valueStr = String(value);
        const digitCount = valueStr.length;
        
        // 基础字体大小（4位数以内）
        let fontSize = this.fontSize;
        
        // 根据位数调整字体大小
        if (digitCount === 1) {
            fontSize = this.fontSize; // 1位数：100%
        } else if (digitCount === 2) {
            fontSize = this.fontSize * 0.9; // 2位数：90%
        } else if (digitCount === 3) {
            fontSize = this.fontSize * 0.75; // 3位数：75%
        } else if (digitCount === 4) {
            fontSize = this.fontSize * 0.6; // 4位数：60%
        } else {
            fontSize = this.fontSize * 0.5; // 5位数及以上：50%
        }
        
        // 确保字体大小不小于最小值
        const minFontSize = Math.max(12, this.TILE_SIZE * 0.2);
        fontSize = Math.max(fontSize, minFontSize);
        
        // 缓存结果
        this.fontSizeCache.set(value, fontSize);
        
        return fontSize;
    }

    // 清除字体大小缓存（当tile大小改变时调用）
    clearFontSizeCache() {
        this.fontSizeCache.clear();
    }
}

// Worker 实例
const canvasManager = new CanvasManager();

// 消息处理
self.onmessage = function(e) {
    const { type, data } = e.data;
    
    switch (type) {
        case 'init':
            canvasManager.init(data.offscreenCanvas, data.width, data.height, data.devicePixelRatio);
            self.postMessage({ type: 'initComplete', data: {} });
            break;
            
        case 'drawBoardRaw':
            canvasManager.drawBoardRaw(data.tiles);
            break;
            
        case 'drawBoardWithAnimation':
            canvasManager.drawBoardWithAnimation(data.animTiles);
            break;
            
        case 'resize':
            canvasManager.resize(data.width, data.height, data.devicePixelRatio);
            break;
            
        case 'updateTileColors':
            canvasManager.updateTileColors(data.theme);
            break;
            
        case 'isAnimating':
            const animating = canvasManager.isAnimatingNow();
            self.postMessage({ type: 'isAnimatingResponse', data: { animating } });
            break;
            
        default:
            // 忽略未知消息类型
            break;
    }
};
