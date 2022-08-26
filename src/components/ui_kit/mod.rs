use crate::themes::Theme;

pub mod button;
pub mod icon_button;
pub mod tooltip;
pub mod switch;
pub mod input;
pub mod icon_input;
pub mod pin;
pub mod loader;
pub mod photo_picker;

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
            }}
            h1, h2, h3, h4, h5, h6 {{
                font-family: 'Space Mono', monospace;
                margin-bottom: 0;
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