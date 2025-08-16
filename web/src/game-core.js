import init, { Rusty2048Web, init_panic_hook } from '/public/pkg/rusty2048_web.js';
import { CanvasManager } from './game-canvas.js';
import { AnimationManager } from './game-animation.js';
import { EventManager } from './game-event.js';
import { UIManager } from './game-ui.js';

export class GameCore {
    constructor() {
        this.canvasManager = new CanvasManager();
        this.game = null;
        this.previousBoard = null;
        
        // Initialize managers
        this.animationManager = new AnimationManager();
        this.uiManager = new UIManager(this, this.animationManager);
        this.eventManager = new EventManager(this, this.canvasManager, this.animationManager);
    }

    async init() {
        // Initialize WASM
        await init();
        init_panic_hook();

        // Create game instance
        this.game = new Rusty2048Web();
        this.uiManager.setCurrentLanguage(this.game.get_language());

        // Initialize Canvas manager
        await this.canvasManager.init();

        // Ensure initial two tiles
        await this.game.new_game();

        // Setup event listeners
        this.eventManager.setupEventListeners();

        // Update display
        this.uiManager.updateDisplay();
        this.uiManager.updateLanguageDisplay();

        // Apply default theme
        await this.uiManager.applyTheme('Classic');
    }

    // Handle new game
    async handleNewGame() {
        this.previousBoard = null;
        await this.game.new_game();
        this.uiManager.updateGrid();
        this.uiManager.updateStats();
        this.uiManager.updateMessage();
    }

    async handleMove(direction) {
        const before = await this.game.get_board();
        await this.game.make_move(direction);
        const after = await this.game.get_board();

        if (!this.uiManager.hasBoardChanged(before, after)) return;

        const animTiles = this.animationManager.buildAnimationFromBoards(before, after, direction);
        this.canvasManager.actuate(animTiles);

        // Non-blocking UI updates for smooth animation
        this.uiManager.updateStats();
        this.uiManager.updateMessage();
        this.uiManager.updateUndoButton();

        this.previousBoard = after;
    }

    // Game state access methods
    async get_board() { return await this.game.get_board(); }
    async get_score() { return await this.game.get_score(); }
    async get_state() { return await this.game.get_state(); }
    get_moves() { return this.game.get_moves(); }
    async set_language(language) { return await this.game.set_language(language); }
    get_language() { return this.game.get_language(); }
    get_translation(key) { return this.game.get_translation(key); }
    async set_theme(themeName) { return await this.game.set_theme(themeName); }
    get_theme() { return this.game.get_theme(); }
    async undo() { return await this.game.undo(); }
    async new_game() { return await this.game.new_game(); }
    async make_move(direction) { return await this.game.make_move(direction); }

    // Property accessors
    get currentLanguage() { return this.uiManager.getCurrentLanguage(); }
    set currentLanguage(language) { this.uiManager.setCurrentLanguage(language); }
    get currentTheme() { return this.uiManager.getCurrentTheme(); }

    // Convenience methods
    updateDisplay() { return this.uiManager.updateDisplay(); }
    async applyTheme(themeName) { return await this.uiManager.applyTheme(themeName); }
    async toggleLanguage() { return await this.uiManager.toggleLanguage(); }
}
