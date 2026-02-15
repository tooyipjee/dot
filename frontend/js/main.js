import { PetRenderer } from './renderer.js';
import { PetAPI } from './api.js';

class DotApp {
    constructor() {
        this.renderer = new PetRenderer('pet-canvas');
        this.api = new PetAPI();
        this.petState = null;
        this.updateInterval = null;

        this.init();
    }

    async init() {
        // Set up button event listeners
        document.getElementById('feed-btn').addEventListener('click', () => this.feedPet());
        document.getElementById('play-btn').addEventListener('click', () => this.playWithPet());
        document.getElementById('sleep-btn').addEventListener('click', () => this.putToSleep());
        document.getElementById('revive-btn').addEventListener('click', () => this.revivePet());

        // Settings menu
        const settingsBtn = document.getElementById('settings-btn');
        const settingsMenu = document.getElementById('settings-menu');
        settingsBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            settingsMenu.classList.toggle('hidden');
        });
        document.getElementById('reset-btn').addEventListener('click', () => {
            this.revivePet();
            settingsMenu.classList.add('hidden');
        });
        document.getElementById('quit-btn').addEventListener('click', () => this.api.quitApp());
        document.addEventListener('click', () => settingsMenu.classList.add('hidden'));

        // Load initial state
        await this.updatePetState();

        // Start render loop
        this.renderer.start((deltaTime) => {
            if (this.petState) {
                this.renderer.render(this.petState);
            }
        });

        // Poll for state updates every 5 seconds
        this.updateInterval = setInterval(() => this.updatePetState(), 5000);
    }

    async updatePetState() {
        try {
            this.petState = await this.api.getPetState();
            this.updateUI();
        } catch (error) {
            console.error('Failed to get pet state:', error);
        }
    }

    updateUI() {
        if (!this.petState) return;

        // Update stat bars
        this.updateStatBar('hunger', this.petState.hunger);
        this.updateStatBar('happiness', this.petState.happiness);
        this.updateStatBar('energy', this.petState.energy);
        this.updateStatBar('health', this.petState.health);

        // Update pet info
        document.getElementById('pet-name').textContent = this.petState.name || 'dot';
        document.getElementById('pet-stage').textContent = this.petState.stage;

        // Calculate age in days
        const ageDays = Math.floor(this.petState.age / 86400);
        document.getElementById('pet-age').textContent = `${ageDays}d`;

        // Handle death state
        const deathOverlay = document.getElementById('death-overlay');
        const actionBtns = document.querySelectorAll('#actions-container .action-btn');

        if (!this.petState.is_alive) {
            deathOverlay.classList.remove('hidden');
            actionBtns.forEach(btn => btn.disabled = true);
        } else {
            deathOverlay.classList.add('hidden');
            actionBtns.forEach(btn => btn.disabled = false);
        }
    }

    updateStatBar(stat, value) {
        const bar = document.getElementById(`${stat}-bar`);
        const valueSpan = document.getElementById(`${stat}-value`);

        bar.style.width = `${value}%`;
        valueSpan.textContent = value;

        // Keep black and white - no color changes
        bar.style.background = '#000';
    }

    async feedPet() {
        try {
            this.petState = await this.api.feedPet();
            this.updateUI();
        } catch (error) {
            console.error('Failed to feed pet:', error);
        }
    }

    async playWithPet() {
        try {
            this.petState = await this.api.playWithPet();
            this.updateUI();
        } catch (error) {
            console.error('Failed to play with pet:', error);
        }
    }

    async putToSleep() {
        try {
            this.petState = await this.api.putToSleep();
            this.updateUI();
        } catch (error) {
            console.error('Failed to put pet to sleep:', error);
        }
    }

    async revivePet() {
        try {
            this.petState = await this.api.revivePet();
            this.updateUI();
        } catch (error) {
            console.error('Failed to revive pet:', error);
        }
    }
}

// Start the app
new DotApp();
