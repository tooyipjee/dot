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
        // Clear canvas
        this.ctx.fillStyle = 'rgba(135, 206, 235, 0.3)';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

        // Draw ground
        this.ctx.fillStyle = '#8bc34a';
        this.ctx.fillRect(0, 200, this.canvas.width, 40);

        // Draw simple placeholder pet (will be replaced with actual sprites)
        this.drawPlaceholderPet(petState);
    }

    drawPlaceholderPet(petState) {
        const x = 160;
        const y = 180;

        // Draw based on stage
        switch (petState.stage) {
            case 'Egg':
                this.drawEgg(x, y);
                break;
            case 'Baby':
                this.drawBaby(x, y);
                break;
            case 'Teen':
                this.drawTeen(x, y);
                break;
            case 'Adult':
                this.drawAdult(x, y);
                break;
            default:
                this.drawEgg(x, y);
        }
    }

    drawEgg(x, y) {
        // Simple egg shape
        this.ctx.fillStyle = '#fff';
        this.ctx.strokeStyle = '#333';
        this.ctx.lineWidth = 2;

        this.ctx.beginPath();
        this.ctx.ellipse(x, y, 20, 28, 0, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.stroke();

        // Add some spots
        this.ctx.fillStyle = '#ddd';
        this.ctx.beginPath();
        this.ctx.arc(x - 8, y - 5, 4, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.beginPath();
        this.ctx.arc(x + 6, y + 8, 5, 0, Math.PI * 2);
        this.ctx.fill();
    }

    drawBaby(x, y) {
        // Simple blob creature
        const bounce = Math.sin(this.currentFrame * 0.5) * 2;

        // Body
        this.ctx.fillStyle = '#ff6b9d';
        this.ctx.strokeStyle = '#333';
        this.ctx.lineWidth = 2;

        this.ctx.beginPath();
        this.ctx.arc(x, y + bounce, 24, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.stroke();

        // Eyes
        this.ctx.fillStyle = '#333';
        this.ctx.beginPath();
        this.ctx.arc(x - 8, y - 5 + bounce, 3, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.beginPath();
        this.ctx.arc(x + 8, y - 5 + bounce, 3, 0, Math.PI * 2);
        this.ctx.fill();

        // Mouth
        this.ctx.beginPath();
        this.ctx.arc(x, y + 5 + bounce, 6, 0, Math.PI);
        this.ctx.stroke();
    }

    drawTeen(x, y) {
        // Slightly larger with ears
        const bounce = Math.sin(this.currentFrame * 0.5) * 3;

        // Body
        this.ctx.fillStyle = '#c77dff';
        this.ctx.strokeStyle = '#333';
        this.ctx.lineWidth = 2;

        this.ctx.beginPath();
        this.ctx.arc(x, y + bounce, 28, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.stroke();

        // Ears
        this.ctx.beginPath();
        this.ctx.arc(x - 20, y - 15 + bounce, 10, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.stroke();
        this.ctx.beginPath();
        this.ctx.arc(x + 20, y - 15 + bounce, 10, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.stroke();

        // Eyes
        this.ctx.fillStyle = '#333';
        this.ctx.beginPath();
        this.ctx.arc(x - 10, y - 5 + bounce, 4, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.beginPath();
        this.ctx.arc(x + 10, y - 5 + bounce, 4, 0, Math.PI * 2);
        this.ctx.fill();
    }

    drawAdult(x, y) {
        // Full grown with details
        const bounce = Math.sin(this.currentFrame * 0.5) * 4;

        // Body
        this.ctx.fillStyle = '#4cc9f0';
        this.ctx.strokeStyle = '#333';
        this.ctx.lineWidth = 2;

        this.ctx.beginPath();
        this.ctx.arc(x, y + bounce, 32, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.stroke();

        // Ears
        this.ctx.beginPath();
        this.ctx.arc(x - 25, y - 20 + bounce, 12, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.stroke();
        this.ctx.beginPath();
        this.ctx.arc(x + 25, y - 20 + bounce, 12, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.stroke();

        // Eyes
        this.ctx.fillStyle = '#fff';
        this.ctx.beginPath();
        this.ctx.arc(x - 12, y - 8 + bounce, 6, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.beginPath();
        this.ctx.arc(x + 12, y - 8 + bounce, 6, 0, Math.PI * 2);
        this.ctx.fill();

        this.ctx.fillStyle = '#333';
        this.ctx.beginPath();
        this.ctx.arc(x - 12, y - 8 + bounce, 3, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.beginPath();
        this.ctx.arc(x + 12, y - 8 + bounce, 3, 0, Math.PI * 2);
        this.ctx.fill();

        // Mouth
        this.ctx.beginPath();
        this.ctx.arc(x, y + 8 + bounce, 10, 0, Math.PI);
        this.ctx.stroke();
    }
}
