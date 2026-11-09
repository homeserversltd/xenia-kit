#[path="test_tab_a.rs"] mod categories_a;
#[path="test_tab_b.rs"] mod categories_b;
#[path="test_tab_c.rs"] mod categories_c;
use crate::{html, Markup, Fragment, Render, layout::*, status::*, motion::*, ux::Badge};

/// Render the complete Test tab from typed kit components.
pub fn test_tab() -> Markup {
    let categories: &[(&str,&str,&str,&[&str])] = &[
        ("buttons","Buttons","Button Variants",&["button"]), ("toggles","Toggles","Toggle Switches",&["toggle"]),
        ("tabs","Tabs","Tab Navigation",&["tabs"]), ("inputs","Inputs","Input Fields",&["input"]),
        ("cards","Cards","Cards",&["card"]), ("badges","Badges","Badges",&["badge"]),
        ("checkboxes","Checkboxes","Checkboxes",&["checkbox"]), ("utilities","Utilities","Utility Components",&["plus-button","editable-field"]),
        ("visibility-system","Visibility System","Visibility System",&["visibility-toggle"]), ("calendar-time","Calendar & Time","Calendar & Time Picker",&["calendar","time-picker"]),
        ("lan-ip-calendar","LAN IP Calendar","LAN IP Calendar",&["ip-calendar"]), ("upstream-resolvers","Upstream Resolvers","Upstream Resolvers",&[]),
        ("row-info-tile","Row Info Tile","Row Info Tile",&["row-info-tile"]), ("dropdowns","Dropdowns","Click-to-Open Dropdowns",&["select"]),
        ("slider","Slider","Slider Component",&["slider"]), ("textbox","Text Box","TextBox Component",&["text-box"]),
        ("upload-components","Upload Components","Upload Components",&["breadcrumbs","icon-button","file-input"]),
        ("progress-bar","Progress Bar","Progress Bar",&["progress-bar"]), ("loading-spinner","Loading / Spinner","Loading / Spinner",&["loading-spinner"]),
        ("skeleton-slot","Skeleton & Slot","Skeleton & Slot",&["skeleton","slot"]), ("table","Table","Table",&["table"]),
        ("collapsible","Collapsible","Collapsible",&["collapsible"]), ("modals","Modals","Modals",&["modal"]), ("toasts","Toasts / Notifications","Toasts / Notifications",&[]),
        ("stat-tiles","Stat Tiles","Stat Tiles",&["stat-tile"]), ("sub-tabs","Sub-tabs","In-place Sub-tabs",&["sub-tabs"]),
        ("switches","Switches","Switch Rows",&["switch"]), ("empty-states","Empty States","Empty States",&["empty-state"]),
        ("setting-rows","Setting Rows","Setting Rows",&["setting-row"]),
        ("update-cards","Update Cards","Update Cards",&["update-card"]), ("usage-bars","Usage Bars","Usage Bars",&["usage-bar"]),
    ];
    let chips=TabStrip{label:"Component showcase categories".into(),depth:TabDepth::Category,categories:true,tabs:categories.iter().enumerate().map(|(i,(id,name,_,_))|TabChoice{id:(*id).into(),label:(*name).into(),active:i==0}).collect()};
    let section_order=["buttons","toggles","tabs","inputs","cards","badges","checkboxes","utilities","toasts","visibility-system","lan-ip-calendar","upstream-resolvers","calendar-time","row-info-tile","dropdowns","slider","textbox","upload-components","progress-bar","loading-spinner","skeleton-slot","table","collapsible","modals","stat-tiles","sub-tabs","switches","empty-states","setting-rows","update-cards","usage-bars"];
    let sections=html!{@for id in section_order { @let (_,_,heading,_)=categories.iter().find(|(key,_,_,_)|*key==id).expect("category"); (ShowcaseCategory{id:id.into(),title:(*heading).into(),active:id=="buttons",content:categories_a::section(id).or_else(|| categories_b::section(id)).or_else(|| categories_c::section(id)).expect("named showcase category")})}};
    let showcase=Panel{id:"showcase".into(),active:true,kind:PanelKind::Basic,catalog_specimen:None,content:html!{(Showcase{scope:"showcase".into(),identity:"og-test".into(),tabs:chips,categories:sections})}};
    let portal_data=[("Home Server","Primary appliance web service","systemd",PortalState::Up),("Matrix Chat","Private household messaging","systemd",PortalState::Down),("Backup Sweep","Scheduled storage snapshot script","script",PortalState::Partial),("Search","Household metasearch portal","link",PortalState::Up),("Documentation","External reference with no health probe","link",PortalState::Unknown),("AI Worker","Administrative inference service","systemd",PortalState::Down),("Update Scanner","One update source unavailable","script",PortalState::Partial)];
    let services=Panel{id:"services".into(),active:false,kind:PanelKind::Services,catalog_specimen:Some("test-services-portal-grid".into()),content:html!{
        (SubpanelHeader{kicker:"Services specimen".into(),title:"Service Portals".into(),description:"Portal-card states, service types, and appliance controls.".into(),trailing:Badge::new("7 services").render()})
        (PortalGrid{content:html!{@for (name,description,service_type,state) in portal_data{
            @let details=if service_type=="systemd" {let rows=[[ ("Start","Start service"),("Stop","Stop service"),("Restart","Restart service") ].as_slice(),[("Enable","Enable service at boot"),("Disable","Disable service at boot"),("Status","Check service status")].as_slice()].into_iter().map(|row|row.iter().map(|(label,title)|ServiceAction{label:(*label).into(),title:(*title).into()}).collect()).collect(); ServiceActions{rows}.render()} else if service_type=="script" {ServiceNotice{title:"System restart required for changes to take effect. Script-managed services are controlled through system scripts rather than direct service commands.".into(),text:"Script-managed Service — system restart required".into()}.render()} else {html!{}};
            (ServicePortal{visible:true,service_type:service_type.into(),card:PortalCard{name:name.into(),description:description.into(),image:format!("/api/portals/images/{name}.png"),state,details}})
        } (AddPortalCard{accessible_label:"Add new portal".into(),title:"Add Portal".into(),description:"Create a new portal for your services".into()})}})
    }};
    let readbacks=[("Capabilities",[("Component catalog","Enabled"),("Health checks","Enabled")]),("Settings",[("Default section","Component Showcase"),("Refresh interval","30 seconds")])];
    let editor=ConfigEditor{name_label:"Display name".into(),name:"Test catalog".into(),selection_label:"Default section".into(),choices:["Component Showcase","Services"].into_iter().map(|label|ConfigChoice{label:label.into()}).collect(),toggle_label:"Automatic health checks".into(),checked:true,reset_label:"Reset".into(),save_label:"Save".into()};
    let table=SettingsTable{headings:["Setting".into(),"Value".into(),"Source".into(),"Status".into()],rows:[("Default tab","showcase","shell","Active"),("Health interval","30s","profile","Active"),("External actions","disabled","catalog","Specimen")].into_iter().map(|(setting,value,source,status)|SettingsRow{setting:setting.into(),value:value.into(),source:source.into(),status:status.into()}).collect()};
    let config=Panel{id:"config".into(),active:false,kind:PanelKind::Configuration,catalog_specimen:Some("test-configuration-patterns".into()),content:html!{
      (SubpanelHeader{kicker:"Configuration specimens".into(),title:"Configuration".into(),description:"Three appliance settings patterns.".into(),trailing:Badge::new("3 patterns").render()})
      (ViewLayout{kind:ViewLayoutKind::Configuration,content:html!{
       (ConfigPattern{pattern:"readback".into(),marker:"Pattern A".into(),title:"Sectioned Readback".into(),kind:ConfigKind::Readback,body:html!{@for (title,items) in readbacks{(ReadbackSection{title:title.into(),entries:items.into_iter().map(|(label,value)|ReadbackEntry{label:label.into(),value:value.into()}).collect()})}}})
       (ConfigPattern{pattern:"form".into(),marker:"Pattern B".into(),title:"Form Editor".into(),kind:ConfigKind::Form,body:editor.render()})
       (ConfigPattern{pattern:"grid".into(),marker:"Pattern C".into(),title:"Settings Grid".into(),kind:ConfigKind::Table,body:table.render()})
      }})
    }};
    let summary=HealthSummary{score:"3 / 4".into(),caption:"systems ready".into(),dependencies:[("Rust shell","Available",HealthState::Healthy),("UX library","Available",HealthState::Healthy),("Portal probes","Partial",HealthState::Partial),("Legacy API","Not connected",HealthState::Unknown)].into_iter().map(|(label,value,state)|HealthDependency{label:label.into(),value:value.into(),state}).collect()};
    let timeline=HealthTimeline{entries:[("10:42:01","PASS","Shell route answered"),("10:42:02","PASS","UX packs loaded"),("10:42:03","WARN","Portal probe unavailable")].into_iter().map(|(time,outcome,description)|TimelineEntry{time:time.into(),outcome:outcome.into(),description:description.into()}).collect()};
    let health=Panel{id:"health".into(),active:false,kind:PanelKind::Health,catalog_specimen:Some("test-health-viewports".into()),content:html!{
      (SubpanelHeader{kicker:"Health specimens".into(),title:"Health Status".into(),description:"Summary, service detail, and diagnostic readouts.".into(),trailing:Badge::new("3 viewports").render()})
      (ViewLayout{kind:ViewLayoutKind::Health,content:html!{
       (HealthViewport{id:"summary".into(),kind:HealthViewKind::Summary,marker:"Viewport A".into(),title:"Dashboard Summary".into(),action:HealthCheck{label:"Run Health Check".into()}.render(),body:summary.render()})
       (HealthViewport{id:"services".into(),kind:HealthViewKind::Services,marker:"Viewport B".into(),title:"Per-Service Health".into(),action:html!{},body:html!{@for (label,value,state) in [("Shell","Healthy",HealthState::Healthy),("Assets","Healthy",HealthState::Healthy),("Portals","Partial",HealthState::Partial),("Worker","Down",HealthState::Down),("Legacy API","Unknown",HealthState::Unknown)]{(HealthReadout{label:label.into(),value:value.into(),state})}}})
       (HealthViewport{id:"diagnostics".into(),kind:HealthViewKind::Diagnostics,marker:"Viewport C".into(),title:"Timeline and Results".into(),action:html!{},body:html!{(&timeline)(HealthResults{counts:[("8","tests"),("7","passed"),("1","warning")].into_iter().map(|(value,label)|ResultCount{value:value.into(),label:label.into()}).collect()})(HealthOutput{text:r#"{"schema":"coronatio.test.health.v1","status":"ready","passed":7}"#.into()})}})
      }})
    }};
    let atoms=[("Fade in / out",MotionAtom::Fade,"Fade"),("Slide up / down",MotionAtom::Slide,"Slide"),("Scale presence",MotionAtom::Scale,"Scale"),("Color crossfade",MotionAtom::Color,"Color"),("Height collapse / expand",MotionAtom::Height,"Height")];
    let composed=[("Toggle slider travel",MotionDetailStage::Toggle,"Play"),("Modal fade + scale",MotionDetailStage::Modal{dialog_label:"Dialog".into()},"Play"),("Toast enter / exit",MotionDetailStage::Toast{message:"Settings saved".into()},"Play"),("Progress stripe",MotionDetailStage::Progress,"Start demo"),("Spinner / indicator",MotionDetailStage::Spinner{accessible_label:"Demo indicator".into()},"Start demo")];
    let animations=Panel{id:"animations".into(),active:false,kind:PanelKind::Animations,catalog_specimen:Some("indranet-animation-lab".into()),content:html!{
      (SubpanelHeader{kicker:"IndraNet Animation Lab".into(),title:"Animations".into(),description:"Motion primitives proven here before they reflect through crown panes.".into(),trailing:StillnessControl{label:"Stillness".into()}.render()})
      (AnimationBand{id:"atoms".into(),title:"Band A — Motion Atoms".into(),description:"Small, explicit transitions. Each specimen moves only when played.".into(),content:ViewLayout{kind:ViewLayoutKind::Animation,content:html!{@for (title,atom,label) in atoms{(MotionAtomSpecimen{title:title.into(),atom,jewel_label:label.into(),play_label:"Play".into()})}}}.render()})
      (AnimationBand{id:"composed".into(),title:"Band B — Composed Motion".into(),description:"Reusable reflections raise cards while their stage keeps hit geometry fixed.".into(),content:ViewLayout{kind:ViewLayoutKind::Animation,content:html!{
        (AnimationSpecimen{title:"Button hover / press".into(),kind:MotionKind::Plain,hover:true,catalog_id:None,description:None,stage:MotionDetailStage::Button{label:"Hover me".into()}.render(),play_label:None})
        (AnimationSpecimen{title:"Stable-stage raise — MOTION-TRANCH-01".into(),kind:MotionKind::Plain,hover:true,catalog_id:Some("MOTION-TRANCH-01".into()),description:Some("REST → ENTER (pointer/focus on stage) → HOLD → EXIT → REST. The pointer-transparent face rises 3px with calm depth/glimmer while the stage hit box and status border identity stay fixed; interruption or Stillness restores REST.".into()),stage:StableCardStage{title:"Fixed stage".into(),detail:"Raise + depth/glimmer".into()}.render(),play_label:Some("Play / Stillness".into())})
        @for (title,stage,play) in composed{(AnimationSpecimen{title:title.into(),kind:MotionKind::Plain,hover:false,catalog_id:None,description:None,stage:stage.render(),play_label:Some(play.into())})}
      }}.render()})
      (AnimationBand{id:"lifecycle".into(),title:"Band C — Yijing Lifecycle".into(),description:"REST → ENTER → HOLD → EXIT → REST".into(),content:MotionLifecycleStage{phase:MotionPhase::Rest,jewel_label:"Jewel".into(),play_label:"Run lifecycle".into()}.render()})
      (AnimationBand{id:"accessibility".into(),title:"Band D — Accessibility".into(),description:"Compare ordinary motion with the reduced-motion result.".into(),content:ReducedMotionGroup{normal_title:"Normal".into(),normal_stage:MotionStage{label:"Move".into()}.render(),normal_play_label:"Play".into(),reduced_title:"Reduced motion".into(),reduced_stage:MotionStage{label:"Settle".into()}.render(),reduced_play_label:"Play".into()}.render()})
    }};
    let tabs=TabStrip{label:"Test sections".into(),depth:TabDepth::Panel,categories:false,tabs:[("showcase","Component Showcase"),("services","Services"),("config","Configuration"),("health","Health Status"),("animations","Animations")].into_iter().enumerate().map(|(i,(id,label))|TabChoice{id:id.into(),label:label.into(),active:i==0}).collect()};
    let tablet=Tablet{scope:"test".into(),tabs,panels:html!{(showcase)(services)(config)(health)(animations)}};
    Fragment::new("test",html!{(tablet)}).render()
}