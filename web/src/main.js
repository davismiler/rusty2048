import { GameCore } from './game-core.js';

// Game instance
let game;

// Initialize game
async function initGame() {
    game = new GameCore();
    await game.init();
}

// Start game when page loads
document.addEventListener('DOMContentLoaded', initGame);
