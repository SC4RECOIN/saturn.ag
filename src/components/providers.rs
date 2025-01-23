use leptos::*;

#[component]
pub fn Providers() -> impl IntoView {
    view! {
        <div class="card">
            <div class="card-header">
                <h3>"Select provider"</h3>
            </div>
            <p class="provider-description">
                Available providers and aggregators are ranked by the estimated received value after network fees from this transaction. Select your choice of providers below.
            </p>
            <div class="provider-card">
                <div class="provider-header">
                    <div class="provider-info">
                        <img src="images/okx.webp" alt="OKX" width="20px" height="20px" />
                        <div>OKX DEX Aggregator</div>
                    </div>
                    <span class="best-tag">Best</span>
                </div>
                <span class="sol-amount">0.01952016 SOL</span>
                <span class="usd-amount">"≈ $5.01 (excluding priority fee)"</span>
                <div class="row-details">
                    <div class="row">
                        <span>Est network fee</span>
                        <b>$0.0012</b>
                    </div>
                </div>
            </div>
        </div>
    }
}
