const STYLE_ID: &str = "codex-buddy-theme";

pub fn default_css() -> String {
    include_str!("../../../themes/default/style.css").to_string()
}

/// Wrap theme css so it can be replaced without accumulating style tags.
pub fn managed_css(css: String) -> String {
    format!(
        r#"
(() => {{
  let style = document.getElementById('{id}');
  if (!style) {{
    style = document.createElement('style');
    style.id = '{id}';
    document.head.appendChild(style);
  }}
  style.innerHTML = {css:?};
}})();
"#,
        id = STYLE_ID,
        css = css
    )
}
