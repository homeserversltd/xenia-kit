//! Typed markup builders for motion specimens and their detailed stages.
//!
//! These types only describe structure. Motion behavior, stillness controls,
//! and reduced-motion policy remain owned by their consuming surface.
use crate::layout::{AnimationSpecimen, MotionKind, MotionStage};
use crate::{html, Markup, Render};

/// A supported atom whose appearance is selected by a fixed motion variant.
#[derive(Clone, Copy, Debug)]
pub enum MotionAtom {
    Fade,
    Slide,
    Scale,
    Color,
    Height,
}

impl MotionAtom {
    fn specimen_kind(self) -> MotionKind {
        match self {
            Self::Fade => MotionKind::Fade,
            Self::Slide => MotionKind::Slide,
            Self::Scale => MotionKind::Scale,
            Self::Color => MotionKind::Color,
            Self::Height => MotionKind::Height,
        }
    }
}

/// A reusable labelled atom specimen, composed with the existing layout types.
#[derive(Clone, Debug)]
pub struct MotionAtomSpecimen {
    pub title: String,
    pub atom: MotionAtom,
    pub jewel_label: String,
    pub play_label: String,
}

impl Render for MotionAtomSpecimen {
    fn render(&self) -> Markup {
        AnimationSpecimen {
            title: self.title.clone(),
            kind: self.atom.specimen_kind(),
            hover: false,
            catalog_id: None,
            description: None,
            stage: MotionStage {
                label: self.jewel_label.clone(),
            }
            .render(),
            play_label: Some(self.play_label.clone()),
        }
        .render()
    }
}

/// A typed choice of the detailed composed stages used by motion specimens.
///
/// Stage classes and semantics are fixed here; callers provide only text.
#[derive(Clone, Debug)]
pub enum MotionDetailStage {
    Button { label: String },
    Toggle,
    Modal { dialog_label: String },
    Toast { message: String },
    Progress,
    Spinner { accessible_label: String },
}

impl Render for MotionDetailStage {
    fn render(&self) -> Markup {
        match self {
            Self::Button { label } => html! {
                div class="motion-stage" {
                    button class="ui-button ui-button--primary motion-button" { (label) }
                }
            },
            Self::Toggle => html! {
                div class="motion-stage" {
                    div class="motion-toggle" role="switch" aria-checked="false" { span {} }
                }
            },
            Self::Modal { dialog_label } => html! {
                div class="motion-modal-stage" {
                    div class="motion-backdrop" { div class="motion-dialog" { (dialog_label) } }
                }
            },
            Self::Toast { message } => html! {
                div class="motion-stage" { div class="ui-card motion-toast" { (message) } }
            },
            Self::Progress => html! {
                div class="motion-stage" { div class="motion-progress" { span {} } }
            },
            Self::Spinner { accessible_label } => html! {
                div class="motion-stage" { div class="motion-spinner" aria-label=(accessible_label) {} }
            },
        }
    }
}

/// A focusable fixed hit-area with a non-interactive card face.
#[derive(Clone, Debug)]
pub struct StableCardStage {
    pub title: String,
    pub detail: String,
}

impl Render for StableCardStage {
    fn render(&self) -> Markup {
        html! {
            div class="motion-card-stage" tabindex="0" {
                div class="motion-card-face" {
                    strong { (&self.title) }
                    span { (&self.detail) }
                }
            }
        }
    }
}

/// One explicit lifecycle state; the enum prevents arbitrary phase attributes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MotionPhase {
    Rest,
    Enter,
    Hold,
    Exit,
}

impl MotionPhase {
    fn as_str(self) -> &'static str {
        match self {
            Self::Rest => "REST",
            Self::Enter => "ENTER",
            Self::Hold => "HOLD",
            Self::Exit => "EXIT",
        }
    }
}

/// A lifecycle specimen with phase readback, jewel, and delegated play control.
#[derive(Clone, Debug)]
pub struct MotionLifecycleStage {
    pub phase: MotionPhase,
    pub jewel_label: String,
    pub play_label: String,
}

impl Render for MotionLifecycleStage {
    fn render(&self) -> Markup {
        let phase = self.phase.as_str();
        html! {
            article class="motion-lifecycle" data-motion-lifecycle data-motion-phase=(phase) {
                div class="motion-phase" {
                    "Phase: " strong data-motion-phase-readback { (phase) }
                }
                div class="motion-stage" {
                    span class="motion-jewel motion-lifecycle-jewel" { (&self.jewel_label) }
                }
                button class="ui-button" data-animation-play { (&self.play_label) }
            }
        }
    }
}

/// A side-by-side slide specimen and its reduced-motion preview.
#[derive(Clone)]
pub struct ReducedMotionGroup {
    pub normal_title: String,
    pub normal_stage: Markup,
    pub normal_play_label: String,
    pub reduced_title: String,
    pub reduced_stage: Markup,
    pub reduced_play_label: String,
}

impl Render for ReducedMotionGroup {
    fn render(&self) -> Markup {
        let normal = AnimationSpecimen {
            title: self.normal_title.clone(),
            kind: MotionKind::Slide,
            hover: false,
            catalog_id: None,
            description: None,
            stage: self.normal_stage.clone(),
            play_label: Some(self.normal_play_label.clone()),
        };
        let reduced = AnimationSpecimen {
            title: self.reduced_title.clone(),
            kind: MotionKind::ReducedSlide,
            hover: false,
            catalog_id: None,
            description: None,
            stage: self.reduced_stage.clone(),
            play_label: Some(self.reduced_play_label.clone()),
        };
        html! { div class="motion-reduce-row" { (normal) (reduced) } }
    }
}
