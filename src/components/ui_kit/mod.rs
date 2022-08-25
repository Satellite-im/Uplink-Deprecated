use crate::themes::Theme;

pub mod button;
pub mod icon_button;
pub mod tooltip;
pub mod switch;
pub mod input;
pub mod icon_input;
pub mod pin;
pub mod loader;

pub fn build_style_tag() -> String {
    format!(
        "
            body, html {{
                background: {background};
                color: {text};
                font-family: 'Poppins', sans-serif;
                font-size: 12pt;
                height: 100%;
                overflow: hidden;
            }}
            #main {{
                height: 100%;
            }}
            label {{
                color: {text_muted};
            }}
            .error_text {{
                color: {red};
                opacity: 1;
                transition: 0.2s;
            }}
            h1, h2, h3, h4, h5, h6 {{
                font-family: 'Space Mono', monospace;
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

            .m-bottom-xl {{
                margin-bottom: 4rem;
            }}

            {button}
            {icon_button}
            {tooltip}
            {switch}
            {input}
            {icon_input}
            {pin}
            {loader}

            ::placeholder {{
                color: {placeholder};
            }}
        ",
        button = button::styles(),
        switch = switch::styles(),
        pin = pin::styles(),
        input = input::styles(),
        loader = loader::styles(),
        icon_input = icon_input::styles(),
        tooltip = tooltip::styles(),
        icon_button = icon_button::styles(),
        background = Theme::load_or_default().background,
        placeholder = Theme::load_or_default().placeholder,
        text = Theme::load_or_default().text,
        text_muted = Theme::load_or_default().text_muted,
        red = Theme::load_or_default().red,
    )
}