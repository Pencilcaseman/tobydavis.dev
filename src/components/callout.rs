use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum CalloutKind {
    Note,
    Tip,
    Warning,
    Success,
}

impl CalloutKind {
    fn css_class(&self) -> &'static str {
        match self {
            CalloutKind::Note => "callout-note",
            CalloutKind::Tip => "callout-tip",
            CalloutKind::Warning => "callout-warning",
            CalloutKind::Success => "callout-success",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            CalloutKind::Note => "Note",
            CalloutKind::Tip => "Tip",
            CalloutKind::Warning => "Warning",
            CalloutKind::Success => "Success",
        }
    }
}

#[component]
pub fn Callout(kind: CalloutKind, children: Element) -> Element {
    let class = format!("callout {}", kind.css_class());
    let label = kind.label();

    rsx! {
        div {
            class: "{class}",
            strong { "{label}: " }
            {children}
        }
    }
}
