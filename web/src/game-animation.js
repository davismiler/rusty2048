export class AnimationManager {
    constructor() {
        this._animIdSeq = 1; // 给动画里的 tile 分配临时唯一ID
    }

    // 添加动画类 - 使用CSS动画自动清理
    addAnimationClass(element, className) {
        // 先移除可能存在的动画类，确保动画重新开始
        element.classList.remove(className);
        // 强制重排，确保动画重新触发
        element.offsetHeight;
        // 添加动画类
        element.classList.add(className);
        
        // 使用一次性事件监听器自动清理
        element.addEventListener('animationend', () => {
            element.classList.remove(className);
        }, { once: true });
    }
    
    // 淡出元素 - 使用CSS过渡自动清理
    fadeOutElement(element) {
        element.style.opacity = '0';
        
        // 使用一次性事件监听器自动隐藏
        element.addEventListener('transitionend', () => {
            element.style.display = 'none';
        }, { once: true });
    }

    // 从 oldBoard + direction + newBoard 生成动画 tiles
    buildAnimationFromBoards(oldBoard, newBoard, direction) {
        // 1) 把一维数组转 4x4
        const oldGrid = Array.from({ length: 4 }, (_, r) => oldBoard.slice(r * 4, r * 4 + 4));
        const newGrid = Array.from({ length: 4 }, (_, r) => newBoard.slice(r * 4, r * 4 + 4));

        // 2) 做一次"按方向"的压缩+合并模拟，以获得每个旧格子移动到的新位置，以及是否合并
        const moves = this._planMoves(oldGrid, direction);

        // 3) 把 newGrid 遍历为最终 tile 列表
        const animTiles = [];
        const usedTargets = new Set();

        for (let y = 0; y < 4; y++) {
            for (let x = 0; x < 4; x++) {
                const val = newGrid[y][x];
                if (val === 0) continue;

                const key = `${x},${y}`;
                const mv = moves.byTarget.get(key);

                if (mv) {
                    usedTargets.add(key);
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
                            x, y,
                            previousPosition: { x: s.x, y: s.y }
                        }));
                        const near = this._pickNearSource(mergedFrom, { x, y });
                        animTiles.push({
                            id: this._animIdSeq++,
                            value: val, x, y,
                            previousPosition: { x: near.previousPosition.x, y: near.previousPosition.y },
                            mergedFrom
                        });
                    }
                } else {
                    // 新生
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

    // 以 oldGrid 和方向，做一轮"压缩+合并"规划，输出所有终点的来源集合
    _planMoves(grid4x4, direction) {
        const byTarget = new Map();

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

            // 把 result 映射回目标坐标
            for (let k = 0; k < result.length; k++) {
                if (direction === 'left') { const y = li; const x = k; putTarget(x, y, ...result[k].froms); }
                else if (direction === 'right') { const y = li; const x = 3 - k; putTarget(x, y, ...result[k].froms); }
                else if (direction === 'up') { const x = li; const y = k; putTarget(x, y, ...result[k].froms); }
                else if (direction === 'down') { const x = li; const y = 3 - k; putTarget(x, y, ...result[k].froms); }
            }
        }

        return { byTarget };
    }
}
