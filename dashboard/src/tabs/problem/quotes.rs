// what: quote cards displaying influential tweets about WASM/WASI
// why: establishes credibility with quotes from Docker founder and Mozilla engineer
// relations: used by problem/component.rs as a sub-section

use leptos::*;

/// Solomon Hykes tweet info
const SOLOMON_NAME: &str = "Solomon Hykes";
const SOLOMON_ROLE: &str = "Docker Co-Founder";
const SOLOMON_DATE: &str = "March 27, 2019";
const SOLOMON_TWEET: &str = "So will wasm replace Docker? No, but imagine a future where Docker runs linux containers, windows containers and wasm containers side by side. Over time wasm might become the most popular container type. Docker will love them all equally, and run it all :)";
const SOLOMON_URL: &str = "https://x.com/solomonstre/status/1111113329647325185";

/// Lin Clark tweet info
const LIN_NAME: &str = "Lin Clark";
const LIN_ROLE: &str = "Mozilla Principal Engineer";
const LIN_DATE: &str = "March 27, 2019";
const LIN_TWEET: &str = "WebAssembly running outside the web has a huge future. And that future gets one giant leap closer today with... Announcing WASI: A system interface for running WebAssembly outside the web.";
const LIN_URL: &str = "https://x.com/linclark/status/1110920999061594113";
const LIN_ARTICLE: &str = "https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/";

/// renders the quote cards section with both tweets
#[component]
pub fn QuotesSection() -> impl IntoView {
    view! {
        <div class="quotes-section">
            <h3>"💬 The Vision: Containers + WASM"</h3>
            <p class="section-hint">"Docker's founder on the coexistence of containers and WASM (2019)"</p>
            
            <div class="quote-cards">
                <QuoteCard 
                    name=SOLOMON_NAME
                    role=SOLOMON_ROLE
                    date=SOLOMON_DATE
                    quote=SOLOMON_TWEET
                    tweet_url=SOLOMON_URL
                    avatar="🐳"
                />
                
                <QuoteCard 
                    name=LIN_NAME
                    role=LIN_ROLE
                    date=LIN_DATE
                    quote=LIN_TWEET
                    tweet_url=LIN_URL
                    avatar="🦊"
                    article_url=LIN_ARTICLE
                />
            </div>
        </div>
    }
}

/// individual quote card styled like a tweet
#[component]
fn QuoteCard(
    name: &'static str,
    role: &'static str,
    date: &'static str,
    quote: &'static str,
    tweet_url: &'static str,
    avatar: &'static str,
    #[prop(optional)] article_url: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class="quote-card">
            <div class="quote-header">
                <span class="quote-avatar">{avatar}</span>
                <div class="quote-author">
                    <span class="quote-name">{name}</span>
                    <span class="quote-role">{role}</span>
                </div>
            </div>
            
            <p class="quote-text">"\""{ quote }"\""</p>
            
            <div class="quote-footer">
                <span class="quote-date">{date}</span>
                <div class="quote-links">
                    <a href={tweet_url} target="_blank" rel="noopener" class="quote-link">
                        "View on X →"
                    </a>
                    {article_url.map(|url| view! {
                        <a href={url} target="_blank" rel="noopener" class="quote-link article">
                            "Read Article →"
                        </a>
                    })}
                </div>
            </div>
        </div>
    }
}
