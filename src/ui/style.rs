use egui_taffy::taffy;
use egui_taffy::taffy::prelude::*;

#[inline]
pub fn column() -> taffy::Style {
    taffy::Style {
        display: taffy::Display::Flex,
        flex_direction: taffy::FlexDirection::Column,
        align_items: Some(taffy::AlignItems::Center),
        ..Default::default()
    }
}

#[inline]
pub fn row() -> taffy::Style {
    taffy::Style {
        display: taffy::Display::Flex,
        flex_direction: taffy::FlexDirection::Row,
        ..Default::default()
    }
}
#[inline]
pub fn flex() -> taffy::Style {
    taffy::Style {
        display: taffy::Display::Flex,
        ..Default::default()
    }
}

#[inline]
pub fn width(l: f32) -> taffy::Style {
    taffy::Style {
        size: taffy::Size {
            width: length(l),
            height: auto(),
        },
        ..Default::default()
    }
}

#[inline]
pub fn full_size() -> taffy::Style {
    taffy::Style {
        size: taffy::Size {
            width: percent(1.),
            height: percent(1.),
        },
        ..Default::default()
    }
}

#[inline]
pub fn size_len(w: f32, h: f32) -> taffy::Style {
    taffy::Style {
        size: taffy::Size {
            width: length(w),
            height: length(h),
        },
        ..Default::default()
    }
}

#[inline]
pub fn size_percent(w: f32, h: f32) -> taffy::Style {
    taffy::Style {
        size: taffy::Size {
            width: percent(w),
            height: percent(h),
        },
        ..Default::default()
    }
}

#[inline]
pub fn gap_y(px: f32) -> taffy::Style {
    taffy::Style {
        gap: taffy::Size {
            width: length(0.),
            height: length(px),
        },
        ..Default::default()
    }
}

#[inline]
pub fn even_margin_percent(pc: f32) -> taffy::Style {
    taffy::Style {
        margin: percent(pc),
        ..Default::default()
    }
}

#[inline]
pub fn even_margin_len(pc: f32) -> taffy::Style {
    taffy::Style {
        margin: length(pc),
        ..Default::default()
    }
}

#[inline]
pub fn even_padding_percent(pc: f32) -> taffy::Style {
    taffy::Style {
        padding: percent(pc),
        ..Default::default()
    }
}

#[inline]
pub fn even_padding_len(pc: f32) -> taffy::Style {
    taffy::Style {
        padding: length(pc),
        ..Default::default()
    }
}
#[inline]
pub fn align_end() -> taffy::Style {
    taffy::Style {
        justify_content: Some(taffy::JustifyContent::FlexEnd),
        align_items: Some(taffy::AlignItems::Center),
        ..Default::default()
    }
}
#[inline]
pub fn align_self_center() -> taffy::Style {
    taffy::Style {
        align_self: Some(taffy::AlignSelf::Center),
        // justify_self: Some(taffy::JustifySelf::FlexEnd),
        ..Default::default()
    }
}
#[inline]
pub fn flex_shrink() -> taffy::Style {
    taffy::Style {
        min_size: Size {
            width: length(0.),
            height: length(0.),
        },

        ..Default::default()
    }
}

#[inline]
pub fn align_center() -> taffy::Style {
    taffy::Style {
        justify_content: Some(taffy::JustifyContent::Center),
        align_items: Some(taffy::AlignItems::Center),
        ..Default::default()
    }
}

use egui_taffy::taffy::Style;

/// Compose multiple Taffy styles into one.
///
/// ## Semantics
/// - Styles are applied in order
/// - Later styles override earlier ones
/// - `Style::DEFAULT` is treated as "unset"
/// - This function is tightly coupled to the current
///   `egui_taffy::taffy::Style` definition
///
/// ⚠️ If egui_taffy / taffy updates `Style`,
/// this function MUST be reviewed.
#[inline]
pub fn compose_style<I>(styles: I) -> Style
where
    I: IntoIterator<Item = Style>,
{
    let mut out = Style::DEFAULT;

    for style in styles {
        merge_style(&mut out, style);
    }

    out
}

#[inline]
fn merge_style(dst: &mut Style, src: Style) {
    let def = Style::DEFAULT;

    macro_rules! take {
        ($field:ident) => {
            if src.$field != def.$field {
                dst.$field = src.$field;
            }
        };
    }

    // ---- Box model & layout ----
    take!(display);
    take!(item_is_table);
    take!(box_sizing);
    take!(overflow);
    take!(scrollbar_width);
    take!(position);
    take!(inset);
    take!(size);
    take!(min_size);
    take!(max_size);
    take!(aspect_ratio);
    take!(margin);
    take!(padding);
    take!(border);

    // ---- Alignment ----
    take!(align_items);
    take!(align_self);
    take!(justify_items);
    take!(justify_self);
    take!(align_content);
    take!(justify_content);
    take!(gap);
    take!(text_align);

    // ---- Flexbox ----
    take!(flex_direction);
    take!(flex_wrap);
    take!(flex_basis);
    take!(flex_grow);
    take!(flex_shrink);

    // ---- Grid (Vec-based: overwrite if non-empty) ----
    if !src.grid_template_rows.is_empty() {
        dst.grid_template_rows = src.grid_template_rows;
    }
    if !src.grid_template_columns.is_empty() {
        dst.grid_template_columns = src.grid_template_columns;
    }
    if !src.grid_auto_rows.is_empty() {
        dst.grid_auto_rows = src.grid_auto_rows;
    }
    if !src.grid_auto_columns.is_empty() {
        dst.grid_auto_columns = src.grid_auto_columns;
    }

    take!(grid_auto_flow);
    take!(grid_row);
    take!(grid_column);
}
