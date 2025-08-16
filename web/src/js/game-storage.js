// 游戏存储管理器
export class GameStorage {
    constructor() {
        this.STORAGE_KEY = 'rusty2048_game_state';
        this.SETTINGS_KEY = 'rusty2048_settings';
    }

    // 保存游戏状态
    saveGameState(gameState) {
        try {
            // 确保board是普通数组
            const boardArray = Array.isArray(gameState.board) ? gameState.board : Array.from(gameState.board);
            
            const stateToSave = {
                board: boardArray,
                score: gameState.score,
                moves: gameState.moves,
                state: gameState.state,
                timestamp: Date.now(),
                version: '1.0.0' // 用于版本兼容性
            };
            
            localStorage.setItem(this.STORAGE_KEY, JSON.stringify(stateToSave));
            return true;
        } catch (error) {
            console.error('保存游戏状态失败:', error);
            return false;
        }
    }

    // 加载游戏状态
    loadGameState() {
        try {
            const savedState = localStorage.getItem(this.STORAGE_KEY);
            if (!savedState) {
                return null;
            }

            const parsedState = JSON.parse(savedState);
            
            // 检查版本兼容性
            if (!parsedState.version || parsedState.version !== '1.0.0') {
                this.clearGameState();
                return null;
            }

            // 检查数据完整性
            if (!parsedState.board || !parsedState.score || 
                typeof parsedState.moves !== 'number' || !parsedState.state) {
                this.clearGameState();
                return null;
            }

            // 确保board是数组格式
            if (!Array.isArray(parsedState.board)) {
                this.clearGameState();
                return null;
            }

            return parsedState;
        } catch (error) {
            console.error('加载游戏状态失败:', error);
            this.clearGameState();
            return null;
        }
    }

    // 清除游戏状态
    clearGameState() {
        try {
            localStorage.removeItem(this.STORAGE_KEY);
        } catch (error) {
            console.error('清除游戏状态失败:', error);
        }
    }

    // 保存设置
    saveSettings(settings) {
        try {
            const settingsToSave = {
                language: settings.language,
                theme: settings.theme,
                timestamp: Date.now(),
                version: '1.0.0'
            };
            
            localStorage.setItem(this.SETTINGS_KEY, JSON.stringify(settingsToSave));
            return true;
        } catch (error) {
            console.error('保存设置失败:', error);
            return false;
        }
    }

    // 加载设置
    loadSettings() {
        try {
            const savedSettings = localStorage.getItem(this.SETTINGS_KEY);
            if (!savedSettings) {
                return null;
            }

            const parsedSettings = JSON.parse(savedSettings);
            
            // 检查版本兼容性
            if (!parsedSettings.version || parsedSettings.version !== '1.0.0') {
                this.clearSettings();
                return null;
            }

            return parsedSettings;
        } catch (error) {
            console.error('加载设置失败:', error);
            this.clearSettings();
            return null;
        }
    }

    // 清除设置
    clearSettings() {
        try {
            localStorage.removeItem(this.SETTINGS_KEY);
        } catch (error) {
            console.error('清除设置失败:', error);
        }
    }

    // 检查是否有保存的游戏
    hasSavedGame() {
        return localStorage.getItem(this.STORAGE_KEY) !== null;
    }

    // 获取保存的游戏信息
    getSavedGameInfo() {
        try {
            const savedState = localStorage.getItem(this.STORAGE_KEY);
            if (!savedState) {
                return null;
            }

            const parsedState = JSON.parse(savedState);
            return {
                score: parsedState.score?.current || 0,
                bestScore: parsedState.score?.best || 0,
                moves: parsedState.moves || 0,
                state: parsedState.state || 'playing',
                timestamp: parsedState.timestamp || 0,
                savedDate: new Date(parsedState.timestamp || 0).toLocaleString()
            };
        } catch (error) {
            console.error('获取保存的游戏信息失败:', error);
            return null;
        }
    }

    // 自动保存（用于定期保存）
    autoSave(gameState) {
        // 只在游戏进行中时保存
        if (gameState.state === 'playing' || gameState.state === 'won') {
            this.saveGameState(gameState);
        }
    }


}
