//! SelectValue component implementation.

use dioxus::prelude::*;

use super::super::context::SelectContext;

/// The props for the [`SelectValue`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectValueProps {
    /// Additional attributes for the value element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # SelectValue
///
/// The trigger button for the [`Select`](super::select::Select) component which controls if the [`SelectList`](super::list::SelectList) is rendered.
///
/// This must be used inside a [`Select`](super::select::Select) component.
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::select::{
///     Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
///     SelectTrigger, SelectValue,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Select::<String> {
///             placeholder: "Select a fruit...",
///             SelectTrigger {
///                 aria_label: "Select Trigger",
///                 width: "12rem",
///                 SelectValue {}
///             }
///             SelectList {
///                 aria_label: "Select Demo",
///                 SelectGroup {
///                     SelectGroupLabel { "Fruits" }
///                     SelectOption::<String> {
///                         index: 0usize,
///                         value: "apple",
///                         "Apple"
///                         SelectItemIndicator { "✔️" }
///                     }
///                     SelectOption::<String> {
///                         index: 1usize,
///                         value: "banana",
///                         "Banana"
///                         SelectItemIndicator { "✔️" }
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
///
/// ## Styling
///
/// The [`SelectValue`] component defines a span with a `data-placeholder` attribute if a placeholder is set.
#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
    let ctx = use_context::<SelectContext>();

    let selected_text_value = use_memo(move || {
        let value = ctx.value.read();
        value.as_ref().and_then(|v| {
            ctx.options
                .read()
                .iter()
                .find(|opt| opt.value == *v)
                .map(|opt| opt.text_value.clone())
        })
    });

    // Remember the last successfully resolved text so we don't flash the
    // placeholder during transient states where `ctx.options` hasn't been
    // (re-)populated yet (e.g. when the listbox mounts/unmounts during the
    // open/close animation).
    let mut last_resolved = use_signal(String::new);
    use_effect(move || {
        if let Some(text) = selected_text_value() {
            last_resolved.set(text);
        }
    });

    let has_value = ctx.value.read().is_some();
    let display_value = match selected_text_value() {
        Some(text) => text,
        None if has_value => {
            // Value is set but options haven't resolved yet — keep showing
            // the last known text instead of falling back to the placeholder.
            let last = last_resolved.read().clone();
            if last.is_empty() {
                ctx.placeholder.cloned()
            } else {
                last
            }
        }
        None => ctx.placeholder.cloned(),
    };

    rsx! {
        // Add placeholder option if needed
        span {
            "data-placeholder": !has_value,
            ..props.attributes,
            {display_value}
        }
    }
}
