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
    
    // 请求通知权限（在游戏胜利时使用）
    if ('Notification' in window && Notification.permission === 'default') {
        // 延迟请求权限，避免在页面加载时立即弹出
        setTimeout(() => {
            Notification.requestPermission();
        }, 5000);
    }
}

// 发送游戏胜利通知
function sendWinNotification(score) {
    if ('Notification' in window && Notification.permission === 'granted') {
        new Notification('🎉 恭喜！', {
            body: `你获得了 ${score} 分！`,
            icon: '/icons/icon-192x192.png',
            badge: '/icons/icon-72x72.png',
            tag: 'game-won'
        });
    }
}

// Initialize game
async function initGame() {
    game = new GameCore();
    await game.init();
    
    // 初始化 PWA 功能
    initPWA();
    
    // 监听游戏胜利事件
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
