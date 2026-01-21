import { SPRITES } from './sprites.js';

export class PetRenderer {
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        this.ctx = this.canvas.getContext('2d');
        this.ctx.imageSmoothingEnabled = false;

        this.currentAnimation = 'idle';
        this.currentFrame = 0;
        this.frameTimer = 0;
        this.frameDelay = 200; // ms between frames
        this.lastFrameTime = 0;

        this.running = false;

        // Load sprite images (placeholder for now)
        this.sprites = {};
        this.loadSprites();
    }

    loadSprites() {
        // For now, we'll draw simple geometric shapes
        // Later we'll load actual pixel art sprite sheets
        this.spritesLoaded = true;
    }

    start(renderCallback) {
        this.running = true;
        this.renderCallback = renderCallback;
        this.lastFrameTime = performance.now();
        this.loop();
    }

    stop() {
        this.running = false;
    }

    loop(currentTime = performance.now()) {
        if (!this.running) return;

        const deltaTime = currentTime - this.lastFrameTime;
        this.lastFrameTime = currentTime;

        // Update frame timing
        this.frameTimer += deltaTime;
        if (this.frameTimer >= this.frameDelay) {
            this.frameTimer = 0;
            this.currentFrame = (this.currentFrame + 1) % 4; // 4 frames for now
        }

        // Call render callback
        if (this.renderCallback) {
            this.renderCallback(deltaTime);
        }

        requestAnimationFrame((time) => this.loop(time));
    }

    render(petState) {
        // Clear canvas with off-white background (Tamagotchi screen color)
        this.ctx.fillStyle = '#c8d4c0';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

        // Draw pixel grid effect (optional, subtle)
        this.drawPixelGrid();

        // Draw simple placeholder pet (will be replaced with actual sprites)
        this.drawPlaceholderPet(petState);
    }

    drawPixelGrid() {
        // Very subtle pixel grid for retro effect
        this.ctx.strokeStyle = 'rgba(0, 0, 0, 0.03)';
        this.ctx.lineWidth = 1;
        const gridSize = 8;

        for (let x = 0; x < this.canvas.width; x += gridSize) {
            this.ctx.beginPath();
            this.ctx.moveTo(x, 0);
            this.ctx.lineTo(x, this.canvas.height);
            this.ctx.stroke();
        }

        for (let y = 0; y < this.canvas.height; y += gridSize) {
            this.ctx.beginPath();
            this.ctx.moveTo(0, y);
            this.ctx.lineTo(this.canvas.width, y);
            this.ctx.stroke();
        }
    }

    drawPlaceholderPet(petState) {
        const x = 64;
        const y = 64;

        // Draw based on stage
        switch (petState.stage) {
            case 'Egg':
                this.drawEgg(x, y);
                break;
            case 'Baby':
                this.drawBaby(x, y);
                break;
            case 'Child':
            case 'Teen':  // Support old Teen stage
                this.drawChild(x, y);
                break;
            case 'Adult':
                this.drawAdult(x, y);
                break;
            default:
                this.drawEgg(x, y);
        }
    }

    drawEgg(x, y) {
        const pixelSize = 4;
        const bounce = Math.floor(Math.sin(this.currentFrame * 0.3) * 0.5) * pixelSize;

        // Pixelated egg shape (simple oval)
        this.ctx.fillStyle = '#000';

        // Draw pixel by pixel for retro look
        const eggPixels = [
            [0,2],[0,3],[0,4],[0,5],  // left edge
            [1,1],[1,2],[1,3],[1,4],[1,5],[1,6],
            [2,0],[2,1],[2,2],[2,3],[2,4],[2,5],[2,6],[2,7],
            [3,0],[3,1],[3,2],[3,3],[3,4],[3,5],[3,6],[3,7],
            [4,1],[4,2],[4,3],[4,4],[4,5],[4,6],
            [5,2],[5,3],[5,4],[5,5]   // right edge
        ];

        eggPixels.forEach(([px, py]) => {
            this.ctx.fillRect(
                x - 12 + px * pixelSize,
                y - 16 + py * pixelSize + bounce,
                pixelSize,
                pixelSize
            );
        });
    }

    drawBaby(x, y) {
        const pixelSize = 4;
        const bounce = Math.floor(Math.sin(this.currentFrame * 0.5) * 1) * pixelSize;

        this.ctx.fillStyle = '#000';

        // Simple baby body
        const babyPixels = [
            // Body (rounded blob)
            [1,1],[1,2],[1,3],[1,4],[1,5],[1,6],
            [2,0],[2,1],[2,2],[2,3],[2,4],[2,5],[2,6],[2,7],
            [3,0],[3,1],[3,2],[3,3],[3,4],[3,5],[3,6],[3,7],
            [4,1],[4,2],[4,3],[4,4],[4,5],[4,6]
        ];

        babyPixels.forEach(([px, py]) => {
            this.ctx.fillRect(
                x - 10 + px * pixelSize,
                y - 14 + py * pixelSize + bounce,
                pixelSize,
                pixelSize
            );
        });

        // Eyes (white background then black pupils)
        this.ctx.fillStyle = '#c8d4c0';
        this.ctx.fillRect(x - 6, y - 6 + bounce, pixelSize, pixelSize);
        this.ctx.fillRect(x + 2, y - 6 + bounce, pixelSize, pixelSize);

        this.ctx.fillStyle = '#000';
        this.ctx.fillRect(x - 4, y - 4 + bounce, pixelSize/2, pixelSize/2);
        this.ctx.fillRect(x + 4, y - 4 + bounce, pixelSize/2, pixelSize/2);
    }

    drawChild(x, y) {
        const pixelSize = 4;
        const bounce = Math.floor(Math.sin(this.currentFrame * 0.5) * 1.5) * pixelSize;

        this.ctx.fillStyle = '#000';

        // Child body (larger with small ears)
        const childPixels = [
            // Ears
            [0,2],[0,3],
            [6,2],[6,3],
            // Head/Body
            [1,1],[1,2],[1,3],[1,4],[1,5],[1,6],[1,7],
            [2,0],[2,1],[2,2],[2,3],[2,4],[2,5],[2,6],[2,7],[2,8],
            [3,0],[3,1],[3,2],[3,3],[3,4],[3,5],[3,6],[3,7],[3,8],
            [4,0],[4,1],[4,2],[4,3],[4,4],[4,5],[4,6],[4,7],[4,8],
            [5,1],[5,2],[5,3],[5,4],[5,5],[5,6],[5,7]
        ];

        childPixels.forEach(([px, py]) => {
            this.ctx.fillRect(
                x - 14 + px * pixelSize,
                y - 18 + py * pixelSize + bounce,
                pixelSize,
                pixelSize
            );
        });

        // Eyes
        this.ctx.fillStyle = '#c8d4c0';
        this.ctx.fillRect(x - 8, y - 8 + bounce, pixelSize, pixelSize);
        this.ctx.fillRect(x + 4, y - 8 + bounce, pixelSize, pixelSize);

        this.ctx.fillStyle = '#000';
        this.ctx.fillRect(x - 6, y - 6 + bounce, pixelSize/2, pixelSize/2);
        this.ctx.fillRect(x + 6, y - 6 + bounce, pixelSize/2, pixelSize/2);
    }

    drawAdult(x, y) {
        const pixelSize = 4;
        const bounce = Math.floor(Math.sin(this.currentFrame * 0.5) * 2) * pixelSize;

        this.ctx.fillStyle = '#000';

        // Adult body (largest with prominent ears)
        const adultPixels = [
            // Left ear
            [0,3],[0,4],[0,5],
            [1,2],[1,3],[1,4],
            // Right ear
            [7,2],[7,3],[7,4],
            [8,3],[8,4],[8,5],
            // Head/Body
            [1,5],[1,6],[1,7],[1,8],[1,9],
            [2,1],[2,2],[2,3],[2,4],[2,5],[2,6],[2,7],[2,8],[2,9],[2,10],
            [3,0],[3,1],[3,2],[3,3],[3,4],[3,5],[3,6],[3,7],[3,8],[3,9],[3,10],
            [4,0],[4,1],[4,2],[4,3],[4,4],[4,5],[4,6],[4,7],[4,8],[4,9],[4,10],
            [5,0],[5,1],[5,2],[5,3],[5,4],[5,5],[5,6],[5,7],[5,8],[5,9],[5,10],
            [6,1],[6,2],[6,3],[6,4],[6,5],[6,6],[6,7],[6,8],[6,9],[6,10],
            [7,5],[7,6],[7,7],[7,8],[7,9]
        ];

        adultPixels.forEach(([px, py]) => {
            this.ctx.fillRect(
                x - 18 + px * pixelSize,
                y - 22 + py * pixelSize + bounce,
                pixelSize,
                pixelSize
            );
        });

        // Eyes (larger with white background)
        this.ctx.fillStyle = '#c8d4c0';
        this.ctx.fillRect(x - 10, y - 10 + bounce, pixelSize * 1.5, pixelSize * 1.5);
        this.ctx.fillRect(x + 6, y - 10 + bounce, pixelSize * 1.5, pixelSize * 1.5);

        this.ctx.fillStyle = '#000';
        this.ctx.fillRect(x - 8, y - 8 + bounce, pixelSize, pixelSize);
        this.ctx.fillRect(x + 8, y - 8 + bounce, pixelSize, pixelSize);
    }
}
