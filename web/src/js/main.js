import { GameCore } from './game-core.js';

// Game instance
let game;

// PWA 相关功能
function initPWA() {
    // 检查是否支持 PWA
    if ('serviceWorker' in navigator) {
        // 监听网络状态
        window.addEventListener('online', () => {
            console.log('网络已连接');
        });
        
        window.addEventListener('offline', () => {
            console.log('网络已断开');
        });
        
        // 检查是否已安装
        if (window.matchMedia('(display-mode: standalone)').matches) {
            console.log('应用已安装');
        }
    }
    
    // 通知权限请求已关闭 - 用户需要手动授权
    // 如果需要通知功能，用户可以在浏览器设置中手动开启
}

// 游戏胜利通知功能已移除
function sendWinNotification(score) {
    // 通知功能已禁用
    console.log(`游戏胜利！得分：${score}`);
}

// Initialize game
async function initGame() {
    game = new GameCore();
    await game.init();
    
    // 初始化 PWA 功能
    initPWA();
    
    // 游戏胜利事件处理
    if (game.onGameWon) {
        const originalOnGameWon = game.onGameWon;
        game.onGameWon = (score) => {
            originalOnGameWon(score);
            sendWinNotification(score);
        };
    }
}

// Start game when page loads
document.addEventListener('DOMContentLoaded', initGame);
