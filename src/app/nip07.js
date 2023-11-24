// nip-07
// async window.nostr.getPublicKey(): string // returns a public key as hex
// async window.nostr.signEvent(event: { created_at: number, kind: number, tags: string[][], content: string }): Event // takes an event object, adds `id`, `pubkey` and `sig` and returns it
// async window.nostr.getRelays(): { [url: string]: {read: boolean, write: boolean} } // returns a basic map of relay urls to relay policies
// async window.nostr.nip04.encrypt(pubkey, plaintext): string // returns ciphertext and iv as specified in nip-04
// async window.nostr.nip04.decrypt(pubkey, ciphertext): string // takes ciphertext and iv as specified in nip-04
export async function getPublicKey() {
  return await window.nostr.getPublicKey();
}

export async function signEvent(event) {
  return await window.nostr.signEvent(event);
}

export async function getRelays() {
  return await window.nostr.getRelays();
}

export async function encryptDM(pubkey, plaintext) {
  return await window.nostr.nip04.encrypt(pubkey, plaintext);
}

export async function decryptDM(pubkey, ciphertext) {
  return await window.nostr.nip04.decrypt(pubkey, ciphertext);
}
