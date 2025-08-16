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
        undoBtn.disabled = false;
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
