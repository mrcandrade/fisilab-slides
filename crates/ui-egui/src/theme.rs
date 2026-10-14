//! Design tokens (the Suíte FisiLab themes "UFRGS Noite" and "UFRGS Dia"; and the original light,
//! PowerPoint-like, and dark ones) and fonts for the UI.

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, Stroke, Visuals};
use fisilab_brand::palette::{self as brand, Tons};
use serde::{Deserialize, Serialize};

/// The interface theme. Persisted in `ui.json` under its camelCase id (`ufrgsNoite`, `ufrgsDia`,
/// `light`, `dark`); the Suíte FisiLab default is "UFRGS Noite".
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Brightness {
    /// "UFRGS Noite": the suite's dark theme (default).
    #[default]
    UfrgsNoite,
    /// "UFRGS Dia": the suite's light theme.
    UfrgsDia,
    /// The original light theme.
    Light,
    /// The original dark theme.
    Dark,
}

impl Brightness {
    /// Every theme, in the order the pickers list them (the suite's first).
    pub const ALL: [Brightness; 4] = [Brightness::UfrgsNoite, Brightness::UfrgsDia, Brightness::Light, Brightness::Dark];

    /// The id used in `ui.json` and by `view.theme`.
    pub fn id(self) -> &'static str {
        match self {
            Brightness::UfrgsNoite => "ufrgsNoite",
            Brightness::UfrgsDia => "ufrgsDia",
            Brightness::Light => "light",
            Brightness::Dark => "dark",
        }
    }

    /// The name shown in the pickers.
    pub fn label(self) -> &'static str {
        match self {
            Brightness::UfrgsNoite => "UFRGS Noite",
            Brightness::UfrgsDia => "UFRGS Dia",
            Brightness::Light => "Claro (original)",
            Brightness::Dark => "Escuro (original)",
        }
    }

    /// A theme by id or name (case-insensitive).
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        Self::ALL.into_iter().find(|b| b.id().eq_ignore_ascii_case(s) || b.label().eq_ignore_ascii_case(s))
    }

    pub fn is_dark(self) -> bool {
        matches!(self, Brightness::UfrgsNoite | Brightness::Dark)
    }

    /// The same family's dark (`true`) or light variant: the suite's themes switch between
    /// themselves, the original ones between themselves.
    pub fn with_dark(self, dark: bool) -> Self {
        match (self, dark) {
            (Brightness::UfrgsNoite | Brightness::UfrgsDia, true) => Brightness::UfrgsNoite,
            (Brightness::UfrgsNoite | Brightness::UfrgsDia, false) => Brightness::UfrgsDia,
            (Brightness::Light | Brightness::Dark, true) => Brightness::Dark,
            (Brightness::Light | Brightness::Dark, false) => Brightness::Light,
        }
    }
}

/// Every colour the chrome uses.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tokens {
    pub dark: bool,
    /// Window background (title bar, tab row, side panes).
    pub chrome: Color32,
    /// The ribbon card and popovers.
    pub card: Color32,
    pub card_border: Color32,
    /// Canvas around the slide.
    pub canvas: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_faint: Color32,
    pub border: Color32,
    pub separator: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    pub selected: Color32,
    /// The app's accent (tabs underline, selected thumbnail, primary buttons).
    pub accent: Color32,
    pub accent_text: Color32,
    pub contextual: Color32,
    pub selection: Color32,
    pub handle_fill: Color32,
    pub handle_stroke: Color32,
    pub guide: Color32,
    pub caret: Color32,
    pub text_selection: Color32,
    pub field_bg: Color32,
    pub shadow: Color32,
    /// Hyperlinks (the suite's themes use their focus blue; the original ones the accent).
    pub link: Color32,
    pub warn: Color32,
    pub error: Color32,
}

/// `a` moved `k` (0–1) of the way to `b`, opaque.
fn mix(a: Color32, b: Color32, k: f32) -> Color32 {
    let l = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * k).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgb(l(a.r(), b.r()), l(a.g(), b.g()), l(a.b(), b.b()))
}

fn with_alpha(c: Color32, a: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), a)
}

impl Tokens {
    /// "UFRGS Noite", the Suíte FisiLab default.
    pub fn ufrgs_noite() -> Self {
        Self::from_tons(&brand::NOITE)
    }

    /// "UFRGS Dia".
    pub fn ufrgs_dia() -> Self {
        Self::from_tons(&brand::DIA)
    }

    /// The chrome tokens for one of the suite's palettes. Colours drawn on the slide itself
    /// (handles, caret) stay neutral so they read on any slide.
    pub fn from_tons(n: &Tons) -> Self {
        Tokens {
            dark: n.escuro,
            chrome: n.painel,
            card: n.elevado,
            card_border: n.borda,
            canvas: n.fundo,
            text: n.texto,
            text_dim: n.texto_fraco,
            text_faint: mix(n.texto_fraco, n.painel, 0.3),
            border: n.borda,
            separator: mix(n.borda, n.painel, 0.25),
            hover: n.hover,
            pressed: n.ativo,
            selected: mix(n.hover, n.ativo, 0.5),
            accent: n.destaque,
            accent_text: n.texto_destaque,
            contextual: n.destaque_hover,
            selection: n.texto_fraco,
            handle_fill: Color32::WHITE,
            handle_stroke: brand::DIA.texto_fraco,
            guide: if n.escuro { n.destaque_hover } else { n.destaque },
            caret: Color32::BLACK,
            text_selection: with_alpha(n.foco, 90),
            field_bg: if n.escuro { n.painel } else { n.elevado },
            shadow: if n.escuro { with_alpha(mix(brand::MARINHO, Color32::BLACK, 0.6), 110) } else { with_alpha(brand::MARINHO, 36) },
            link: n.foco,
            warn: n.aviso,
            error: n.erro,
        }
    }

    pub fn light() -> Self {
        Tokens {
            dark: false,
            chrome: Color32::from_rgb(0xF5, 0xF5, 0xF5),
            card: Color32::WHITE,
            card_border: Color32::from_rgb(0xE3, 0xE3, 0xE3),
            canvas: Color32::from_rgb(0xF0, 0xF0, 0xF0),
            text: Color32::from_rgb(0x24, 0x24, 0x24),
            text_dim: Color32::from_rgb(0x5C, 0x5C, 0x5C),
            text_faint: Color32::from_rgb(0x99, 0x99, 0x99),
            border: Color32::from_rgb(0xD6, 0xD6, 0xD6),
            separator: Color32::from_rgb(0xE1, 0xE1, 0xE1),
            hover: Color32::from_rgb(0xEB, 0xEB, 0xEB),
            pressed: Color32::from_rgb(0xDE, 0xDE, 0xDE),
            selected: Color32::from_rgb(0xE6, 0xE6, 0xE6),
            accent: Color32::from_rgb(0xD9, 0x5A, 0x1A),
            accent_text: Color32::WHITE,
            contextual: Color32::from_rgb(0xC2, 0x4E, 0x14),
            selection: Color32::from_rgb(0x7A, 0x7A, 0x7A),
            handle_fill: Color32::WHITE,
            handle_stroke: Color32::from_rgb(0x6E, 0x6E, 0x6E),
            guide: Color32::from_rgb(0xE8, 0x5D, 0x2A),
            caret: Color32::BLACK,
            text_selection: Color32::from_rgba_unmultiplied(0x5A, 0x8F, 0xE0, 90),
            field_bg: Color32::WHITE,
            shadow: Color32::from_black_alpha(28),
            link: Color32::from_rgb(0xD9, 0x5A, 0x1A),
            warn: Color32::from_rgb(255, 100, 0),
            error: Color32::from_rgb(255, 0, 0),
        }
    }
    pub fn dark() -> Self {
        Tokens {
            dark: true,
            chrome: Color32::from_rgb(0x26, 0x26, 0x26),
            card: Color32::from_rgb(0x33, 0x33, 0x33),
            card_border: Color32::from_rgb(0x44, 0x44, 0x44),
            canvas: Color32::from_rgb(0x1E, 0x1E, 0x1E),
            text: Color32::from_rgb(0xEE, 0xEE, 0xEE),
            text_dim: Color32::from_rgb(0xBB, 0xBB, 0xBB),
            text_faint: Color32::from_rgb(0x88, 0x88, 0x88),
            border: Color32::from_rgb(0x4A, 0x4A, 0x4A),
            separator: Color32::from_rgb(0x48, 0x48, 0x48),
            hover: Color32::from_rgb(0x40, 0x40, 0x40),
            pressed: Color32::from_rgb(0x4C, 0x4C, 0x4C),
            selected: Color32::from_rgb(0x47, 0x47, 0x47),
            accent: Color32::from_rgb(0xF2, 0x6B, 0x1D),
            accent_text: Color32::WHITE,
            contextual: Color32::from_rgb(0xF5, 0x8A, 0x4B),
            selection: Color32::from_rgb(0xAA, 0xAA, 0xAA),
            handle_fill: Color32::WHITE,
            handle_stroke: Color32::from_rgb(0x6E, 0x6E, 0x6E),
            guide: Color32::from_rgb(0xF2, 0x8A, 0x4B),
            caret: Color32::BLACK,
            text_selection: Color32::from_rgba_unmultiplied(0x5A, 0x8F, 0xE0, 90),
            field_bg: Color32::from_rgb(0x2A, 0x2A, 0x2A),
            shadow: Color32::from_black_alpha(90),
            link: Color32::from_rgb(0xF2, 0x6B, 0x1D),
            warn: Color32::from_rgb(255, 143, 0),
            error: Color32::from_rgb(255, 0, 0),
        }
    }
    pub fn for_brightness(b: Brightness) -> Self {
        match b {
            Brightness::UfrgsNoite => Self::ufrgs_noite(),
            Brightness::UfrgsDia => Self::ufrgs_dia(),
            Brightness::Light => Self::light(),
            Brightness::Dark => Self::dark(),
        }
    }
    /// The tokens [`apply`] last installed on `ctx` (before that, the default theme's variant
    /// for egui's dark mode).
    pub fn get(ctx: &egui::Context) -> Self {
        if let Some(t) = ctx.data(|d| d.get_temp::<Tokens>(tokens_id())) {
            return t;
        }
        Self::for_brightness(Brightness::default().with_dark(ctx.global_style().visuals.dark_mode))
    }
}

fn tokens_id() -> egui::Id {
    egui::Id::new("deckcraft-theme-tokens")
}

/// Apply tokens to egui's style.
pub fn apply(ctx: &egui::Context, t: &Tokens) {
    let mut v = if t.dark { Visuals::dark() } else { Visuals::light() };
    v.panel_fill = t.chrome;
    v.window_fill = t.card;
    v.extreme_bg_color = t.field_bg;
    v.faint_bg_color = t.hover;
    v.window_stroke = Stroke::new(1.0, t.card_border);
    v.window_corner_radius = CornerRadius::same(10);
    v.menu_corner_radius = CornerRadius::same(8);
    v.selection.bg_fill = t.accent.gamma_multiply(0.85);
    v.selection.stroke = Stroke::new(1.0, t.accent_text);
    v.hyperlink_color = t.link;
    v.warn_fg_color = t.warn;
    v.error_fg_color = t.error;
    v.override_text_color = Some(t.text);
    for w in [&mut v.widgets.noninteractive, &mut v.widgets.inactive, &mut v.widgets.hovered, &mut v.widgets.active, &mut v.widgets.open] {
        w.corner_radius = CornerRadius::same(5);
    }
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, t.separator);
    v.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    v.widgets.inactive.bg_fill = t.field_bg;
    v.widgets.inactive.bg_stroke = Stroke::NONE;
    v.widgets.hovered.weak_bg_fill = t.hover;
    v.widgets.hovered.bg_fill = t.hover;
    v.widgets.hovered.bg_stroke = Stroke::NONE;
    v.widgets.active.weak_bg_fill = t.pressed;
    v.widgets.active.bg_fill = t.pressed;
    v.widgets.open.weak_bg_fill = t.pressed;
    v.window_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 18, spread: 0, color: t.shadow };
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 3], blur: 12, spread: 0, color: t.shadow };
    ctx.set_visuals(v);
    ctx.data_mut(|d| d.insert_temp(tokens_id(), *t));
    ctx.global_style_mut(|s| {
        s.spacing.item_spacing = egui::vec2(6.0, 4.0);
        s.spacing.button_padding = egui::vec2(6.0, 3.0);
        s.spacing.interact_size.y = 22.0;
        s.text_styles.insert(egui::TextStyle::Body, egui::FontId::proportional(13.0));
        s.text_styles.insert(egui::TextStyle::Button, egui::FontId::proportional(13.0));
        s.text_styles.insert(egui::TextStyle::Small, egui::FontId::proportional(11.0));
        s.text_styles.insert(egui::TextStyle::Heading, egui::FontId::proportional(18.0));
    });
}

/// UI fonts: Inter (from craft-fonts) when embedded, a Japanese UI face for CJK, then egui's.
pub fn install_fonts(ctx: &egui::Context) {
    install_fonts_with(ctx, &[]);
}

/// [`install_fonts`], with `extra` faces from the font database after every interface font: the
/// installed or embedded faces that draw text the interface faces lack (Hangul, Simplified
/// Chinese, Devanagari…, #101). See [`crate::uifonts`].
pub fn install_fonts_with(ctx: &egui::Context, extra: &[std::sync::Arc<deckcraft_fonts::FontFace>]) {
    ctx.set_fonts(font_definitions(extra));
}

/// The interface font definitions; see [`install_fonts_with`].
pub fn font_definitions(extra: &[std::sync::Arc<deckcraft_fonts::FontFace>]) -> FontDefinitions {
    let mut defs = FontDefinitions::default();
    let mut names = vec![];
    for (i, f) in deckcraft_fonts::CRAFT_FONTS.iter().enumerate() {
        let wanted = (f.family == "Inter" && (f.style == "Regular" || f.style == "SemiBold" || f.style == "Bold"))
            || (f.family == "BIZ UDPGothic" && f.style == "Regular")
            || (f.family == "Noto Sans Arabic");
        if !wanted {
            continue;
        }
        let name = format!("craft-{i}-{}-{}", f.family, f.style);
        defs.font_data.insert(name.clone(), std::sync::Arc::new(FontData::from_static(f.bytes)));
        names.push((f.family, f.style, name));
    }
    let regular: Vec<String> = names.iter().filter(|(fam, st, _)| !(*fam == "Inter" && *st != "Regular")).map(|(_, _, n)| n.clone()).collect();
    if let Some(list) = defs.families.get_mut(&FontFamily::Proportional) {
        for (k, n) in regular.iter().enumerate() {
            list.insert(k, n.clone());
        }
    }
    // A bold family for headings and selected tabs.
    let mut bold: Vec<String> = names.iter().filter(|(fam, st, _)| *fam == "Inter" && *st == "SemiBold").map(|(_, _, n)| n.clone()).collect();
    bold.extend(defs.families.get(&FontFamily::Proportional).cloned().unwrap_or_default());
    defs.families.insert(FontFamily::Name("bold".into()), bold);
    add_fallback_faces(&mut defs, extra);
    // The Japanese interface face (the first one when Inter isn't embedded) hints its en and em
    // dashes, bars thinner than a pixel, down to nothing, so they drew as gaps in interface text;
    // unhinted, they draw.
    for (name, d) in defs.font_data.iter_mut() {
        if name.contains("BIZ UDPGothic") {
            let mut data = (**d).clone();
            data.tweak.hinting = Some(false);
            *d = std::sync::Arc::new(data);
        }
    }
    defs
}

/// Add `faces` after every other font of each interface family (the last fallbacks).
pub fn add_fallback_faces(defs: &mut FontDefinitions, faces: &[std::sync::Arc<deckcraft_fonts::FontFace>]) {
    for f in faces {
        let name = format!("fallback {} {} {}", f.family, f.style, f.id());
        if defs.font_data.contains_key(&name) {
            continue;
        }
        let mut data = FontData::from_owned(f.data().to_vec());
        data.index = f.index();
        defs.font_data.insert(name.clone(), std::sync::Arc::new(data));
        for list in defs.families.values_mut() {
            list.push(name.clone());
        }
    }
}

pub fn bold(size: f32) -> egui::FontId {
    egui::FontId::new(size, FontFamily::Name("bold".into()))
}

pub fn font(size: f32) -> egui::FontId {
    egui::FontId::proportional(size)
}

pub fn to_color32(c: deckcraft_color::Rgba) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
}

#[cfg(test)]
mod tests {
    /// Each character of `text` laid out in the interface fonts at interface sizes, and whether it
    /// has ink (a glyph image).
    fn drawn(text: &str) -> Vec<(char, f32, bool)> {
        let ctx = egui::Context::default();
        ctx.set_fonts(super::font_definitions(&[]));
        let mut o = ctx.run_ui(Default::default(), |_| {});
        o.textures_delta.clear();
        let mut out = vec![];
        ctx.fonts_mut(|f| {
            for size in [11.0, 12.0, 13.0, 16.0] {
                for family in [egui::FontFamily::Proportional, egui::FontFamily::Name("bold".into())] {
                    let g = f.layout_no_wrap(text.to_string(), egui::FontId::new(size, family), egui::Color32::WHITE);
                    for gl in g.rows.iter().flat_map(|r| &r.glyphs) {
                        out.push((gl.chr, size, gl.uv_rect.size.x > 0.0 && gl.uv_rect.size.y > 0.0));
                    }
                }
            }
        });
        let mut o = ctx.run_ui(Default::default(), |_| {});
        o.textures_delta.clear();
        out
    }

    #[test]
    fn thin_punctuation_is_drawn_in_interface_text() {
        // En and em dashes (thin bars) were hinted down to nothing in the Japanese interface face,
        // so notes and the outline showed gaps where they were.
        for (c, size, ink) in drawn("\u{2013}\u{2014}-_\u{2026}\u{2022}\u{00b7}\u{2018}\u{2019}\u{201c}\u{201d}=~") {
            assert!(ink, "{c:?} ({:04X}) draws nothing at {size} px", c as u32);
        }
    }

    use super::{Brightness, Tokens};

    fn lum(c: egui::Color32) -> f32 {
        let f = |x: u8| {
            let s = x as f32 / 255.0;
            if s <= 0.03928 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) }
        };
        0.2126 * f(c.r()) + 0.7152 * f(c.g()) + 0.0722 * f(c.b())
    }

    fn contrast(a: egui::Color32, b: egui::Color32) -> f32 {
        let (x, y) = (lum(a), lum(b));
        (x.max(y) + 0.05) / (x.min(y) + 0.05)
    }

    #[test]
    fn ufrgs_noite_is_the_default_theme() {
        assert_eq!(Brightness::default(), Brightness::UfrgsNoite);
        assert_eq!(crate::UiState::default().brightness, Brightness::UfrgsNoite);
        assert_eq!(Brightness::ALL[..2], [Brightness::UfrgsNoite, Brightness::UfrgsDia], "the suite's themes come first");
        assert!(Tokens::for_brightness(Brightness::default()).dark);
    }

    #[test]
    fn every_theme_has_an_id_a_name_and_round_trips() {
        for b in Brightness::ALL {
            assert_eq!(Brightness::parse(b.id()), Some(b));
            assert_eq!(Brightness::parse(b.label()), Some(b));
            let json = serde_json::to_string(&b).unwrap_or_default();
            assert_eq!(json, format!("\"{}\"", b.id()));
            assert_eq!(serde_json::from_str::<Brightness>(&json).ok(), Some(b));
            assert_eq!(Tokens::for_brightness(b).dark, b.is_dark(), "{}", b.label());
            assert_eq!(b.with_dark(b.is_dark()), b);
        }
        assert_eq!(Brightness::parse("UFRGS Dia"), Some(Brightness::UfrgsDia));
        assert_eq!(Brightness::UfrgsNoite.with_dark(false), Brightness::UfrgsDia);
        assert_eq!(Brightness::Light.with_dark(true), Brightness::Dark);
        // A ui.json written by the original app still loads.
        assert_eq!(serde_json::from_str::<Brightness>("\"light\"").ok(), Some(Brightness::Light));
    }

    #[test]
    fn the_suite_themes_use_the_brand_palette_and_stay_readable() {
        use fisilab_brand::palette::{DIA, NOITE};
        for (b, tons) in [(Brightness::UfrgsNoite, NOITE), (Brightness::UfrgsDia, DIA)] {
            let t = Tokens::for_brightness(b);
            assert_eq!((t.chrome, t.card, t.canvas, t.accent, t.link), (tons.painel, tons.elevado, tons.fundo, tons.destaque, tons.foco));
            for bg in [t.chrome, t.card, t.canvas, t.hover, t.field_bg] {
                assert!(contrast(t.text, bg) >= 7.0, "{}: text on {bg:?}", b.label());
                assert!(contrast(t.text_dim, bg) >= 3.0, "{}: dim text on {bg:?}", b.label());
            }
            assert!(contrast(t.accent_text, t.accent) >= 4.5, "{}: text on the accent", b.label());
        }
    }

    #[test]
    fn apply_remembers_the_theme_for_tokens_get() {
        let ctx = egui::Context::default();
        for b in Brightness::ALL {
            let t = Tokens::for_brightness(b);
            super::apply(&ctx, &t);
            assert_eq!(Tokens::get(&ctx), t, "{}", b.label());
            assert_eq!(ctx.global_style().visuals.dark_mode, b.is_dark());
        }
    }
}
