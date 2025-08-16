export class EventManager {
    constructor(gameInstance, canvasManager, animationManager) {
        this.game = gameInstance;
        this.canvasManager = canvasManager;
        this.animationManager = animationManager;
        
        // Mouse drag related state
        this.isDragging = false;
        this.startX = null;
        this.startY = null;
        this.dragThreshold = 30;
        
        // Touch related state
        this.touchStartX = null;
        this.touchStartY = null;
    }

    setupEventListeners() {
        this.setupKeyboardControls();
        this.setupMouseControls();
        this.setupButtonControls();
        this.setupTouchControls();
        this.setupResizeHandler();
    }

    setupKeyboardControls() {
        document.addEventListener('keydown', async (e) => {
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
                // Quick check game state without blocking
                this.game.get_state().then(state => {
                    if (state === 'playing') {
                        // Async move processing without blocking keyboard events
                        this.game.handleMove(direction);
                    }
                });
            }
        });
    }

    setupMouseControls() {
        const canvasElement = this.canvasManager.getCanvas();
        const dragHint = document.querySelector('.drag-hint');

        if (!canvasElement) return;

        canvasElement.addEventListener('mousedown', (e) => {
            // Use Promise to get state asynchronously without blocking
            this.game.get_state().then(state => {
                if (state !== 'playing') return;

                this.isDragging = true;
                this.startX = e.clientX;
                this.startY = e.clientY;

                canvasElement.style.cursor = 'grabbing';
                canvasElement.style.userSelect = 'none';

                if (dragHint) {
                    dragHint.style.display = 'block';
                    dragHint.style.opacity = '1';
                }
            });
        });

        document.addEventListener('mousemove', (e) => {
            if (!this.isDragging) return;

            const deltaX = e.clientX - this.startX;
            const deltaY = e.clientY - this.startY;
            const distance = Math.sqrt(deltaX * deltaX + deltaY * deltaY);

            if (distance > this.dragThreshold) {
                let direction = null;

                if (Math.abs(deltaX) > Math.abs(deltaY)) {
                    direction = deltaX > 0 ? 'right' : 'left';
                } else {
                    direction = deltaY > 0 ? 'down' : 'up';
                }

                if (direction) {
                    // Quick check game state without blocking
                    this.game.get_state().then(state => {
                        if (state === 'playing') {
                            // Async move processing without blocking mouse events
                            this.game.handleMove(direction);
                        }
                    });
                }

                this.resetDragState(canvasElement, dragHint);
            }
        });

        document.addEventListener('mouseup', () => {
            if (this.isDragging) {
                this.resetDragState(canvasElement, dragHint);
            }
        });

        document.addEventListener('mouseleave', () => {
            if (this.isDragging) {
                this.resetDragState(canvasElement, dragHint);
            }
        });

        canvasElement.style.cursor = 'grab';
    }

    resetDragState(canvasElement, dragHint) {
        this.isDragging = false;
        canvasElement.style.cursor = 'grab';
        canvasElement.style.userSelect = 'auto';

        if (dragHint) {
            this.animationManager.fadeOutElement(dragHint);
        }
    }

    setupButtonControls() {
        document.getElementById('newGame').addEventListener('click', async () => {
            await this.game.handleNewGame();
        });

        document.getElementById('undo').addEventListener('click', async () => {
            await this.game.undo();
            await this.game.updateDisplay();
        });

        document.getElementById('languageToggle').addEventListener('click', async () => {
            await this.game.toggleLanguage();
        });

        document.querySelectorAll('.theme-btn').forEach(btn => {
            btn.addEventListener('click', async () => {
                const themeName = btn.getAttribute('data-theme');
                await this.game.applyTheme(themeName);
            });
        });
    }

    setupTouchControls() {
        document.addEventListener('touchstart', (e) => {
            this.touchStartX = e.touches[0].clientX;
            this.touchStartY = e.touches[0].clientY;
        });

        document.addEventListener('touchend', (e) => {
            if (!this.touchStartX || !this.touchStartY) return;

            const endX = e.changedTouches[0].clientX;
            const endY = e.changedTouches[0].clientY;

            const diffX = this.touchStartX - endX;
            const diffY = this.touchStartY - endY;

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
                // Quick check game state without blocking
                this.game.get_state().then(state => {
                    if (state === 'playing') {
                        // Async move processing without blocking touch events
                        this.game.handleMove(direction);
                    }
                });
            }

            this.touchStartX = this.touchStartY = null;
        });
    }

    setupResizeHandler() {
        window.addEventListener('resize', () => {
            this.canvasManager.handleResize();
            if (this.game.previousBoard) {
                const tiles = this.game.uiManager.boardToTiles(this.game.previousBoard);
                this.canvasManager.drawBoardRaw(tiles);
            }
        });
    }
}
