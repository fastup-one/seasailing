use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "SeaSailing - Coming Soon" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no" }
        document::Meta { name: "description", content: "SeaSailing - Lightning-fast deployments for your web applications" }
        document::Meta { name: "theme-color", content: "#7e22ce" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        style { {r#"
            /* Base styles */
            * {
                box-sizing: border-box;
                margin: 0;
                padding: 0;
            }
            
            html, body {
                margin: 0;
                padding: 0;
                font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
                color: white;
                background: linear-gradient(135deg, #4338ca, #7e22ce, #ec4899);
                background-size: 200% 200%;
                animation: gradientMove 15s ease infinite;
                overflow-x: hidden;
                width: 100%;
                height: 100%;
                -webkit-text-size-adjust: 100%;
            }
            
            body {
                display: flex;
                flex-direction: column;
                min-height: 100vh;
                min-height: -webkit-fill-available;
                position: relative;
            }
            
            /* Mobile viewport height fix */
            @supports (-webkit-touch-callout: none) {
                body, .container {
                    min-height: -webkit-fill-available;
                }
            }
            
            /* Gradient animation */
            @keyframes gradientMove {
                0% { background-position: 0% 50%; }
                50% { background-position: 100% 50%; }
                100% { background-position: 0% 50%; }
            }
            
            /* Animated blobs - scaled for different devices */
            .blob {
                position: fixed;
                border-radius: 50%;
                filter: blur(60px);
                opacity: 0.4;
                mix-blend-mode: screen;
                animation: blobFloat 10s infinite ease-in-out;
                z-index: -1;
                will-change: transform;
                pointer-events: none;
                transform: translateZ(0);
            }
            
            @keyframes blobFloat {
                0% { transform: translate(0px, 0px) scale(1); }
                33% { transform: translate(30px, -50px) scale(1.1); }
                66% { transform: translate(-20px, 20px) scale(0.9); }
                100% { transform: translate(0px, 0px) scale(1); }
            }
            
            .blob-1 {
                width: 35vw;
                height: 35vw;
                max-width: 500px;
                max-height: 500px;
                background-color: rgba(139, 92, 246, 0.5);
                left: -10vw;
                top: -10vw;
                animation-delay: 0s;
            }
            
            .blob-2 {
                width: 30vw;
                height: 30vw;
                max-width: 400px;
                max-height: 400px;
                background-color: rgba(236, 72, 153, 0.5);
                right: -5vw;
                top: 0;
                animation-delay: 2s;
            }
            
            .blob-3 {
                width: 25vw;
                height: 25vw;
                max-width: 300px;
                max-height: 300px;
                background-color: rgba(59, 130, 246, 0.5);
                left: 10%;
                bottom: 10%;
                animation-delay: 4s;
            }
            
            .blob-4 {
                width: 32vw;
                height: 32vw;
                max-width: 450px;
                max-height: 450px;
                background-color: rgba(245, 158, 11, 0.3);
                right: 0;
                bottom: -10vw;
                animation-delay: 3s;
            }
            
            /* Main container */
            .container {
                width: 100%;
                max-width: 1200px;
                margin: 0 auto;
                padding: clamp(1rem, 5vw, 2rem);
                min-height: 100vh;
                min-height: -webkit-fill-available;
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                text-align: center;
                position: relative;
                z-index: 10;
            }
            
            /* Logo and title - fluid sizing */
            .logo {
                font-size: clamp(2rem, 5vw + 1rem, 3.5rem);
                font-weight: 800;
                letter-spacing: -0.025em;
                line-height: 1.1;
                margin-bottom: clamp(1rem, 3vw, 1.5rem);
            }
            
            .logo-gradient {
                background: linear-gradient(to right, #4ade80, #06b6d4);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
                background-clip: text;
            }
            
            .badge {
                display: inline-block;
                margin-left: 0.5rem;
                font-size: clamp(0.75rem, 2vw, 1rem);
                background-color: rgba(255, 255, 255, 0.1);
                backdrop-filter: blur(4px);
                -webkit-backdrop-filter: blur(4px);
                padding: 0.25rem 0.5rem;
                border-radius: 0.5rem;
                font-weight: 500;
                vertical-align: middle;
            }
            
            /* Tagline - fluid sizing */
            .tagline {
                font-size: clamp(1rem, 2.5vw, 1.25rem);
                max-width: 42rem;
                margin: 0 auto clamp(1.5rem, 5vw, 2rem);
                font-weight: 400;
                opacity: 0.8;
                line-height: 1.6;
                padding: 0 clamp(0.5rem, 3vw, 1rem);
            }
            
            /* Email form - mobile-optimized */
            .email-form {
                position: relative;
                width: 100%;
                max-width: 28rem;
                margin: 0 auto clamp(1.5rem, 5vw, 2.5rem);
                background-color: rgba(255, 255, 255, 0.1);
                backdrop-filter: blur(8px);
                -webkit-backdrop-filter: blur(8px);
                border-radius: 0.5rem;
                padding: 0.25rem;
                overflow: hidden;
                transition: all 0.3s ease;
            }
            
            .email-form:hover, .email-form:focus-within {
                background: linear-gradient(to right, #06b6d4, #4ade80);
            }
            
            .email-form-inner {
                display: flex;
                flex-direction: row;
                background-color: rgba(0, 0, 0, 0.3);
                border-radius: 0.375rem;
                overflow: hidden;
                transition: all 0.3s ease;
            }
            
            .email-form:hover .email-form-inner, .email-form:focus-within .email-form-inner {
                background-color: rgba(0, 0, 0, 0.2);
            }
            
            .email-input {
                flex: 1;
                background: transparent;
                border: none;
                color: white;
                font-size: 1rem;
                padding: 0.75rem 1rem;
                outline: none;
                min-width: 0; /* Prevents flex item from overflowing */
                -webkit-appearance: none; /* Removes iOS styling */
                border-radius: 0; /* Removes iOS styling */
            }
            
            .email-input::placeholder {
                color: rgba(255, 255, 255, 0.5);
            }
            
            .notify-btn {
                background-color: rgba(255, 255, 255, 0.1);
                color: white;
                font-size: 0.875rem;
                font-weight: 500;
                padding: 0.625rem clamp(0.75rem, 3vw, 1.25rem);
                border: none;
                cursor: pointer;
                transition: background-color 0.3s ease;
                white-space: nowrap;
                -webkit-appearance: none; /* Removes iOS styling */
                border-radius: 0; /* Removes iOS styling */
                -webkit-tap-highlight-color: transparent;
            }
            
            .notify-btn:hover, .notify-btn:active {
                background-color: rgba(255, 255, 255, 0.2);
            }
            
            /* Links - touch-friendly */
            .links {
                display: flex;
                flex-wrap: wrap;
                justify-content: center;
                gap: clamp(0.75rem, 3vw, 1.5rem);
                margin-bottom: clamp(2rem, 5vw, 3rem);
                width: 100%;
            }
            
            .link {
                display: flex;
                align-items: center;
                color: rgba(255, 255, 255, 0.7);
                text-decoration: none;
                font-size: clamp(0.75rem, 2vw, 0.875rem);
                font-weight: 500;
                transition: color 0.3s ease;
                padding: 0.5rem;
                -webkit-tap-highlight-color: transparent;
            }
            
            .link:hover, .link:active {
                color: rgba(255, 255, 255, 1);
            }
            
            .link svg {
                margin-right: 0.5rem;
                width: clamp(1rem, 3vw, 1.25rem);
                height: clamp(1rem, 3vw, 1.25rem);
                flex-shrink: 0;
            }
            
            /* Footer */
            .footer {
                margin-top: clamp(2rem, 8vw, 4rem);
                color: rgba(255, 255, 255, 0.5);
                font-size: clamp(0.75rem, 2vw, 0.875rem);
                width: 100%;
                padding: 0 1rem;
            }
            
            .separator {
                margin: 0 0.5rem;
            }
            
            /* Mobile-specific adjustments */
            @media (max-width: 640px) {
                .container {
                    padding-top: 2rem;
                    padding-bottom: 2rem;
                    justify-content: flex-start;
                }
                
                .email-form-inner {
                    flex-direction: column;
                }
                
                .email-input {
                    padding: 0.75rem;
                    text-align: center;
                }
                
                .notify-btn {
                    padding: 0.75rem;
                    width: 100%;
                }
                
                .links {
                    flex-direction: column;
                    align-items: center;
                }
                
                .link {
                    padding: 0.75rem;
                }
                
                .footer {
                    flex-direction: column;
                    gap: 0.5rem;
                }
            }
            
            /* Tablet-specific adjustments */
            @media (min-width: 641px) and (max-width: 1024px) {
                .container {
                    padding: 3rem 2rem;
                }
                
                .links {
                    max-width: 80%;
                }
            }
            
            /* Dark mode preference support */
            @media (prefers-color-scheme: dark) {
                html {
                    color-scheme: dark;
                }
            }
            
            /* Accessibility improvements */
            @media (prefers-reduced-motion: reduce) {
                *, ::before, ::after {
                    animation-duration: 0.01ms !important;
                    animation-iteration-count: 1 !important;
                    transition-duration: 0.01ms !important;
                    scroll-behavior: auto !important;
                }
            }
        "#} }
        AnimatedBackground {}
        Hero {}
    }
}

#[component]
fn AnimatedBackground() -> Element {
    rsx! {
        div { class: "blob blob-1" }
        div { class: "blob blob-2" }
        div { class: "blob blob-3" }
        div { class: "blob blob-4" }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        div { class: "container",
            // Logo/Brand
            div { class: "logo",
                span { class: "logo-gradient", "Sea" }
                span { "Sailing" }
                span { class: "badge", "1.0" }
            }
            // Tagline
            h2 { class: "tagline",
                "Lightning-fast deployments for your web applications.",
                br {}
                "Coming soon."
            }
            // Email signup form
            div { class: "email-form",
                div { class: "email-form-inner",
                    input {
                        class: "email-input",
                        placeholder: "Enter your email for updates",
                        r#type: "email",
                        autocomplete: "email",
                        autocapitalize: "off"
                    }
                    button {
                        class: "notify-btn",
                        "Notify Me"
                    }
                }
            }
            // CTA links
            div { class: "links",
                a {
                    class: "link",
                    href: "#",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none", 
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round", 
                        stroke_linejoin: "round",
                        path { d: "M12 1v6m0 18v-6" }
                        path { d: "M5 12H1m22 0h-4" }
                        path { d: "M6.3 6.3 3.9 3.9m16.2 16.2-2.4-2.4" }
                        path { d: "M17.7 6.3l2.4-2.4M3.9 20.1l2.4-2.4" }
                    }
                    "Learn More"
                }
                a {
                    class: "link",
                    href: "#",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none", 
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round", 
                        stroke_linejoin: "round",
                        path { d: "M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22" }
                    }
                    "GitHub"
                }
                a {
                    class: "link",
                    href: "#",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none", 
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round", 
                        stroke_linejoin: "round",
                        path { d: "M22 4s-.7 2.1-2 3.4c1.6 10-9.4 17.3-18 11.6 2.2.1 4.4-.6 6-2C3 15.5.5 9.6 3 5c2.2 2.6 5.6 4.1 9 4-.9-4.2 4-6.6 7-3.8 1.1 0 3-1.2 3-1.2z" }
                    }
                    "Twitter"
                }
            }
            // Footer
            div { class: "footer",
                "© 2025 SeaSailing. All rights reserved.",
                span { class: "separator", "•" }
                "Built with Dioxus"
            }
        }
    }
}