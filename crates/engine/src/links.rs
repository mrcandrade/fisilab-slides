//! Project links (shown in Help, About, the CLI and MCP).
//!
//! FisiSlides is the Suíte FisiLab (UFRGS) build of DeckCraft. These mirror the constants of the
//! `fisilab-brand` crate, which the engine can't depend on (it is egui-only, and nothing below
//! the UI layer depends on egui).

/// Site da Suíte FisiLab (código-fonte, instaladores, créditos).
pub const SUITE: &str = "https://github.com/mrcandrade/larfic/tree/main/suite-fisilab";
/// Manifesto pelas tecnologias livres.
pub const MANIFESTO: &str = "https://github.com/mrcandrade/larfic/blob/main/suite-fisilab/MANIFESTO.md";
/// The original project, DeckCraft.
pub const UPSTREAM: &str = "https://github.com/storytold/deckcraft";
/// FisiSlides' source.
pub const GITHUB: &str = "https://github.com/mrcandrade/fisislides";
/// Where to report a problem.
pub const ISSUES: &str = "https://github.com/mrcandrade/fisislides/issues";
