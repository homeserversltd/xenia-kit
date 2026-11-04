//! Category-B component-showcase bodies, built from xenia-kit primitives.
//!
//! This module returns category contents only: it deliberately does not add
//! the surrounding ShowcaseCategory section or its h3 heading.
use crate::{
    html, Markup, Render,
    present::{Paragraph, ShowcaseArticle, ShowcaseRow, CatalogGrid, CatalogKind, Text, TextKind, SliderValue, List},
    specimen::{LoadingFrame,LoadingView,LoadingButton,SliderLabels,AddressBoundaryReadouts,ScaffoldGrid,ProgressGroup,ProgressGroupKind,BreadcrumbReadout,FileSelectionReadout},
    layout::{ShowcaseGrid, ShowcaseItem, ToastSpawn, ToastSpawnKind},
    showcase,
    upload::{
        DirectoryBrowser, DirectoryCrumb, DirectoryEntry, DirectoryExpansion,
        DirectoryLocation, DirectoryTree, UploadControls, UploadPack, UploadProgress,
        UploadProgressList, UploadProgressPlacement, UploadStatus,
    },
    ux::{
        Button, FileInput, Hx, LoadingSpinner, ProgressBar, ProgressKind, ScaffoldSlot,
        Size, Skeleton, SkeletonChild, SkeletonLine, Slider, SlotLifecycle, TextBox,
        TextBoxKind,
    },
};

fn flow(parts: impl IntoIterator<Item = Markup>) -> Markup {
    html! { @for part in parts { (part) } }
}

fn grid(parts: impl IntoIterator<Item = Markup>) -> Markup {
    ShowcaseGrid { content: flow(parts) }.render()
}

fn item(title: &str, content: Markup) -> Markup {
    ShowcaseItem { title: title.into(), content }.render()
}

fn slider(min: i64, max: i64, value: i64, step: Option<u64>, disabled: bool, dhcp: bool) -> Markup {
    Slider {
        min,
        max,
        value,
        step,
        disabled,
        dhcp_boundary: dhcp,
        hx: Hx::default(),
    }
    .render()
}

fn progress(
    kind: ProgressKind,
    label: &str,
    right: Option<&str>,
    aria_label: &str,
    value: Option<u8>,
    fill_text: Option<&str>,
    styled_labels: bool,
    default_fill: bool,
) -> Markup {
    ProgressBar {
        kind,
        label: label.into(),
        right: right.map(str::to_owned),
        aria_label: aria_label.into(),
        value,
        fill_text: fill_text.map(str::to_owned),
        styled_labels,
        default_fill,
        hx: Hx::default(),
    }
    .render()
}

fn upload_pack() -> Markup {
    let location = DirectoryLocation::new(
        "Specimen directory path",
        "/mnt/nas/media",
        vec![
            DirectoryCrumb::new("nas", "/mnt/nas"),
            DirectoryCrumb::new("media", "/mnt/nas/media").current(),
        ],
    );
    let tree = DirectoryTree::new(
        "Sample directories",
        vec![
            DirectoryEntry::new("/mnt/nas", "nas").expansion(DirectoryExpansion::Expanded),
            DirectoryEntry::new("/mnt/nas/media", "media")
                .selected(true)
                .expansion(DirectoryExpansion::Expanded),
            DirectoryEntry::new("/mnt/nas/backups", "backups")
                .expansion(DirectoryExpansion::Collapsed),
        ],
    );
    UploadPack::new(
        Vec::new(),
        DirectoryBrowser::new(
            location,
            tree,
            "Refresh specimen directory",
            "Admin action unavailable",
        ),
        UploadControls::new(
            "Upload specimen files",
            "Upload Selected Files",
            "No files selected",
        ),
    )
    .render()
}

fn progress_uploads() -> Markup {
    let rows = vec![
        UploadProgress::new(
            "queued-backup.tar",
            UploadStatus::Pending,
            2,
            "2 MB",
            "100 MB",
        ),
        UploadProgress::new(
            "family-video.mp4",
            UploadStatus::Uploading,
            64,
            "640 MB",
            "1 GB",
        )
        .with_speed("24 MB/s"),
        UploadProgress::new(
            "photos.zip",
            UploadStatus::Completed,
            100,
            "80 MB",
            "80 MB",
        ),
        UploadProgress::new(
            "blocked.exe",
            UploadStatus::Error,
            18,
            "18 MB",
            "100 MB",
        )
        .with_error("File type is not accepted"),
    ];
    UploadProgressList::new(rows, UploadProgressPlacement::ProgressPanel).render()
}

fn lifecycle_slot(name: &str, placeholder: &str, fill: &str, state: SlotLifecycle) -> ScaffoldSlot {
    let mut slot = ScaffoldSlot::new("test", name, state).expect("valid showcase slot");
    slot.placeholder = Some(placeholder.into());
    slot.fill = Some(fill.into());
    slot
}

fn skeleton_pending() -> Markup {
    let slot = lifecycle_slot(
        "showcase-skeleton-pending",
        "summary",
        "summary",
        SlotLifecycle::Pending {
            children: vec![SkeletonChild::Stack(vec![
                SkeletonChild::Line(SkeletonLine::Full),
                SkeletonChild::Line(SkeletonLine::Medium),
            ])],
            message: "Waiting for the summary…".into(),
        },
    );
    Skeleton::new("test", "1")
        .expect("valid showcase guest")
        .with_label("Pending summary skeleton")
        .with_children(vec![
            SkeletonChild::Row(vec![
                SkeletonChild::Avatar,
                SkeletonChild::Stack(vec![
                    SkeletonChild::Line(SkeletonLine::Medium),
                    SkeletonChild::Line(SkeletonLine::Short),
                ]),
            ]),
            SkeletonChild::LifecycleSlot(slot),
        ])
        .render()
}

fn skeleton_ready() -> Markup {
    let slot = lifecycle_slot(
        "showcase-skeleton-ready",
        "summary",
        "summary",
        SlotLifecycle::Ready {
            updated_at: Some(1_791_993_183_924),
            content: html! {(Text{kind:TextKind::Strong,content:html!{"Summary ready"}})(Paragraph{text:"Three regions arrived independently.".into()})},
        },
    );
    Skeleton::new("test", "1")
        .expect("valid showcase guest")
        .with_label("Ready summary frame")
        .with_children(vec![SkeletonChild::LifecycleSlot(slot)])
        .render()
}

fn skeleton_fault() -> Markup {
    let slot = lifecycle_slot(
        "showcase-skeleton-fault",
        "summary",
        "summary",
        SlotLifecycle::Fault {
            error: "Read timed out".into(),
            children: vec![SkeletonChild::Stack(vec![SkeletonChild::Line(
                SkeletonLine::Short,
            )])],
            message: "Read timed out.".into(),
            retry_guest: "test".into(),
            retry_key: "showcase-skeleton-fault".into(),
        },
    );
    Skeleton::new("test", "1")
        .expect("valid showcase guest")
        .with_label("Summary frame with local fault")
        .with_children(vec![SkeletonChild::LifecycleSlot(slot)])
        .render()
}

fn slot_pending() -> Markup {
    lifecycle_slot(
        "showcase-slot-pending",
        "portal details",
        "details",
        SlotLifecycle::Pending {
            children: vec![SkeletonChild::Stack(vec![
                SkeletonChild::Line(SkeletonLine::Full),
                SkeletonChild::Line(SkeletonLine::Medium),
            ])],
            message: "Loading portal details…".into(),
        },
    )
    .render()
}

fn slot_ready() -> Markup {
    lifecycle_slot(
        "showcase-slot-ready",
        "portal details",
        "details",
        SlotLifecycle::Ready {
            updated_at: Some(1_791_993_183_924),
            content: html! {(Text{kind:TextKind::Strong,content:html!{"Portal details loaded."}})(Paragraph{text:"Ready content occupies the same slot frame.".into()})},
        },
    )
    .render()
}

fn slot_fault() -> Markup {
    lifecycle_slot(
        "showcase-slot-fault",
        "portal details",
        "details",
        SlotLifecycle::Fault {
            error: "Read failed (HTTP 503)".into(),
            children: Vec::new(),
            message: "Read failed (HTTP 503).".into(),
            retry_guest: "test".into(),
            retry_key: "showcase-slot-fault".into(),
        },
    )
    .render()
}

fn row_info_tile() -> Markup {
    flow([Paragraph{text:"A flexible row information tile component for displaying structured data with icons, badges, metadata, and actions.".into()}.render(), grid([showcase::row_info_tile()])])
}

fn dropdowns() -> Markup {
    flow([
        Paragraph{text:"Dropdown components that open on click, providing a list of selectable options.".into()}.render(),
        grid([showcase::select()]),
        item("Usage Notes", List{ordered:false,entries:vec![html!{"Click anywhere on the dropdown trigger to open the options list"},html!{"Keyboard accessible"}]}.render()),
    ])
}

fn slider_category() -> Markup {
    let readout=|n: &str| SliderValue{label:"Value:".into(),value:n.into()}.render();
    flow([Paragraph{text:"Range slider component for selecting numeric values within a min/max range.".into()}.render(),grid([
        item("Basic Slider", html!{(slider(0,100,50,None,false,false))(readout("50"))}),
        item("Slider with Labels", html!{(SliderLabels{start:"More Leases".into(),end:"More Reservations".into()})(slider(0,100,25,None,false,false))(readout("25"))}),
        item("Slider with Step", html!{(slider(0,100,75,Some(5),false,false))(readout("75"))}),
        item("Disabled Slider", html!{(slider(0,100,75,None,true,false))(readout("75"))}),
        item("DHCP Reservation Example", html!{(slider(8,247,50,None,false,true))(readout("50"))(AddressBoundaryReadouts{reserved_prefix:"192.168.123.2 - 192.168.123.".into(),reserved_end:"51".into(),pool_prefix:"192.168.123.".into(),pool_start:"52".into(),pool_suffix:" - 192.168.123.250".into()})}),
    ])])
}

fn textbox() -> Markup {
    flow([Paragraph{text:"Reliable text display component for logs, code, terminal output, and other text content.".into()}.render(),grid([
        item(
            "Log Variant",
            TextBox {
                kind: TextBoxKind::Log,
                title: Some("System Logs".into()),
                content: "[2025-12-08 18:53:57] [INFO] Starting reinstallation of legacy installer tab: testTab\n[2025-12-08 18:55:28] [INFO] Legacy installer tab 'testTab' reinstalled successfully".into(),
                empty: false,
                copy_label: Some("Copy".into()),
                refresh_label: Some("Refresh".into()),
                delegated: true,
                hx: Hx::default(),
            }
            .render(),
        ),
        item(
            "Code Variant",
            TextBox {
                kind: TextBoxKind::Code,
                title: None,
                content: "function calculateTotal(items) { return items.reduce((sum, item) => sum + (item.price * item.quantity), 0); }".into(),
                empty: false,
                copy_label: None,
                refresh_label: None,
                delegated: false,
                hx: Hx::default(),
            }
            .render(),
        ),
        item(
            "Terminal Variant",
            TextBox {
                kind: TextBoxKind::Terminal,
                title: None,
                content: "$ npm install -g serve\n$ serve -s build".into(),
                empty: false,
                copy_label: None,
                refresh_label: None,
                delegated: false,
                hx: Hx::default(),
            }
            .render(),
        ),
        item(
            "With Placeholder",
            TextBox {
                kind: TextBoxKind::Log,
                title: None,
                content: "No log entries yet. Logs will appear here when available.".into(),
                empty: true,
                copy_label: None,
                refresh_label: None,
                delegated: false,
                hx: Hx::default(),
            }
            .render(),
        ),
    ])])
}

fn upload_components() -> Markup {
    let file=html!{
        (FileInput{label:"Select files".into(),button:"Choose Files".into(),display:"No files selected".into(),input_label:"Select test files".into(),display_label:"Selected test files".into(),multiple:true,delegated:true,..FileInput::default()})
        (FileSelectionReadout{message:"No files selected".into(),submit_label:"Use selected files".into()})
        (FileInput{label:"Disabled".into(),button:"Choose File".into(),display:"Selection unavailable".into(),display_label:"Disabled file selection".into(),disabled:true,..FileInput::default()})
        (FileInput{label:"File error".into(),button:"Choose File".into(),display:"Rejected file".into(),display_label:"Rejected file selection".into(),error:Some("File type is not accepted".into()),..FileInput::default()})
    };
    html!{(Paragraph{text:"Working library controls and an Upload domain-pack specimen share the production vocabulary.".into()})
      (CatalogGrid{kind:CatalogKind::UploadLibrary,content:html!{
        (ShowcaseArticle{title:"Breadcrumbs".into(),content:html!{(showcase::breadcrumbs())(BreadcrumbReadout{path:"/mnt/nas/media/kids".into(),reset_label:"Reset to Root".into()})}})
        (ShowcaseArticle{title:"IconButton".into(),content:ShowcaseRow{modal_controls:false,content:showcase::icon_button()}.render()})
        (ShowcaseArticle{title:"FileInput".into(),content:file})
      }})
      (Text{kind:TextKind::Heading3,content:html!{"Upload domain-pack specimen"}})(upload_pack())
    }
}

fn progress_bar() -> Markup {
    let basic = ProgressGroup{kind:ProgressGroupKind::Basic,title:"Basic determinate".into(),content:
        grid([
            progress(ProgressKind::Medium, "Starting", Some("8%"), "Starting progress", Some(8), None, true, true),
            progress(ProgressKind::Medium, "Halfway", Some("50%"), "Halfway progress", Some(50), Some("50%"), true, true),
            progress(ProgressKind::Medium, "Complete", Some("100%"), "Complete progress", Some(100), Some("100%"), true, true),
            progress(ProgressKind::Medium, "Percentage hidden", None, "Progress without visible percentage", Some(72), None, true, true),
        ])}.render();
    let sizes = ProgressGroup{kind:ProgressGroupKind::Sizes,title:"Sizes".into(),content:
        grid([
            progress(ProgressKind::Small, "Small", Some("32%"), "Small progress", Some(32), None, false, false),
            progress(ProgressKind::Medium, "Medium", Some("58%"), "Medium progress", Some(58), None, false, false),
            progress(ProgressKind::Large, "Large", Some("84%"), "Large progress", Some(84), Some("84%"), false, false),
        ])}.render();
    let capacities = ProgressGroup{kind:ProgressGroupKind::Variants,title:"Capacity variants".into(),content:
        grid([
            progress(ProgressKind::Memory, "Memory", Some("68%"), "Memory used", Some(68), Some("68%"), false, false),
            progress(ProgressKind::Swap, "Swap", Some("24%"), "Swap used", Some(24), None, false, false),
            progress(ProgressKind::Process, "Process", Some("46%"), "Process capacity", Some(46), Some("46%"), false, false),
            progress(ProgressKind::Disk, "Disk", Some("82%"), "Disk used", Some(82), Some("82%"), false, false),
        ])}.render();
    let indeterminate = ProgressGroup{kind:ProgressGroupKind::Indeterminate,title:"Indeterminate activity".into(),content:
        progress(
            ProgressKind::Medium,
            "Preparing appliance update",
            Some("Working"),
            "Preparing appliance update",
            None,
            Some("Working"),
            true,
            false,
        )}.render();
    html!{(Paragraph{text:"General library primitives and their Upload domain-pack reflection. Progress reports job completion; capacity examples use meter semantics.".into()})(basic)(sizes)(capacities)(indeterminate)(Text{kind:TextKind::Heading4,content:html!{"Upload domain-pack states"}})(progress_uploads())}
}

fn loading_spinner() -> Markup {
    let spinner = |size, label: Option<&str>, inline, hidden| LoadingSpinner {
        size,
        label: label.map(str::to_owned),
        inline,
        hidden,
        hx: Hx::default(),
    }
    .render();
    let saving = LoadingButton{label:"Saving".into(),spinner:spinner(Size::Small,None,true,true)};
    let sizes=ShowcaseArticle{title:"Size variants".into(),content:ShowcaseRow{modal_controls:false,content:html!{(spinner(Size::Small,Some("Small loading spinner"),false,false))(spinner(Size::Medium,Some("Medium loading spinner"),false,false))(spinner(Size::Large,Some("Large loading spinner"),false,false))}}.render()};
    let inline=ShowcaseArticle{title:"Inline and button".into(),content:html!{(Text{kind:TextKind::Paragraph,content:html!{(Text{kind:TextKind::Span,content:html!{"Loading a compact result…"}}) " " (spinner(Size::Small,Some("Loading"),true,false))}})(saving)}};
    let frame=ShowcaseArticle{title:"Live loading frame".into(),content:LoadingFrame{toggle_label:"Show loaded state".into(),result:"Loading state active".into(),loading:"Loading network data...".into(),spinner:spinner(Size::Medium,Some("Loading network data"),false,false)}.render()};
    html!{(Text{kind:TextKind::Paragraph,content:html!{"Og " (Text{kind:TextKind::Code,content:html!{"LoadingSpinner"}}) " is one primitive; full-tablet, in-frame, and inline loading are compositions of its size plus local layout."}})(CatalogGrid{kind:CatalogKind::Spinner,content:html!{(sizes)(inline)(frame)}})(Text{kind:TextKind::Heading4,content:html!{"Stats full-tablet composition"}})(LoadingView{spinner:spinner(Size::Large,Some("Connecting to server"),false,false),message:"Connecting to server...".into()})}
}

fn skeleton_slot() -> Markup {
    let intro=Text{kind:TextKind::Paragraph,content:html!{"Skeleton is the request-free first frame. Each slot moves from pending to ready, or reports a fault with retry. For a guest pane, put " (Text{kind:TextKind::Code,content:html!{"hx-get=\"/api/tabs/<guest>/...\""}}) " and " (Text{kind:TextKind::Code,content:html!{"hx-trigger=\"load\""}}) " on the slot; the crown defers the read until admission and confines it to that guest's own part path."}};
    html!{(intro)(ScaffoldGrid{guest:"test".into(),version:"1".into(),content:html!{
        (ShowcaseArticle{title:"Skeleton — pending".into(),content:skeleton_pending()})
        (ShowcaseArticle{title:"Skeleton — ready".into(),content:skeleton_ready()})
        (ShowcaseArticle{title:"Skeleton — fault with retry".into(),content:skeleton_fault()})
    }})(Text{kind:TextKind::Heading4,content:html!{"Slot states"}})(ScaffoldGrid{guest:"test".into(),version:"1".into(),content:html!{(slot_pending())(slot_ready())(slot_fault())}})}
}

fn table() -> Markup {
    item("Service Table", showcase::table())
}

fn collapsible() -> Markup {
    grid([showcase::collapsible()])
}

fn modals() -> Markup {
    let buttons = [
        Button {
            text: "Open small modal".into(),
            size: Some(Size::Small),
            button_type: true,
            modal_open: Some("small".into()),
            ..Button::default()
        },
        Button {
            text: "Open medium modal".into(),
            button_type: true,
            modal_open: Some("medium".into()),
            ..Button::default()
        },
        Button {
            text: "Open fullscreen modal".into(),
            size: Some(Size::Large),
            button_type: true,
            modal_open: Some("fullscreen".into()),
            ..Button::default()
        },
    ]
    .into_iter()
    .map(|button| button.render());
    html!{(ShowcaseRow{content:flow(buttons),modal_controls:true})(showcase::modal())}
}

fn toasts() -> Markup {
    // ToastSpawn emits declarative controls only; the host owns the effects.
    let variants = [
        ToastSpawn {
            label: "Spawn info".into(),
            message: "Information notification".into(),
            kind: ToastSpawnKind::Info,
        },
        ToastSpawn {
            label: "Spawn success".into(),
            message: "Settings saved".into(),
            kind: ToastSpawnKind::Success,
        },
        ToastSpawn {
            label: "Spawn warning".into(),
            message: "Review this appliance setting".into(),
            kind: ToastSpawnKind::Warning,
        },
        ToastSpawn {
            label: "Spawn error".into(),
            message: "The requested action failed".into(),
            kind: ToastSpawnKind::Error,
        },
    ]
    .into_iter()
    .map(|toast| toast.render());
    let intro=Text{kind:TextKind::Paragraph,content:html!{"Og " (Text{kind:TextKind::Code,content:html!{"Toast"}}) " feedback grammar: spawn a result, keep multiple results stacked, hover to pause its lifetime, and click a result to dismiss it."}};
    let first=ShowcaseArticle{title:"Variant controls".into(),content:ShowcaseRow{content:flow(variants),modal_controls:false}.render()};
    let second=ShowcaseArticle{title:"Stack and dismissal".into(),content:html!{(Paragraph{text:"Each result lasts three seconds unless hovered. Click a result to dismiss it through the og exit animation.".into()})(ToastSpawn{label:"Spawn another stack item".into(),message:"Stack item".into(),kind:ToastSpawnKind::Stack})}};
    html!{(intro)(CatalogGrid{kind:CatalogKind::Toast,content:html!{(first)(second)}})}
}

/// Return the content for one category-B showcase row.
pub fn section(id: &str) -> Option<Markup> {
    Some(match id {
        "row-info-tile" => row_info_tile(),
        "dropdowns" => dropdowns(),
        "slider" => slider_category(),
        "textbox" => textbox(),
        "upload-components" => upload_components(),
        "progress-bar" => progress_bar(),
        "loading-spinner" => loading_spinner(),
        "skeleton-slot" => skeleton_slot(),
        "table" => table(),
        "collapsible" => collapsible(),
        "modals" => modals(),
        "toasts" => toasts(),
        _ => return None,
    })
}