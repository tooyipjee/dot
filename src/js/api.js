const { invoke } = window.__TAURI__.core;

export class PetAPI {
    async getPetState() {
        return await invoke('get_pet_state');
    }

    async feedPet() {
        return await invoke('feed_pet');
    }

    async playWithPet() {
        return await invoke('play_with_pet');
    }

    async putToSleep() {
        return await invoke('put_to_sleep');
    }

    async revivePet() {
        return await invoke('revive_pet');
    }
}
