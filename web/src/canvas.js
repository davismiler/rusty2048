// canvas-manager.js
import { Application, Graphics, Text, Container } from 'pixi.js';

export class CanvasManager {
    constructor() {
        this.app = null;
        this.tiles = new Map();      // id -> Container
        this.isAnimating = false;
        this.animationDuration = 150;

        this.GRID_SIZE = 4;

        this.TILE_SIZE = 88;        // 每格大小（会重新计算）
        this.TILE_GAP = 12;        // 仅用于格与格之间的间距
        this.CANVAS_W = 400;       // 画布宽高（可能非正方）
        this.CANVAS_H = 400;
        this.GRID_ORIGIN_X = 0;     // 网格左上角起点（会重新计算）
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
        });
        const container = document.querySelector('.canvas-container');
        const canvas = this.app.canvas;
        canvas.id = 'gameCanvas';
        canvas.style.borderRadius = '12px';
        canvas.style.cursor = 'grab';
        // 用新 canvas 替换旧的（如果 HTML 里已有一个占位 canvas）
        const old = document.getElementById('gameCanvas');
        if (old && old !== canvas) old.replaceWith(canvas);
        else container.appendChild(canvas);

        this.calculateCanvasSize();
        this.drawGrid();
        window.addEventListener('resize', () => this.handleResize());
    }

    calculateCanvasSize() {
        const container = document.querySelector('.canvas-container');
        if (!container) return;

        const bounds = container.getBoundingClientRect();

        // 画布尺寸（Pixi 会等比撑满容器；这里用可见区域）
        const w = bounds.width || container.clientWidth || container.offsetWidth;
        const h = bounds.height || container.clientHeight || container.offsetHeight || w;     // 你的容器用了 aspect-ratio:1，通常 w≈h
        this.CANVAS_W = w;
        this.CANVAS_H = h;

        // 建议固定一个 gap 比例（或保持你原来的 12 像素常量）
        // 这里用相对值，让不同尺寸下观感一致：
        this.TILE_GAP = Math.max(8, Math.round(Math.min(w, h) * 0.03)); // 约 3% 宽

        // 网格总宽高：GRID_SIZE 个 tile + (GRID_SIZE+1) 个 gap
        // 反推 tile 尺寸（浮点避免累计误差）
        const gridTotalW = w;
        const gridTotalH = h;
        const tileSizeW = (gridTotalW - (this.GRID_SIZE + 1) * this.TILE_GAP) / this.GRID_SIZE;
        const tileSizeH = (gridTotalH - (this.GRID_SIZE + 1) * this.TILE_GAP) / this.GRID_SIZE;

        // 取正方形 tile，使用较小者以确保不超界
        this.TILE_SIZE = Math.floor(Math.min(tileSizeW, tileSizeH));

        // 以最终 tileSize 反推“实际占用”的网格宽高
        const actualGridW = this.GRID_SIZE * this.TILE_SIZE + (this.GRID_SIZE + 1) * this.TILE_GAP;
        const actualGridH = this.GRID_SIZE * this.TILE_SIZE + (this.GRID_SIZE + 1) * this.TILE_GAP;

        // 居中起点（四舍五入避免半像素导致渲染模糊）
        this.GRID_ORIGIN_X = Math.round((w - actualGridW) / 2);
        this.GRID_ORIGIN_Y = Math.round((h - actualGridH) / 2);
    }


    drawGrid() {
        if (!this.app) return;
        this.app.stage.removeChildren();

        for (let row = 0; row < this.GRID_SIZE; row++) {
            for (let col = 0; col < this.GRID_SIZE; col++) {
                const { x, y } = this.positionToXY(col, row);
                const cell = new Graphics()
                    .roundRect(0, 0, this.TILE_SIZE, this.TILE_SIZE, 6) // 画圆角矩形
                    .fill(0xcdc1b4); // 填充颜色
                cell.x = x; cell.y = y;
                this.app.stage.addChild(cell);
            }
        }
    }

    // tiles: [{id,value,x,y, previousPosition?, mergedFrom?, isNew?}, ...]
    actuate(tiles) {
        if (!this.app) return;
        this.isAnimating = true;
        this.drawGrid();
        this.tiles.clear();

        const spritesById = new Map();

        // 1) 先渲染在旧位置（无 previousPosition 则渲染在新位置）
        for (const t of tiles) {
            const start = t.previousPosition ?? { x: t.x, y: t.y };
            const sprite = this.makeTileSprite(t.value);
            const startXY = this.positionToXY(start.x, start.y);
            sprite.x = startXY.x;
            sprite.y = startXY.y;
            if (t.isNew && sprite._inner) sprite._inner.scale.set(0); // 在内层做缩放
            this.app.stage.addChild(sprite);
            spritesById.set(t.id, sprite);
            this.tiles.set(t.id, sprite);
        }

        // 2) 下一帧：滑动到新位置；合并来源收拢；目标弹一下；新生弹入
        requestAnimationFrame(() => {
            for (const t of tiles) {
                const spr = spritesById.get(t.id);
                if (!spr) continue;

                this.tweenMove(
                    spr,
                    (t.previousPosition ?? { x: t.x, y: t.y }),
                    { x: t.x, y: t.y },
                    this.animationDuration
                );

                if (t.mergedFrom) {
                    for (const src of t.mergedFrom) {
                        const s = this.makeTileSprite(src.value);
                        const fromXY = this.positionToXY(src.previousPosition.x, src.previousPosition.y);
                        s.x = fromXY.x; s.y = fromXY.y;
                        this.app.stage.addChild(s);
                        this.tweenMove(
                            s, src.previousPosition, { x: t.x, y: t.y }, this.animationDuration, () => s.destroy()
                        );
                    }
                    this.popOnce(spr, this.animationDuration * 0.6);
                }

                if (t.isNew) this.scaleIn(spr, this.animationDuration * 0.8);
            }

            setTimeout(() => { this.isAnimating = false; }, this.animationDuration + 20);
        });
    }

    drawBoardRaw(tiles) {
        if (!this.app) return;
        this.drawGrid();
        for (const t of tiles) {
            const s = this.makeTileSprite(t.value);
            const { x, y } = this.positionToXY(t.x, t.y);
            s.x = x; s.y = y;
            this.app.stage.addChild(s);
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

        const bg = new Graphics()
            .roundRect(0, 0, this.TILE_SIZE, this.TILE_SIZE, 6) // 画圆角矩形
            .fill({ color: this.TILE_COLORS[value] ?? 0xcdc1b4 }); // 填充颜色

        const fontSize =
            value >= 1000 ? Math.floor(this.TILE_SIZE * 0.4) :
                value >= 100 ? Math.floor(this.TILE_SIZE * 0.5) :
                    Math.floor(this.TILE_SIZE * 0.6);

        const txt = new Text({
            text: String(value),
            style: {
                fontFamily: 'Segoe UI, Helvetica Neue, Arial, sans-serif',
                fontSize,
                fill: this.TILE_TEXT_COLORS[value] ?? 0x776e65,
                fontWeight: 'bold'
            }
        });
        // 用本地边界做 pivot，严格几何居中
        txt.updateText?.(); // v8可选，确保几何已生成
        const b = txt.getLocalBounds();              // {x,y,width,height}
        txt.pivot.set(b.x + b.width / 2, b.y + b.height / 2);
        txt.position.set(this.TILE_SIZE / 2, this.TILE_SIZE / 2);
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
        return { x: Math.round(x), y: Math.round(y) };
    }

    tweenMove(g, from, to, duration = 150, onEnd) {
        const start = performance.now();
        const startXY = this.positionToXY(from.x, from.y);
        const endXY = this.positionToXY(to.x, to.y);
        const step = (now) => {
            const t = Math.min(1, (now - start) / duration);
            const p = 1 - Math.pow(1 - t, 3);
            g.x = startXY.x + (endXY.x - startXY.x) * p;
            g.y = startXY.y + (endXY.y - startXY.y) * p;
            if (t < 1) requestAnimationFrame(step);
            else if (onEnd) onEnd();
        };
        requestAnimationFrame(step);
    }

    scaleIn(g, duration = 120) {
        const target = g._inner ?? g;
        const start = performance.now();
        const step = (now) => {
            const t = Math.min(1, (now - start) / duration);
            const c1 = 1.70158, c3 = c1 + 1;
            const p = 1 + c3 * Math.pow(t - 1, 3) + c1 * Math.pow(t - 1, 2); // easeOutBack
            target.scale.set(p);
            if (t < 1) requestAnimationFrame(step);
        };
        requestAnimationFrame(step);
    }

    popOnce(g, duration = 100) {
        const target = g._inner ?? g;
        const from = 1, mid = 1.12, to = 1;
        const start = performance.now();
        const step = (now) => {
            const t = Math.min(1, (now - start) / duration);
            const ease = (x) => 1 - Math.pow(1 - x, 3);
            const s = t < .5 ? from + (mid - from) * ease(t * 2)
                : mid + (to - mid) * ease((t - .5) * 2);
            target.scale.set(s);
            if (t < 1) requestAnimationFrame(step);
        };
        requestAnimationFrame(step);
    }

    handleResize() { this.calculateCanvasSize(); this.drawGrid(); }
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
