// use crate::app::core_api::api::gen_qr;
use leptos::*;
use qrcode_generator::QrCodeEcc;

const LNURL: &str = "LNURL1DP68GURN8GHJ7EM9W3SKCCNE9E3K7MF0D3H82UNVWQHHVCT6WU7PKHC0";

#[component]
pub fn QrCodeCmp() -> impl IntoView {
    if leptos::leptos_dom::is_browser() {
        let svgs = view! { <div></div> };
        let result: String = qrcode_generator::to_svg_to_string(LNURL, QrCodeEcc::Medium, 256, None::<&str>).unwrap();
        svgs.set_inner_html(&result);
        svgs
    } else {
        view! { <div></div> }
    }

}

#[component]
pub fn Donate() -> impl IntoView {
    view! {
        <div class="flex flex-col mt-8 w-9/12 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 text-center items-center justify-items-center p-5 place-content-center content-center">
            <label class="text-xs sm:text-xs md:text-sm text-black dark:text-white">
                "บริการนี้ฟรี หากท่านต้องการสนับสนุนค่ากาแฟ และโดเมน."
                <br/>
                "สามารถใช้ Bitcoin lightning ⚡ เพื่อสนับสนุนเสรีภาพของสังคมพวกเราต่อไป."
                <br/> "This service is free, but operating it costs a bit of money and time." <br/>
                "If you like it, please consider a small lightning ⚡ donation to keep it alive."
                <br/> "Zap me:⚡vaz@siamstr.com"
            </label>
            <div class="rounded-lg justify-center items-center border-2 text-white text-xl font-bold p-2 m-2 w-fit self-center">
                <QrCodeCmp/>
            </div>
            <label class="block my-1 text-sm font-medium text-gray-900 dark:text-white">
                - หรือ -
            </label>
            <div class="flex flex-col sm:flex-col md:flex-row lg:flex-row xl:flex-row 2xl:flex-row w-full lg:w-3/4 sm:w-full md:w-full 2xl:w-1/2 xl:w-1/2 text-center items-center justify-items-center p-5 place-content-center content-center">
                <div class="w-9/12 flex flex-row">
                    <input
                        type="text"
                        class="text-xs sm:text-xs md:text-sm bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 text-center"
                        prop:value=LNURL
                    />
                    <button
                        class="btn btn-accent w-fit text-xs sm:text-xs md:text-sm text-black dark:text-white"
                        on:click=move |_| {
                            if let Some(clipboard) = window().navigator().clipboard() {
                                let _ = clipboard.write_text(LNURL);
                            } else {
                                window().alert_with_message("Something went wrong :(").unwrap();
                            }
                        }
                    >

                        "📋"
                    </button>
                </div>
                <div>
                    <button
                        class="btn btn-accent w-full text-xs sm:text-xs md:text-sm text-black dark:text-white"
                        data-npub="npub1tr66yvqghfdgwv9yxhmg7xx6pvgvu5uvdc42tgdhsys8vvzdt8msev06fl"
                        data-relays="wss://relay.damus.io,wss://relay.siamstr.com,wss://relay.notoshi.win,wss://relay.nostr.band"
                    >
                      "Zapเลย⚡️"
                    </button>
                    <script src="https://cdn.jsdelivr.net/npm/nostr-zap@0.22.0"></script>
                </div>
            </div>
        </div>
    }
}

