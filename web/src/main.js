import init, { Rusty2048Web, init_panic_hook } from '../pkg/rusty2048_web.js';
import { CanvasManager } from './canvas-manager.js';

class Game2048 {
    constructor() {
        this.canvasManager = new CanvasManager();
        this.game = null;
        this.previousBoard = null;
        this.currentTheme = 'Classic';
        this.currentLanguage = 'en';

        this._animIdSeq = 1; // NEW: 给动画里的 tile 分配临时唯一ID
    }

    async init() {
        // 初始化WASM
        await init();
        init_panic_hook();

        // 创建游戏实例
        this.game = new Rusty2048Web();
        this.currentLanguage = this.game.get_language();

        // 初始化Canvas管理器
        await this.canvasManager.init();

        // 保证有初始两枚瓦片
        await this.game.new_game();

        // 设置事件监听
        this.setupEventListeners();

        // 更新显示
        await this.updateDisplay();
        await this.updateGrid();
        await this.updateStats();
        await this.updateMessage();
        this.updateLanguageDisplay();

        // 应用默认主题
        await this.applyTheme('Classic');
    }



    async updateDisplay() {
        await this.updateGrid();
        await this.updateStats();
        await this.updateMessage();
        this.updateUndoButton();
    }

    // 处理新游戏
    async handleNewGame() {
        // 清除之前的棋盘状态
        this.previousBoard = null;

        await this.game.new_game();
        await this.updateGrid();     // 先静态首帧
        await this.updateStats();
        await this.updateMessage();
    }

    async handleMove(direction) {
        const before = await this.game.get_board();
        await this.game.make_move(direction);
        const after = await this.game.get_board();

        if (!this.hasBoardChanged(before, after)) return;

        const animTiles = this.buildAnimationFromBoards(before, after, direction);
        this.canvasManager.actuate(animTiles);

        await this.updateStats();
        await this.updateMessage();
        this.updateUndoButton();

        this.previousBoard = after;
    }

    // 检查棋盘是否发生变化
    hasBoardChanged(oldBoard, newBoard) {
        if (!oldBoard || !newBoard) return false;

        for (let i = 0; i < oldBoard.length; i++) {
            if (oldBoard[i] !== newBoard[i]) {
                return true;
            }
        }
        return false;
    }

    async updateGrid() {
        const board = await this.game.get_board();
        if (!board) return;
        // 把一维 16 长数组转成 tiles，直接画出来（无动画）
        const tiles = [];
        for (let i = 0; i < 16; i++) {
            const v = board[i];
            if (v > 0) {
                tiles.push({
                    id: i + 1,                // 任意稳定 id 即可
                    value: v,
                    x: i % 4,
                    y: Math.floor(i / 4),
                });
            }
        }
        this.canvasManager.drawBoardRaw(tiles);
        this.previousBoard = board;
    }

    // —— 核心：从 oldBoard + direction + newBoard 生成动画 tiles —— //
    buildAnimationFromBoards(oldBoard, newBoard, direction) {
        // 1) 把一维数组转 4x4
        const oldGrid = Array.from({ length: 4 }, (_, r) => oldBoard.slice(r * 4, r * 4 + 4));
        const newGrid = Array.from({ length: 4 }, (_, r) => newBoard.slice(r * 4, r * 4 + 4));

        // 2) 做一次“按方向”的压缩+合并模拟，以获得每个旧格子移动到的新位置，以及是否合并
        //    注意：这只用于生成动画轨迹；最终形态以 newGrid 为准。
        const moves = this._planMoves(oldGrid, direction);

        // moves: {to:{x,y}, froms:[{x,y,value}], merged:boolean} 的终点列表
        // 例如一次行向左：每个非零数会被映射到某个终点；合并的终点 froms 有两个来源。

        // 3) 把 newGrid 遍历为最终 tile 列表：为每个最终格生成 {id,value,x,y,previousPosition?, mergedFrom?, isNew?}
        const animTiles = [];
        const usedTargets = new Set(); // 用于判断 new tile

        for (let y = 0; y < 4; y++) {
            for (let x = 0; x < 4; x++) {
                const val = newGrid[y][x];
                if (val === 0) continue;

                // 在 moves 里找到以 (x,y) 为目标的项（可能没有——那它可能是新生）
                const key = `${x},${y}`;
                const mv = moves.byTarget.get(key);

                if (mv) {
                    usedTargets.add(key);
                    // 目标：值=新值；来源：可能1个（纯位移）或2个（合并）
                    if (mv.froms.length === 1) {
                        const src = mv.froms[0];
                        animTiles.push({
                            id: this._animIdSeq++,
                            value: val, x, y,
                            previousPosition: { x: src.x, y: src.y }
                        });
                    } else {
                        // 合并：两个来源分别从各自 previousPosition 收拢到目标
                        const mergedFrom = mv.froms.map(s => ({
                            id: this._animIdSeq++,
                            value: s.value,
                            x, y, // 目标
                            previousPosition: { x: s.x, y: s.y }
                        }));
                        // 目标 tile 本身也有一个从某来源“移动过来”的 previousPosition（取更近的那个）
                        const near = this._pickNearSource(mergedFrom, { x, y });
                        animTiles.push({
                            id: this._animIdSeq++,
                            value: val, x, y,
                            previousPosition: { x: near.previousPosition.x, y: near.previousPosition.y },
                            mergedFrom
                        });
                    }
                } else {
                    // 在 old 是 0，新是 >0，且不是任何 move 的目标 -> 新生
                    animTiles.push({
                        id: this._animIdSeq++,
                        value: val, x, y,
                        isNew: true
                    });
                }
            }
        }

        return animTiles;
    }

    _pickNearSource(sources, target) {
        let best = sources[0], bd = Infinity;
        for (const s of sources) {
            const d = Math.abs(s.previousPosition.x - target.x) + Math.abs(s.previousPosition.y - target.y);
            if (d < bd) { bd = d; best = s; }
        }
        return best;
    }

    // 以 oldGrid 和方向，做一轮“压缩+合并”规划，输出所有终点的来源集合
    _planMoves(grid4x4, direction) {
        const byTarget = new Map(); // key "x,y" -> {to:{x,y}, froms:[{x,y,value}]}

        const lines = [];
        if (direction === 'left' || direction === 'right') {
            for (let y = 0; y < 4; y++) {
                const line = [];
                for (let x = 0; x < 4; x++) line.push({ x, y, val: grid4x4[y][x] });
                if (direction === 'right') line.reverse();
                lines.push(line);
            }
        } else {
            for (let x = 0; x < 4; x++) {
                const line = [];
                for (let y = 0; y < 4; y++) line.push({ x, y, val: grid4x4[y][x] });
                if (direction === 'down') line.reverse();
                lines.push(line);
            }
        }

        const putTarget = (toX, toY, from) => {
            const key = `${toX},${toY}`;
            if (!byTarget.has(key)) byTarget.set(key, { to: { x: toX, y: toY }, froms: [] });
            byTarget.get(key).froms.push(from);
        };

        for (let li = 0; li < lines.length; li++) {
            const line = lines[li].filter(c => c.val !== 0);
            const result = [];
            let idx = 0;
            while (idx < line.length) {
                const a = line[idx];
                if (idx + 1 < line.length && line[idx + 1].val === a.val) {
                    // 合并：a+b -> 2a
                    const b = line[idx + 1];
                    result.push({
                        value: a.val * 2,
                        froms: [{ x: a.x, y: a.y, value: a.val }, { x: b.x, y: b.y, value: b.val }]
                    });
                    idx += 2;
                } else {
                    // 纯位移
                    result.push({
                        value: a.val,
                        froms: [{ x: a.x, y: a.y, value: a.val }]
                    });
                    idx += 1;
                }
            }

            // 把 result 映射回目标坐标（按方向排到 0..）
            for (let k = 0; k < result.length; k++) {
                if (direction === 'left') { const y = li; const x = k; putTarget(x, y, ...result[k].froms); }
                else if (direction === 'right') { const y = li; const x = 3 - k; putTarget(x, y, ...result[k].froms); }
                else if (direction === 'up') { const x = li; const y = k; putTarget(x, y, ...result[k].froms); }
                else if (direction === 'down') { const x = li; const y = 3 - k; putTarget(x, y, ...result[k].froms); }
            }
        }

        return { byTarget };
    }


    async updateStats() {
        const score = await this.game.get_score();
        const scoreElement = document.getElementById('score');
        const bestElement = document.getElementById('best');
        const movesElement = document.getElementById('moves');

        // 检查分数是否增加
        if (score.current > this.previousScore) {
            scoreElement.classList.add('score-animation');
            setTimeout(() => {
                scoreElement.classList.remove('score-animation');
            }, 600);
        }

        // 检查最高分是否增加
        if (score.best > this.previousBest) {
            bestElement.classList.add('score-animation');
            setTimeout(() => {
                bestElement.classList.remove('score-animation');
            }, 600);
        }

        scoreElement.textContent = score.current;
        bestElement.textContent = score.best;
        movesElement.textContent = this.game.get_moves();

        // 保存当前分数
        this.previousScore = score.current;
        this.previousBest = score.best;
    }

    async updateMessage() {
        const messageEl = document.getElementById('message');
        const state = await this.game.get_state();

        messageEl.style.display = 'none';
        messageEl.className = 'message';

        if (state === 'won') {
            messageEl.textContent = '🎉 Congratulations! You won!';
            messageEl.classList.add('won');
            messageEl.style.display = 'block';
            messageEl.classList.add('win-animation');
            setTimeout(() => {
                messageEl.classList.remove('win-animation');
            }, 1000);
        } else if (state === 'game_over') {
            messageEl.textContent = '💀 Game Over!';
            messageEl.classList.add('game-over');
            messageEl.style.display = 'block';
            messageEl.classList.add('game-over-animation');
            setTimeout(() => {
                messageEl.classList.remove('game-over-animation');
            }, 600);
        }
    }

    updateUndoButton() {
        const undoBtn = document.getElementById('undo');
        undoBtn.disabled = false;
    }

    async toggleLanguage() {
        const languages = ['en', 'zh'];
        const currentIndex = languages.indexOf(this.currentLanguage);
        const nextIndex = (currentIndex + 1) % languages.length;
        const newLanguage = languages[nextIndex];

        await this.game.set_language(newLanguage);
        this.currentLanguage = newLanguage;

        this.updateLanguageDisplay();
        this.updateTranslations();
    }

    updateLanguageDisplay() {
        const langBtn = document.getElementById('languageToggle');
        const langNames = { 'en': 'English', 'zh': '中文' };
        langBtn.textContent = langNames[this.currentLanguage] || 'Language';
    }

    updateTranslations() {
        // Update stat labels
        document.querySelector('.stat-box:nth-child(1) .stat-label').textContent = this.game.get_translation('score');
        document.querySelector('.stat-box:nth-child(2) .stat-label').textContent = this.game.get_translation('best');
        document.querySelector('.stat-box:nth-child(3) .stat-label').textContent = this.game.get_translation('moves');

        // Update button texts
        document.getElementById('newGame').textContent = this.game.get_translation('new_game');
        document.getElementById('undo').textContent = this.game.get_translation('undo');

        // Update instructions
        const instructions = document.querySelector('.instructions');
        if (this.currentLanguage === 'zh') {
            instructions.textContent = '使用方向键、鼠标拖拽或滑动来移动瓦片。合并瓦片以达到2048！';
        } else {
            instructions.textContent = 'Use arrow keys, mouse drag, or swipe to move tiles. Combine tiles to reach 2048!';
        }
    }

    async applyTheme(themeName) {
        try {
            await this.game.set_theme(themeName);
            this.currentTheme = themeName;

            // Update theme buttons
            document.querySelectorAll('.theme-btn').forEach(btn => {
                btn.classList.remove('active');
            });
            document.querySelector(`[data-theme="${themeName}"]`).classList.add('active');

            // Apply theme colors
            const theme = this.game.get_theme();
            document.body.style.backgroundColor = theme.background;
            document.querySelector('.title').style.color = theme.title_color;
            document.querySelector('.instructions').style.color = theme.text_color;

            // Update stat boxes
            document.querySelectorAll('.stat-box').forEach(box => {
                box.style.backgroundColor = theme.grid_background;
            });

            // Update tiles
            this.updateTileColors(theme);
        } catch (error) {
            console.error('Error applying theme:', error);
        }
    }

    updateTileColors(theme) {
        // 更新瓦片颜色配置
        this.canvasManager.updateTileColors(theme);

        // 重新绘制棋盘
        if (this.previousBoard) {
            const tiles = this.boardToTiles(this.previousBoard);
            this.canvasManager.drawBoardRaw(tiles);
        }
    }

    boardToTiles(board) {
        const tiles = [];
        for (let i = 0; i < board.length; i++) {
            const v = board[i];
            if (v > 0) {
                tiles.push({
                    id: i + 1,
                    value: v,
                    x: i % 4,
                    y: Math.floor(i / 4),
                });
            }
        }
        return tiles;
    }

    setupEventListeners() {
        // Keyboard controls
        document.addEventListener('keydown', async (e) => {
            const state = await this.game.get_state();
            if (state !== 'playing' || this.isAnimating) return;

            let direction = null;
            switch (e.key) {
                case 'ArrowUp':
                case 'w':
                case 'W':
                    direction = 'up';
                    break;
                case 'ArrowDown':
                case 's':
                case 'S':
                    direction = 'down';
                    break;
                case 'ArrowLeft':
                case 'a':
                case 'A':
                    direction = 'left';
                    break;
                case 'ArrowRight':
                case 'd':
                case 'D':
                    direction = 'right';
                    break;
            }

            if (direction) {
                e.preventDefault();
                await this.handleMove(direction);
            }
        });

        // Mouse drag controls
        this.setupMouseControls();

        // Button controls
        document.getElementById('newGame').addEventListener('click', async () => {
            await this.game.new_game();
            await this.handleNewGame();
        });

        document.getElementById('undo').addEventListener('click', async () => {
            await this.game.undo();
            await this.updateDisplay();
        });

        // Language toggle
        document.getElementById('languageToggle').addEventListener('click', async () => {
            await this.toggleLanguage();
        });

        // Theme controls
        document.querySelectorAll('.theme-btn').forEach(btn => {
            btn.addEventListener('click', async () => {
                const themeName = btn.getAttribute('data-theme');
                await this.applyTheme(themeName);
            });
        });

        // Touch/swipe support for mobile
        let startX, startY;
        document.addEventListener('touchstart', (e) => {
            startX = e.touches[0].clientX;
            startY = e.touches[0].clientY;
        });

        document.addEventListener('touchend', async (e) => {
            if (!startX || !startY || this.isAnimating) return;

            const endX = e.changedTouches[0].clientX;
            const endY = e.changedTouches[0].clientY;

            const diffX = startX - endX;
            const diffY = startY - endY;

            let direction = null;
            if (Math.abs(diffX) > Math.abs(diffY)) {
                // Horizontal swipe
                if (diffX > 0) {
                    direction = 'left';
                } else {
                    direction = 'right';
                }
            } else {
                // Vertical swipe
                if (diffY > 0) {
                    direction = 'up';
                } else {
                    direction = 'down';
                }
            }

            if (direction) {
                await this.handleMove(direction);
            }

            startX = startY = null;
        });

        // 监听窗口大小变化
        window.addEventListener('resize', () => {
            this.canvasManager.handleResize();
            if (this.previousBoard) {
                const tiles = this.boardToTiles(this.previousBoard);
                this.canvasManager.drawBoardRaw(tiles);
            }
        });
    }

    setupMouseControls() {
        let isDragging = false;
        let startX, startY;
        let dragThreshold = 30;
        let canvasElement = this.canvasManager.getCanvas();
        let dragHint = document.querySelector('.drag-hint');

        if (!canvasElement) return;

        // 鼠标按下事件
        canvasElement.addEventListener('mousedown', (e) => {
            const state = this.game.get_state();
            if (state !== 'playing' || this.canvasManager.isAnimatingNow()) return;

            isDragging = true;
            startX = e.clientX;
            startY = e.clientY;

            canvasElement.style.cursor = 'grabbing';
            canvasElement.style.userSelect = 'none';

            // 显示拖拽提示
            if (dragHint) {
                dragHint.style.display = 'block';
                dragHint.style.opacity = '1';
            }
        });

        // 鼠标移动事件
        document.addEventListener('mousemove', async (e) => {
            if (!isDragging) return;

            const deltaX = e.clientX - startX;
            const deltaY = e.clientY - startY;
            const distance = Math.sqrt(deltaX * deltaX + deltaY * deltaY);

            if (distance > dragThreshold) {
                let direction = null;

                if (Math.abs(deltaX) > Math.abs(deltaY)) {
                    direction = deltaX > 0 ? 'right' : 'left';
                } else {
                    direction = deltaY > 0 ? 'down' : 'up';
                }

                if (direction) {
                    try {
                        await this.handleMove(direction);
                    } catch (error) {
                        console.error('Move failed:', error);
                    }
                }

                isDragging = false;
                canvasElement.style.cursor = 'grab';
                canvasElement.style.userSelect = 'auto';

                // 隐藏拖拽提示
                if (dragHint) {
                    dragHint.style.opacity = '0';
                    setTimeout(() => {
                        dragHint.style.display = 'none';
                    }, 300);
                }
            }
        });

        // 鼠标松开事件
        document.addEventListener('mouseup', () => {
            if (isDragging) {
                isDragging = false;
                canvasElement.style.cursor = 'grab';
                canvasElement.style.userSelect = 'auto';

                // 隐藏拖拽提示
                if (dragHint) {
                    dragHint.style.opacity = '0';
                    setTimeout(() => {
                        dragHint.style.display = 'none';
                    }, 300);
                }
            }
        });

        // 鼠标离开事件
        document.addEventListener('mouseleave', () => {
            if (isDragging) {
                isDragging = false;
                canvasElement.style.cursor = 'grab';
                canvasElement.style.userSelect = 'auto';

                // 隐藏拖拽提示
                if (dragHint) {
                    dragHint.style.opacity = '0';
                    setTimeout(() => {
                        dragHint.style.display = 'none';
                    }, 300);
                }
            }
        });

        canvasElement.style.cursor = 'grab';
    }
}

// 初始化游戏
let gameInstance;

async function initGame() {
    gameInstance = new Game2048();
    await gameInstance.init();
}

// 当页面加载完成时初始化游戏
document.addEventListener('DOMContentLoaded', () => {
    initGame().catch(console.error);
});
