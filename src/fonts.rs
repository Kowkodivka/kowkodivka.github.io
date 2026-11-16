use dioxus::prelude::*;

const FONTS_DIR: Asset = asset!("/assets/fonts", AssetOptions::folder());

const FONT_FACES: &str = r#"
@font-face {
  font-family: "Source Serif 4";
  font-style: normal;
  font-weight: 200 900;
  font-display: swap;
  src: url("FONTS_DIR/source-serif-4.woff2") format("woff2");
}

@font-face {
  font-family: "Source Serif 4";
  font-style: italic;
  font-weight: 200 900;
  font-display: swap;
  src: url("FONTS_DIR/source-serif-4-italic.woff2") format("woff2");
}

@font-face {
  font-family: "IBM Plex Mono";
  font-style: normal;
  font-weight: 400;
  font-display: swap;
  src: url("FONTS_DIR/ibm-plex-mono-400.woff2") format("woff2");
}

@font-face {
  font-family: "IBM Plex Mono";
  font-style: normal;
  font-weight: 500;
  font-display: swap;
  src: url("FONTS_DIR/ibm-plex-mono-500.woff2") format("woff2");
}

@font-face {
  font-family: "IBM Plex Mono";
  font-style: normal;
  font-weight: 600;
  font-display: swap;
  src: url("FONTS_DIR/ibm-plex-mono-600.woff2") format("woff2");
}

@font-face {
  font-family: "IBM Plex Mono";
  font-style: italic;
  font-weight: 400;
  font-display: swap;
  src: url("FONTS_DIR/ibm-plex-mono-400-italic.woff2") format("woff2");
}
"#;

#[component]
pub fn Fonts() -> Element {
    let css = FONT_FACES.replace("FONTS_DIR", &FONTS_DIR.to_string());

    rsx! {
        document::Style { {css} }
    }
}
