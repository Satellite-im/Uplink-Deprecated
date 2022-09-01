use crate::themes::Theme;

pub mod button;
pub mod extension_placeholder;
pub mod icon_button;
pub mod icon_input;
pub mod input;
pub mod loader;
pub mod photo_picker;
pub mod pin;
pub mod popup;
pub mod skeletons;
pub mod small_extension_placeholder;
pub mod switch;
pub mod tooltip;
pub mod badge;

pub fn build_style_tag() -> String {
    format!(
        "
            @import url('https://fonts.googleapis.com/css2?family=Poppins&family=Space+Mono&display=swap');
            {theme_colors}
            body, html {{
                background: var(--theme-background);
                color: var(--theme-text);
                font-family: 'Poppins', sans-serif;
                font-size: 12pt;
                height: 100%;
                overflow: hidden;
                margin: 0;
                padding: 0;
            }}
        
            hr {{
                border: none;
                border-bottom: 1px solid var(--theme-borders);
                margin: 1rem 0;
            }}

            .pre-alpha {{
                width: 100%;
                background-color: var(--theme-primary);
                font-size: 10px;
                padding: 0.5 auto;
                text-align: center;

            }}
            .popup .input {{
                background: var(--theme-background);
            }}

            .toast-single {{
                background-color: var(--theme-primary) !important;
                border: none !important;
                color: var(--theme-text-bright) !important; 
                font-family: 'Space Mono', monospace !important;
            }}
            svg {{
                fill: transparent;
                stroke: var(--theme-text);
            }}
            #main {{
                height: 100%;
            }}
            label {{
                color: var(--theme-text-muted);
            }}
            .error_text {{
                color: var(--theme-red);
                opacity: 1;
                transition: 0.2s;
                font-size: 10pt;
            }}
            h1, h2, h3, h4, h5, h6 {{
                font-family: 'Space Mono', monospace;
                margin-bottom: 0;
            }}

            p {{
                margin: 0;
                font-size: 11pt;
                color: var(--theme-text);
                font-family: 'Poppins', sans-serif;

            }}
            h1 {{
                font-size: 18px;
            }}

            .m-top {{
                margin-top: 2rem;
            }}

            .m-bottom {{
                margin-bottom: 2rem;
            }}

            .m-bottom-sm {{
                margin-bottom: 1rem;
            }}

            .m-bottom-xl {{
                margin-bottom: 4rem;
            }}

            label {{
                font-size: 10pt;
                text-align: left;
                margin: 1rem 0;
                font-family: 'Space Mono', monospace;
                color: var(--theme-text-muted);

            }}

            {button}
            {icon_button}
            {icon_input}
            {tooltip}
            {switch}
            {input}
            {loader}
            {photo_picker}

            ::placeholder {{
                color: var(--theme-placeholder);
            }}
        ",
        theme_colors = Theme::load_or_default().rosetta(),

        button = button::css(),
        icon_button = icon_button::css(),
        icon_input = icon_input::css(),
        input = input::css(),
        switch = switch::css(),
        loader = loader::css(),
        tooltip = tooltip::css(),
        photo_picker = photo_picker::css(),
    )
}
