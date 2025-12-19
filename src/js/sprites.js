// Sprite definitions and animations
// This will be expanded later with actual sprite sheet data

export const SPRITES = {
    egg: {
        idle: [
            { x: 0, y: 0, width: 32, height: 32 }
        ],
        hatch: [
            { x: 32, y: 0, width: 32, height: 32 },
            { x: 64, y: 0, width: 32, height: 32 },
            { x: 96, y: 0, width: 32, height: 32 }
        ]
    },
    baby: {
        idle: [
            { x: 0, y: 32, width: 32, height: 32 },
            { x: 32, y: 32, width: 32, height: 32 }
        ],
        eating: [
            { x: 64, y: 32, width: 32, height: 32 },
            { x: 96, y: 32, width: 32, height: 32 }
        ],
        playing: [
            { x: 128, y: 32, width: 32, height: 32 },
            { x: 160, y: 32, width: 32, height: 32 }
        ],
        sleeping: [
            { x: 192, y: 32, width: 32, height: 32 }
        ]
    },
    teen: {
        idle: [
            { x: 0, y: 64, width: 32, height: 32 },
            { x: 32, y: 64, width: 32, height: 32 }
        ],
        eating: [
            { x: 64, y: 64, width: 32, height: 32 },
            { x: 96, y: 64, width: 32, height: 32 }
        ],
        playing: [
            { x: 128, y: 64, width: 32, height: 32 },
            { x: 160, y: 64, width: 32, height: 32 }
        ],
        sleeping: [
            { x: 192, y: 64, width: 32, height: 32 }
        ]
    },
    adult: {
        idle: [
            { x: 0, y: 96, width: 32, height: 32 },
            { x: 32, y: 96, width: 32, height: 32 }
        ],
        eating: [
            { x: 64, y: 96, width: 32, height: 32 },
            { x: 96, y: 96, width: 32, height: 32 }
        ],
        playing: [
            { x: 128, y: 96, width: 32, height: 32 },
            { x: 160, y: 96, width: 32, height: 32 }
        ],
        sleeping: [
            { x: 192, y: 96, width: 32, height: 32 }
        ]
    }
};

export function getAnimation(stage, action = 'idle') {
    const stageSprites = SPRITES[stage.toLowerCase()];
    if (!stageSprites) return SPRITES.egg.idle;

    const animation = stageSprites[action];
    if (!animation) return stageSprites.idle;

    return animation;
}
