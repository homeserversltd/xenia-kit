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
        path: "library/_file-input.css",
        css: include_str!("../style/library/_file-input.css"),
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
        path: "shell/document-2-css.css",
        css: include_str!("../style/shell/document-2-css.css"),
    },
];

pub fn sheet(path: &str) -> Option<&'static str> {
    SHARED.iter().find(|sheet| sheet.path == path).map(|sheet| sheet.css)
}

pub const THEME_CATALOG: &str = include_str!("../style/theme/catalog.json");
pub const AUTHOR_FACE: &str = include_str!("../style/author-face.json");
