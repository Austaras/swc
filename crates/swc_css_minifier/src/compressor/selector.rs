use swc_atoms::atom;
use swc_common::DUMMY_SP;
use swc_css_ast::*;

use super::Compressor;
use crate::util::dedup;

impl Compressor {
    pub(super) fn compress_selector_list(&mut self, selector_list: &mut SelectorList) {
        dedup(&mut selector_list.children);
    }

    pub(super) fn compress_forgiving_selector_list(
        &mut self,
        forgiving_selector_list: &mut ForgivingSelectorList,
    ) {
        dedup(&mut forgiving_selector_list.children);
    }

    pub(super) fn compress_relative_selector_list(
        &mut self,
        relative_selector_list: &mut RelativeSelectorList,
    ) {
        dedup(&mut relative_selector_list.children);
    }

    pub(super) fn compress_forgiving_relative_selector_list(
        &mut self,
        forgiving_relative_selector_list: &mut ForgivingRelativeSelectorList,
    ) {
        dedup(&mut forgiving_relative_selector_list.children);
    }

    pub(super) fn compress_an_plus_b(&mut self, an_plus_b: &mut AnPlusB) {
        match &an_plus_b {
            // `2n+1`, `2n-1`, `2n-3`, etc => `odd`
            AnPlusB::AnPlusBNotation(AnPlusBNotation {
                a: Some(a),
                b: Some(b),
                span,
                ..
            }) if *a == 2 && (*b == 1 || b % 2 == -1) => {
                *an_plus_b = AnPlusB::Ident(Ident {
                    span: *span,
                    value: atom!("odd"),
                    raw: None,
                });
            }
            // `2n-0`, `2n-2`, `2n-4`, etc => `2n`
            AnPlusB::AnPlusBNotation(AnPlusBNotation {
                a: Some(a),
                b: Some(b),
                span,
                ..
            }) if *a == 2 && *b < 0 && b % 2 == 0 => {
                *an_plus_b = AnPlusB::AnPlusBNotation(AnPlusBNotation {
                    span: *span,
                    a: Some(2),
                    a_raw: None,
                    b: None,
                    b_raw: None,
                });
            }
            // `even` => `2n`
            AnPlusB::Ident(Ident { value, span, .. }) if *value == "even" => {
                *an_plus_b = AnPlusB::AnPlusBNotation(AnPlusBNotation {
                    span: *span,
                    a: Some(2),
                    a_raw: None,
                    b: None,
                    b_raw: None,
                });
            }
            // `0n+5` => `5`, `0n-5` => `-5`, etc
            AnPlusB::AnPlusBNotation(AnPlusBNotation {
                a: Some(a),
                b,
                span,
                ..
            }) if *a == 0 => {
                *an_plus_b = AnPlusB::AnPlusBNotation(AnPlusBNotation {
                    span: *span,
                    a: None,
                    a_raw: None,
                    b: *b,
                    b_raw: None,
                });
            }
            // `-5n+0` => `-5n`, etc
            AnPlusB::AnPlusBNotation(AnPlusBNotation {
                a,
                b: Some(b),
                span,
                ..
            }) if *b == 0 => {
                *an_plus_b = AnPlusB::AnPlusBNotation(AnPlusBNotation {
                    span: *span,
                    a: *a,
                    a_raw: None,
                    b: None,
                    b_raw: None,
                });
            }
            _ => {}
        }
    }

    pub(super) fn compress_subclass_selector(&mut self, subclass_selector: &mut SubclassSelector) {
        match &subclass_selector {
            SubclassSelector::PseudoElement(PseudoElementSelector { name, span, .. }) => {
                if matches!(
                    &*name.value,
                    "before" | "after" | "first-letter" | "first-line"
                ) {
                    *subclass_selector = SubclassSelector::PseudoClass(PseudoClassSelector {
                        span: *span,
                        name: name.clone(),
                        children: None,
                    })
                }
            }
            SubclassSelector::PseudoClass(PseudoClassSelector {
                name,
                children: Some(children),
                span,
                ..
            }) if name.value == "nth-child" && children.len() == 1 => match children.first() {
                Some(PseudoClassSelectorChildren::AnPlusB(AnPlusB::AnPlusBNotation(
                    AnPlusBNotation {
                        a: None,
                        b: Some(b),
                        ..
                    },
                ))) if *b == 1 => {
                    *subclass_selector = SubclassSelector::PseudoClass(PseudoClassSelector {
                        span: *span,
                        name: Ident {
                            span: DUMMY_SP,
                            value: atom!("first-child"),
                            raw: None,
                        },
                        children: None,
                    })
                }
                _ => {}
            },
            SubclassSelector::PseudoClass(PseudoClassSelector {
                name,
                children: Some(children),
                span,
                ..
            }) if name.value == "nth-last-child" && children.len() == 1 => match children.first() {
                Some(PseudoClassSelectorChildren::AnPlusB(AnPlusB::AnPlusBNotation(
                    AnPlusBNotation {
                        a: None,
                        b: Some(b),
                        ..
                    },
                ))) if *b == 1 => {
                    *subclass_selector = SubclassSelector::PseudoClass(PseudoClassSelector {
                        span: *span,
                        name: Ident {
                            span: DUMMY_SP,
                            value: atom!("last-child"),
                            raw: None,
                        },
                        children: None,
                    })
                }
                _ => {}
            },
            SubclassSelector::PseudoClass(PseudoClassSelector {
                name,
                children: Some(children),
                span,
                ..
            }) if name.value == "nth-of-type" && children.len() == 1 => match children.first() {
                Some(PseudoClassSelectorChildren::AnPlusB(AnPlusB::AnPlusBNotation(
                    AnPlusBNotation {
                        a: None,
                        b: Some(b),
                        ..
                    },
                ))) if *b == 1 => {
                    *subclass_selector = SubclassSelector::PseudoClass(PseudoClassSelector {
                        span: *span,
                        name: Ident {
                            span: DUMMY_SP,
                            value: atom!("first-of-type"),
                            raw: None,
                        },
                        children: None,
                    })
                }
                _ => {}
            },
            SubclassSelector::PseudoClass(PseudoClassSelector {
                name,
                children: Some(children),
                span,
                ..
            }) if name.value == "nth-last-of-type" && children.len() == 1 => {
                match children.first() {
                    Some(PseudoClassSelectorChildren::AnPlusB(AnPlusB::AnPlusBNotation(
                        AnPlusBNotation {
                            a: None,
                            b: Some(b),
                            ..
                        },
                    ))) if *b == 1 => {
                        *subclass_selector = SubclassSelector::PseudoClass(PseudoClassSelector {
                            span: *span,
                            name: Ident {
                                span: DUMMY_SP,
                                value: atom!("last-of-type"),
                                raw: None,
                            },
                            children: None,
                        })
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub(super) fn compress_compound_selector(&mut self, compound_selector: &mut CompoundSelector) {
        if self.ctx.in_logic_combinator_selector {
            return;
        }

        if !compound_selector.subclass_selectors.is_empty() {
            if let Some(TypeSelector::Universal(UniversalSelector { prefix: None, .. })) =
                compound_selector.type_selector.as_deref()
            {
                compound_selector.type_selector = None;
            }
        }
    }

    pub(super) fn compress_attribute_selector(
        &mut self,
        attribute_selector: &mut AttributeSelector,
    ) {
        if let Some(AttributeSelectorValue::Str(Str { value, span, .. })) =
            &attribute_selector.value
        {
            // A valid unquoted attribute value in CSS is any string of text that is not the
            // empty string, is not just a hyphen (-), consists of escaped characters and/or
            // characters matching [-_a-zA-Z0-9\u00A0-\u10FFFF] entirely, and doesn’t start
            // with a digit or a hyphen followed by a digit.

            // is any string of text that is not the empty string, is not just a hyphen (-)
            if value.is_empty() || value == "-" {
                return;
            }

            let chars = value.chars();
            let mut starts_with_hyphen = false;

            for (idx, c) in chars.enumerate() {
                match c {
                    '0'..='9' if idx == 0 || (starts_with_hyphen && idx == 1) => {
                        return;
                    }
                    '-' => {
                        if idx == 0 {
                            starts_with_hyphen = true;
                        }
                    }
                    _ if !matches!(c, '-' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' | '\u{00a0}'..='\u{10FFFF}') =>
                    {
                        return;
                    }
                    _ => {}
                }
            }

            attribute_selector.value = Some(AttributeSelectorValue::Ident(Ident {
                span: *span,
                value: value.clone(),
                raw: None,
            }));
        }
    }
}
