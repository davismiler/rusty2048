export class UIManager {
    constructor(gameInstance, animationManager) {
        this.game = gameInstance;
        this.animationManager = animationManager;
        this.previousScore = 0;
        this.previousBest = 0;
        
        // Language management
        this.currentLanguage = 'en';
        
        // Theme management
        this.currentTheme = 'Classic';
    }

    // ===== Basic UI Updates =====
    updateDisplay() {
        this.updateGrid();
        this.updateStats();
        this.updateMessage();
        this.updateUndoButton();
    }

    updateGrid() {
        // Use Promise to get board asynchronously without blocking animation
        this.game.get_board().then(board => {
            if (!board) return;
            
            const tiles = [];
            for (let i = 0; i < 16; i++) {
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
            this.game.canvasManager.drawBoardRaw(tiles);
            this.game.previousBoard = board;
        });
    }

    updateStats() {
        // Use Promise to get score asynchronously without blocking animation
        this.game.get_score().then(score => {
            const scoreElement = document.getElementById('score');
            const bestElement = document.getElementById('best');
            const movesElement = document.getElementById('moves');

            if (!scoreElement || !bestElement || !movesElement) return;

            // Check if score increased
            if (score.current > this.previousScore) {
                this.animationManager.addAnimationClass(scoreElement, 'score-animation');
            }

            // Check if best score increased
            if (score.best > this.previousBest) {
                this.animationManager.addAnimationClass(bestElement, 'score-animation');
            }

            scoreElement.textContent = score.current;
            bestElement.textContent = score.best;
            movesElement.textContent = this.game.get_moves();

            // Save current scores
            this.previousScore = score.current;
            this.previousBest = score.best;
        });
    }

    updateMessage() {
        // Use Promise to get state asynchronously without blocking animation
        this.game.get_state().then(state => {
            const messageEl = document.getElementById('message');

            if (!messageEl) return;

            messageEl.style.display = 'none';
            messageEl.className = 'message';

            if (state === 'won') {
                messageEl.textContent = '🎉 Congratulations! You won!';
                messageEl.classList.add('won');
                messageEl.style.display = 'block';
                this.animationManager.addAnimationClass(messageEl, 'win-animation');
            } else if (state === 'game_over') {
                messageEl.textContent = '💀 Game Over!';
                messageEl.classList.add('game-over');
                messageEl.style.display = 'block';
                this.animationManager.addAnimationClass(messageEl, 'game-over-animation');
            }
        });
    }

    updateUndoButton() {
        const undoBtn = document.getElementById('undo');
        if (undoBtn) {
            undoBtn.disabled = false;
        }
    }

    // ===== Language Management =====
    async toggleLanguage() {
        const languages = ['en', 'zh'];
        const currentIndex = languages.indexOf(this.currentLanguage);
        const nextIndex = (currentIndex + 1) % languages.length;
        const newLanguage = languages[nextIndex];

        await this.game.set_language(newLanguage);
        this.currentLanguage = newLanguage;

        this.updateLanguageDisplay();
        this.updateTranslations();
        
        // Save settings
        this.game.saveSettings();
    }

    updateLanguageDisplay() {
        // 更新侧边菜单中的语言切换按钮
        const languageToggleMenu = document.getElementById('languageToggleMenu');
        if (languageToggleMenu) {
            languageToggleMenu.textContent = this.currentLanguage === 'zh' ? 'English' : '中文';
        }
    }

    updateTranslations() {
        // Update stat labels
        const scoreLabel = document.querySelector('.stat-box:nth-child(1) .stat-label');
        const bestLabel = document.querySelector('.stat-box:nth-child(2) .stat-label');
        const movesLabel = document.querySelector('.stat-box:nth-child(3) .stat-label');
        
        if (scoreLabel) scoreLabel.textContent = this.game.get_translation('score');
        if (bestLabel) bestLabel.textContent = this.game.get_translation('best');
        if (movesLabel) movesLabel.textContent = this.game.get_translation('moves');

        // Update button texts
        const newGameBtn = document.getElementById('newGame');
        const undoBtn = document.getElementById('undo');
        
        if (newGameBtn) newGameBtn.textContent = this.game.get_translation('new_game');
        if (undoBtn) undoBtn.textContent = this.game.get_translation('undo');
        
        // Update side menu button texts
        const languageToggleMenu = document.getElementById('languageToggleMenu');
        
        if (languageToggleMenu) {
            languageToggleMenu.textContent = this.currentLanguage === 'zh' ? 'English' : '中文';
        }

        // Update instructions
        const instructions = document.querySelector('.instructions');
        if (instructions) {
            if (this.currentLanguage === 'zh') {
                instructions.textContent = '使用方向键、鼠标拖拽或滑动来移动瓦片。合并瓦片以达到2048！';
            } else {
                instructions.textContent = 'Use arrow keys, mouse drag, or swipe to move tiles. Combine tiles to reach 2048!';
            }
        }

        // Update confirm dialog texts
        const confirmTitle = document.getElementById('confirmTitle');
        const confirmMessage = document.getElementById('confirmMessage');
        const cancelBtn = document.getElementById('cancelNewGame');
        const confirmBtn = document.getElementById('confirmNewGame');
        
        if (confirmTitle && confirmMessage && cancelBtn && confirmBtn) {
            if (this.currentLanguage === 'zh') {
                confirmTitle.textContent = '确认开始新游戏？';
                confirmMessage.textContent = '当前游戏进度将会丢失。';
                cancelBtn.textContent = '取消';
                confirmBtn.textContent = '确认';
            } else {
                confirmTitle.textContent = 'Start New Game?';
                confirmMessage.textContent = 'Current game progress will be lost.';
                cancelBtn.textContent = 'Cancel';
                confirmBtn.textContent = 'Confirm';
            }
        }
    }

    getCurrentLanguage() {
        return this.currentLanguage;
    }

    setCurrentLanguage(language) {
        this.currentLanguage = language;
    }

    // ===== Theme Management =====
    async applyTheme(themeName) {
        await this.game.set_theme(themeName);
        this.currentTheme = themeName;

        // Update theme buttons (both main interface and side menu)
        const allThemeBtns = document.querySelectorAll('.theme-btn');
        const activeThemeBtns = document.querySelectorAll(`[data-theme="${themeName}"]`);
        
        allThemeBtns.forEach(btn => {
            if (btn) btn.classList.remove('active');
        });
        activeThemeBtns.forEach(btn => {
            if (btn) btn.classList.add('active');
        });

        // Apply theme colors
        const theme = this.game.get_theme();
        document.body.style.backgroundColor = theme.background;
        
        const titleElement = document.querySelector('.title');
        const instructionsElement = document.querySelector('.instructions');
        
        if (titleElement) titleElement.style.color = theme.title_color;
        if (instructionsElement) instructionsElement.style.color = theme.text_color;

        // Update stat boxes
        const statBoxes = document.querySelectorAll('.stat-box');
        statBoxes.forEach(box => {
            if (box) {
                box.style.backgroundColor = theme.grid_background;
            }
        });

        // Update tiles
        this.updateTileColors(theme);
        
        // Save settings
        this.game.saveSettings();
    }

    updateTileColors(theme) {
        // Update tile color configuration
        this.game.canvasManager.updateTileColors(theme);

        // Redraw the board
        if (this.game.previousBoard) {
            const tiles = this.boardToTiles(this.game.previousBoard);
            this.game.canvasManager.drawBoardRaw(tiles);
        }
    }

    getCurrentTheme() {
        return this.currentTheme;
    }

    // ===== Utility Methods =====
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

    // Check if board has changed
    hasBoardChanged(oldBoard, newBoard) {
        if (!oldBoard || !newBoard) return false;

        for (let i = 0; i < oldBoard.length; i++) {
            if (oldBoard[i] !== newBoard[i]) {
                return true;
            }
        }
        return false;
    }
}
