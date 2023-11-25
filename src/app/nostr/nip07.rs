use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/app/nostr/nip07.js")]
extern "C" {
    #[wasm_bindgen(js_name = getPublicKey)]
    pub async fn get_public_key() -> JsValue;

    #[wasm_bindgen(js_name = signEvent)]
    pub async fn sign_event(event: JsValue) -> JsValue;

    #[wasm_bindgen(js_name = getRelays)]
    pub async fn get_relays() -> JsValue;

    #[wasm_bindgen(js_name = encryptDM)]
    pub async fn encrypt_message(pubkey: JsValue, plaintext: JsValue) -> JsValue;

    #[wasm_bindgen(js_name = decryptDM)]
    pub async fn decrypt_message(pubkey: JsValue, ciphertext: JsValue) -> JsValue;
}




