//! Shared styles and metadata in the crown's serving order.

pub struct Sheet {
    pub path: &'static str,
    pub css: &'static str,
}

pub const SHARED: &[Sheet] = &[
    Sheet {
        path: "shell/base-and-chrome.css",
        css: include_str!("../style/shell/base-and-chrome.css"),
    },
    Sheet {
        path: "shell/explain.css",
        css: include_str!("../style/shell/explain.css"),
    },
    Sheet {
        path: "library/_badge.css",
        css: include_str!("../style/library/_badge.css"),
    },
    Sheet {
        path: "library/_breadcrumbs.css",
        css: include_str!("../style/library/_breadcrumbs.css"),
    },
    Sheet {
        path: "library/_button.css",
        css: include_str!("../style/library/_button.css"),
    },
    Sheet {
        path: "library/_calendar.css",
        css: include_str!("../style/library/_calendar.css"),
    },
    Sheet {
        path: "library/_card.css",
        css: include_str!("../style/library/_card.css"),
    },
    Sheet {
        path: "library/_checkbox.css",
        css: include_str!("../style/library/_checkbox.css"),
    },
    Sheet {
        path: "library/_choice.css",
        css: include_str!("../style/library/_choice.css"),
    },
    Sheet {
        path: "library/_collapsible.css",
        css: include_str!("../style/library/_collapsible.css"),
    },
    Sheet {
        path: "library/_editable-field.css",
        css: include_str!("../style/library/_editable-field.css"),
    },
    Sheet {
        path: "library/_empty-state.css",
        css: include_str!("../style/library/_empty-state.css"),
    },
    Sheet {
        path: "library/_exhibit.css",
        css: include_str!("../style/library/_exhibit.css"),
    },
    Sheet {
        path: "library/_file-input.css",
        css: include_str!("../style/library/_file-input.css"),
    },
    Sheet {
        path: "library/_health-board.css",
        css: include_str!("../style/library/_health-board.css"),
    },
    Sheet {
        path: "library/_ip-calendar.css",
        css: include_str!("../style/library/_ip-calendar.css"),
    },
    Sheet {
        path: "library/_icon-button.css",
        css: include_str!("../style/library/_icon-button.css"),
    },
    Sheet {
        path: "library/_input.css",
        css: include_str!("../style/library/_input.css"),
    },
    Sheet {
        path: "library/_loading-spinner.css",
        css: include_str!("../style/library/_loading-spinner.css"),
    },
    Sheet {
        path: "library/_modal.css",
        css: include_str!("../style/library/_modal.css"),
    },
    Sheet {
        path: "library/_nested-button.css",
        css: include_str!("../style/library/_nested-button.css"),
    },
    Sheet {
        path: "library/_plus-button.css",
        css: include_str!("../style/library/_plus-button.css"),
    },
    Sheet {
        path: "library/_progress-bar.css",
        css: include_str!("../style/library/_progress-bar.css"),
    },
    Sheet {
        path: "library/_row-info-tile.css",
        css: include_str!("../style/library/_row-info-tile.css"),
    },
    Sheet {
        path: "library/_secret-tile.css",
        css: include_str!("../style/library/_secret-tile.css"),
    },
    Sheet {
        path: "library/_select.css",
        css: include_str!("../style/library/_select.css"),
    },
    Sheet {
        path: "library/_setting-row.css",
        css: include_str!("../style/library/_setting-row.css"),
    },
    Sheet {
        path: "library/_skeleton.css",
        css: include_str!("../style/library/_skeleton.css"),
    },
    Sheet {
        path: "library/_slot.css",
        css: include_str!("../style/library/_slot.css"),
    },
    Sheet {
        path: "library/_slider.css",
        css: include_str!("../style/library/_slider.css"),
    },
    Sheet {
        path: "library/_stat-tile.css",
        css: include_str!("../style/library/_stat-tile.css"),
    },
    Sheet {
        path: "library/_status-tile.css",
        css: include_str!("../style/library/_status-tile.css"),
    },
    Sheet {
        path: "library/_sub-tabs.css",
        css: include_str!("../style/library/_sub-tabs.css"),
    },
    Sheet {
        path: "library/_switch.css",
        css: include_str!("../style/library/_switch.css"),
    },
    Sheet {
        path: "library/_table.css",
        css: include_str!("../style/library/_table.css"),
    },
    Sheet {
        path: "library/_tabs.css",
        css: include_str!("../style/library/_tabs.css"),
    },
    Sheet {
        path: "library/_text-box.css",
        css: include_str!("../style/library/_text-box.css"),
    },
    Sheet {
        path: "library/_time-picker.css",
        css: include_str!("../style/library/_time-picker.css"),
    },
    Sheet {
        path: "library/_toast.css",
        css: include_str!("../style/library/_toast.css"),
    },
    Sheet {
        path: "library/_toggle.css",
        css: include_str!("../style/library/_toggle.css"),
    },
    Sheet {
        path: "library/_update-card.css",
        css: include_str!("../style/library/_update-card.css"),
    },
    Sheet {
        path: "library/_usage-bar.css",
        css: include_str!("../style/library/_usage-bar.css"),
    },
    Sheet {
        path: "library/_detail-sheet.css",
        css: include_str!("../style/library/_detail-sheet.css"),
    },
    Sheet {
        path: "library/_visibility-toggle.css",
        css: include_str!("../style/library/_visibility-toggle.css"),
    },
    Sheet {
        path: "packs/stats.css",
        css: include_str!("../style/packs/stats.css"),
    },
    Sheet {
        path: "packs/portals.css",
        css: include_str!("../style/packs/portals.css"),
    },
    Sheet {
        path: "packs/test-services.css",
        css: include_str!("../style/packs/test-services.css"),
    },
    Sheet {
        path: "shell/document-2-css.css",
        css: include_str!("../style/shell/document-2-css.css"),
    },
];

pub fn sheet(path: &str) -> Option<&'static str> {
    SHARED.iter().find(|sheet| sheet.path == path).map(|sheet| sheet.css)
}

pub const THEME_CATALOG: &str = include_str!("../style/theme/catalog.json");
pub const AUTHOR_FACE: &str = include_str!("../style/author-face.json");
pub const THEME_SCRIPT: &str = include_str!("../style/theme.js");
pub const EXPLAIN_SCRIPT: &str = include_str!("../style/explain.js");
pub const CHOICE_SCRIPT: &str = include_str!("../style/choice.js");

/// The catalog-derived theme properties for pages that paint before it loads.
#[cfg(feature = "builders")]
pub fn theme_css() -> &'static str {
    use std::fmt::Write;
    use std::sync::OnceLock;

    static CSS: OnceLock<String> = OnceLock::new();
    CSS.get_or_init(|| {
        let catalog: serde_json::Value =
            serde_json::from_str(THEME_CATALOG).expect("kit theme catalog is valid JSON");
        let themes = catalog["themes"].as_object().expect("kit themes are an object");
        let mut css = String::new();
        for (name, theme) in themes {
            let values = theme.as_object().expect("kit theme is an object");
            writeln!(css, "[data-theme=\"{name}\"] {{").unwrap();
            for (key, value) in values {
                writeln!(css, "  --theme-{key}: {};", value.as_str().expect("kit theme value is a string")).unwrap();
            }
            writeln!(css, "  --theme-accent-soft: color-mix(in srgb, var(--theme-accent) 16%, transparent);").unwrap();
            for key in ["primary", "background"] {
                let value = values.get(key).and_then(|v| v.as_str()).unwrap_or("");
                if let Some(hex) = value.strip_prefix('#').filter(|v| v.len() == 6 && v.is_ascii()) {
                    if let Ok(rgb) = u32::from_str_radix(hex, 16) {
                        writeln!(css, "  --theme-{key}-rgb: {}, {}, {};", (rgb >> 16) & 255, (rgb >> 8) & 255, rgb & 255).unwrap();
                    }
                }
            }
            writeln!(css, "}}\n").unwrap();
        }
        css
    }).as_str()
}
