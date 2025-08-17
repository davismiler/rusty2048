// canvas-manager.js
import { Application, Graphics, Text, Container } from 'pixi.js';

export class CanvasManager {
    constructor() {
        this.app = null;
        this.tiles = new Map();      // id -> Container
        this.isAnimating = false;
        this.animationDuration = 120;
        
        // 简化的动画管理系统
        this.animations = [];
        this.animationFrameId = null;
        this.gridLayer = null;
        this.tileLayer = null;

        this.GRID_SIZE = 4;

        this.TILE_SIZE = 88;        // Tile size (will be recalculated)
        this.TILE_GAP = 12;        // Gap between tiles
        this.CANVAS_W = 400;       // Canvas width/height (may not be square)
        this.CANVAS_H = 400;
        this.GRID_ORIGIN_X = 0;     // Grid top-left origin (will be recalculated)
        this.GRID_ORIGIN_Y = 0;

        this.TILE_COLORS = {
            0: 0xcdc1b4,
            2: 0xeee4da, 4: 0xede0c8, 8: 0xf2b179, 16: 0xf59563, 32: 0xf67c5f, 64: 0xf65e3b,
            128: 0xedcf72, 256: 0xedcc61, 512: 0xedc850, 1024: 0xedc53f, 2048: 0xedc22e
        };
        this.TILE_TEXT_COLORS = {
            2: 0x776e65, 4: 0x776e65,
            8: 0xf9f6f2, 16: 0xf9f6f2, 32: 0xf9f6f2, 64: 0xf9f6f2,
            128: 0xf9f6f2, 256: 0xf9f6f2, 512: 0xf9f6f2, 1024: 0xf9f6f2, 2048: 0xf9f6f2
        };
    }

    async init() {
        this.app = new Application();
        await this.app.init({
            background: '#bbada0',
            resizeTo: document.querySelector('.canvas-container'),
            antialias: true,
            powerPreference: 'high-performance',
            resolution: window.devicePixelRatio || 1,
            autoDensity: true,
            roundPixels: true,
        });
        
        // Enable pixel alignment (roundPixels is read-only in Pixi v8, set during initialization)
        const container = document.querySelector('.canvas-container');
        const canvas = this.app.canvas;
        canvas.id = 'gameCanvas';
        canvas.style.borderRadius = '12px';
        canvas.style.cursor = 'grab';
        canvas.style.imageRendering = 'crisp-edges';
        canvas.style.imageRendering = '-webkit-optimize-contrast';
        // Replace old canvas with new one (if there's a placeholder canvas in HTML)
        const old = document.getElementById('gameCanvas');
        if (old && old !== canvas) old.replaceWith(canvas);
        else container.appendChild(canvas);

        // 创建分层结构
        this.gridLayer = new Container();
        this.tileLayer = new Container();
        this.app.stage.addChild(this.gridLayer);
        this.app.stage.addChild(this.tileLayer);

        this.calculateCanvasSize();
        this.drawGrid();
        window.addEventListener('resize', () => this.handleResize());
    }

    calculateCanvasSize() {
        const container = document.querySelector('.canvas-container');
        if (!container) return;

        const bounds = container.getBoundingClientRect();

        // Canvas dimensions (Pixi will scale proportionally to fill container; use visible area here)
        const w = bounds.width || container.clientWidth || container.offsetWidth;
        const h = bounds.height || container.clientHeight || container.offsetHeight || w;     // Your container uses aspect-ratio:1, usually w≈h
        this.CANVAS_W = w;
        this.CANVAS_H = h;

        // Suggest using a fixed gap ratio (or keep your original 12px constant)
        // Use relative values here for consistent appearance across different sizes:
        this.TILE_GAP = Math.max(8, Math.round(Math.min(w, h) * 0.03)); // About 3% width

        // Grid total width/height: GRID_SIZE tiles + (GRID_SIZE+1) gaps
        // Calculate tile size backwards (use float to avoid cumulative errors)
        const gridTotalW = w;
        const gridTotalH = h;
        const tileSizeW = (gridTotalW - (this.GRID_SIZE + 1) * this.TILE_GAP) / this.GRID_SIZE;
        const tileSizeH = (gridTotalH - (this.GRID_SIZE + 1) * this.TILE_GAP) / this.GRID_SIZE;

        // Use square tiles, take the smaller one to ensure no overflow
        this.TILE_SIZE = Math.floor(Math.min(tileSizeW, tileSizeH));

        // 以最终 tileSize 反推"实际占用"的网格宽高
        const actualGridW = this.GRID_SIZE * this.TILE_SIZE + (this.GRID_SIZE + 1) * this.TILE_GAP;
        const actualGridH = this.GRID_SIZE * this.TILE_SIZE + (this.GRID_SIZE + 1) * this.TILE_GAP;

        // 居中起点（四舍五入避免半像素导致渲染模糊）
        this.GRID_ORIGIN_X = Math.round((w - actualGridW) / 2);
        this.GRID_ORIGIN_Y = Math.round((h - actualGridH) / 2);
        
        // 确保所有位置都是整数像素
        this.GRID_ORIGIN_X = Math.floor(this.GRID_ORIGIN_X);
        this.GRID_ORIGIN_Y = Math.floor(this.GRID_ORIGIN_Y);
    }

    drawGrid() {
        if (!this.gridLayer) return;
        this.gridLayer.removeChildren();

        for (let row = 0; row < this.GRID_SIZE; row++) {
            for (let col = 0; col < this.GRID_SIZE; col++) {
                const { x, y } = this.positionToXY(col, row);
                const cell = new Graphics()
                    .roundRect(0, 0, this.TILE_SIZE, this.TILE_SIZE, 6) // 画圆角矩形
                    .fill(0xcdc1b4); // 填充颜色
                cell.x = x; cell.y = y;
                this.gridLayer.addChild(cell);
            }
        }
    }

    // 统一的动画管理系统
    startAnimationLoop() {
        if (this.animationFrameId) return; // 已在动画中
        this.isAnimating = true;
        
        const loop = () => {
            const now = performance.now();
            this.animations = this.animations.filter(fn => fn(now)); // 返回 true 保留，false 删去
            if (this.animations.length > 0) {
                this.animationFrameId = requestAnimationFrame(loop);
            } else {
                this.animationFrameId = null;
                this.isAnimating = false;
            }
        };
        this.animationFrameId = requestAnimationFrame(loop);
    }

    // tiles: [{id,value,x,y, previousPosition?, mergedFrom?, isNew?}, ...]
    actuate(tiles) {
        if (!this.app) return;
        
        // 清理之前的 tiles
        this.tileLayer.removeChildren();
        this.tiles.clear();
        this.animations = []; // 清除之前的动画

        const spritesById = new Map();

        // 1) 先渲染在旧位置（无 previousPosition 则渲染在新位置）
        for (const t of tiles) {
            const start = t.previousPosition ?? { x: t.x, y: t.y };
            const sprite = this.makeTileSprite(t.value);
            const startXY = this.positionToXY(start.x, start.y);
            sprite.x = startXY.x;
            sprite.y = startXY.y;
            if (t.isNew && sprite._inner) sprite._inner.scale.set(0); // 在内层做缩放
            this.tileLayer.addChild(sprite);
            spritesById.set(t.id, sprite);
            this.tiles.set(t.id, sprite);
        }

        // 2) 添加所有动画到队列
        for (const t of tiles) {
            const spr = spritesById.get(t.id);
            if (!spr) continue;

            // 移动动画
            this.addMoveAnimation(
                spr,
                (t.previousPosition ?? { x: t.x, y: t.y }),
                { x: t.x, y: t.y },
                this.animationDuration
            );

            // 合并动画
            if (t.mergedFrom) {
                for (const src of t.mergedFrom) {
                    const s = this.makeTileSprite(src.value);
                    const fromXY = this.positionToXY(src.previousPosition.x, src.previousPosition.y);
                    s.x = fromXY.x; s.y = fromXY.y;
                    this.tileLayer.addChild(s);
                    this.addMoveAnimation(
                        s, src.previousPosition, { x: t.x, y: t.y }, this.animationDuration, 
                        () => s.destroy()
                    );
                }
                this.addPopAnimation(spr, this.animationDuration * 0.6);
            }

            // 新生动画
            if (t.isNew) {
                this.addScaleInAnimation(spr, this.animationDuration * 0.8);
            }
        }

        // 启动动画循环
        this.startAnimationLoop();
    }

    drawBoardRaw(tiles) {
        if (!this.app) return;
        this.drawGrid();
        this.tileLayer.removeChildren();
        for (const t of tiles) {
            const s = this.makeTileSprite(t.value);
            const { x, y } = this.positionToXY(t.x, t.y);
            s.x = x; s.y = y;
            this.tileLayer.addChild(s);
        }
    }

    makeTileSprite(value) {
        // 外层：只负责移动（x,y 是格子的左上角）
        const outer = new Container();

        // 内层：负责缩放/弹跳（pivot 在中心）
        const inner = new Container();
        const half = this.TILE_SIZE / 2;
        inner.pivot.set(half, half);
        inner.position.set(half, half); // 把内容对齐回左上角

        // 创建背景
        const bg = new Graphics()
            .roundRect(0, 0, this.TILE_SIZE, this.TILE_SIZE, 6) // 画圆角矩形
            .fill({ color: this.TILE_COLORS[value] ?? 0xcdc1b4 }); // 填充颜色

        // 创建文本 - 恢复到原始的字体设置
        const fontSize =
            value >= 1000 ? Math.floor(this.TILE_SIZE * 0.35) :
                value >= 100 ? Math.floor(this.TILE_SIZE * 0.45) :
                    Math.floor(this.TILE_SIZE * 0.55);

        const dpr = Math.max(2, window.devicePixelRatio || 1);
        
        const txt = new Text({
            text: String(value),
            style: {
                fontFamily: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif",
                fontSize,
                fill: this.TILE_TEXT_COLORS[value] ?? 0x776e65,
                fontWeight: 'bold'
            },
            resolution: dpr
        });
        
        // 确保高分辨率纹理
        if (txt.texture?.baseTexture?.setResolution) {
            txt.texture.baseTexture.setResolution(dpr);
            txt.updateText?.();
        }
        
        txt.anchor.set(0.5);
        txt.x = Math.round(this.TILE_SIZE / 2);
        txt.y = Math.round(this.TILE_SIZE / 2);
        txt.roundPixels = true;

        inner.addChild(bg);
        inner.addChild(txt);
        outer.addChild(inner);

        // 让后续动画方法能拿到内层
        outer._inner = inner;
        
        return outer;
    }

    positionToXY(col, row) {
        const x = this.GRID_ORIGIN_X + this.TILE_GAP + col * (this.TILE_SIZE + this.TILE_GAP);
        const y = this.GRID_ORIGIN_Y + this.TILE_GAP + row * (this.TILE_SIZE + this.TILE_GAP);
        // 用整数像素，避免半像素模糊
        return { x: Math.floor(x), y: Math.floor(y) };
    }

    // 优化的动画方法，使用统一的调度系统
    addMoveAnimation(g, from, to, duration = 150, onEnd) {
        const start = performance.now();
        const startXY = this.positionToXY(from.x, from.y);
        const endXY = this.positionToXY(to.x, to.y);
        
        this.animations.push(now => {
            const t = Math.min(1, (now - start) / duration);
            // 使用更平滑的 easing 函数
            const p = this.easeOutQuad(t);
            // 动画过程中保留小数精度，避免锯齿跳跃
            g.x = startXY.x + (endXY.x - startXY.x) * p;
            g.y = startXY.y + (endXY.y - startXY.y) * p;
            if (t < 1) return true;
            onEnd?.();
            return false;
        });
    }

    addScaleInAnimation(g, duration = 120) {
        const target = g._inner ?? g;
        const start = performance.now();
        
        this.animations.push(now => {
            const t = Math.min(1, (now - start) / duration);
            const p = this.easeOutBack(t);
            target.scale.set(p);
            if (t < 1) return true;
            target.scale.set(1);
            return false;
        });
    }

    addPopAnimation(g, duration = 100) {
        const target = g._inner ?? g;
        const from = 1, mid = 1.12, to = 1;
        const start = performance.now();
        
        this.animations.push(now => {
            const t = Math.min(1, (now - start) / duration);
            const ease = (x) => 1 - Math.pow(1 - x, 3);
            const s = t < .5 ? from + (mid - from) * ease(t * 2)
                : mid + (to - mid) * ease((t - .5) * 2);
            target.scale.set(s);
            if (t < 1) return true;
            target.scale.set(1);
            return false;
        });
    }

    // 优化的 easing 函数
    easeOutQuad(t) {
        return t * (2 - t);
    }

    easeInOutSine(t) {
        return -(Math.cos(Math.PI * t) - 1) / 2;
    }

    easeOutBack(t) {
        const c1 = 1.70158, c3 = c1 + 1;
        return 1 + c3 * Math.pow(t - 1, 3) + c1 * Math.pow(t - 1, 2);
    }

    handleResize() { 
        this.calculateCanvasSize(); 
        this.drawGrid(); 
    }
    
    getCanvas() { return this.app ? this.app.canvas : null; }

    updateTileColors(theme) {
        if (theme.tile_colors) {
            theme.tile_colors.forEach((hex, idx) => {
                const val = Math.pow(2, idx);
                this.TILE_COLORS[val] = parseInt(hex.replace('#', '0x'), 16);
            });
        }
    }

    isAnimatingNow() { return this.isAnimating; }
}
