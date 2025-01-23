use leptos::*;

#[component]
pub fn Providers() -> impl IntoView {
    view! {
        <div class="rounded-2xl p-6 w-[480px] shadow-md border border-gray-200">
            <div class="flex justify-between mb-2">
                <h3 class="font-bold text-lg">"Select provider"</h3>
            </div>
            <p class="text-gray-500 text-xs leading-relaxed mb-5">
                Available providers and aggregators are ranked by the estimated received value after network fees from this transaction. Select your choice of providers below.
            </p>
            <div class="rounded-xl p-4 border border-gray-200 hover:border-black cursor-pointer transition-colors">
                <div class="flex justify-between items-center mb-4">
                    <div class="flex items-center gap-2 font-bold text-sm">
                        <img src="images/okx.webp" alt="OKX" class="w-5 h-5" />
                        <div>"OKX DEX Aggregator"</div>
                    </div>
                    <span class="px-2 py-1 rounded text-xs font-medium bg-green-500">"Best"</span>
                </div>
                <span class="block text-lg font-medium text-gray-900 mb-1">"0.01952016 SOL"</span>
                <span class="text-gray-500 text-sm">"≈ $5.01 (excluding priority fee)"</span>
                <div class="mt-6 pt-4 border-t border-gray-200">
                    <div class="flex justify-between text-gray-500 text-sm">
                        <span>"Est network fee"</span>
                        <b>"$0.0012"</b>
                    </div>
                </div>
            </div>
        </div>
    }
}
