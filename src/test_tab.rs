#[path="test_tab_a.rs"] mod categories_a;
#[path="test_tab_b.rs"] mod categories_b;
#[path="test_tab_c.rs"] mod categories_c;
use crate::{html, Markup, Fragment, Render, layout::*, status::*, motion::*, composite::{Guide, GuideLine, Exhibit}, ux::{Badge, Button, ButtonTone, Toggle, Size, IconButton, Icon, Shape, LoadingSpinner, ToastKind, VisibleToast}};

fn eye() -> PortalEye { PortalEye { visible:true, label:"Hide portal".into(), gate:None, seats:vec![] } }

fn actions() -> Markup {
    let rows = [[("Start","Start service"),("Stop","Stop service"),("Restart","Restart service")],[("Enable","Enable service at boot"),("Disable","Disable service at boot"),("Status","Check service status")]].into_iter().map(|row| row.into_iter().map(|(label,title)| ServiceAction { label:label.into(), title:title.into(), action:None }).collect()).collect();
    ServiceActions { rows, gate:None, services_json:None }.render()
}
fn room(id: &str, active: bool, content: Markup) -> Panel { Panel { id:id.into(), active, kind:PanelKind::Basic, catalog_specimen:None, content } }

/// Render the complete Test tab from typed kit components.
pub fn test_tab() -> Markup {
    let categories: &[(&str,&str,&str,&[&str])] = &[

        ("tabs","Tabs","Tab Navigation",&["tabs"]), ("inputs","Inputs","Input Fields",&["input"]),
        ("cards","Cards","Cards",&["card"]), ("badges","Badges","Badges",&["badge"]),
        ("checkboxes","Checkboxes","Checkboxes",&["checkbox"]), ("utilities","Utilities","Utility Components",&["plus-button","editable-field"]),
        ("visibility-system","Visibility System","Visibility System",&["visibility-toggle"]), ("calendar-time","Calendar & Time","Calendar & Time Picker",&["calendar","time-picker"]),
        ("lan-ip-calendar","LAN IP Calendar","LAN IP Calendar",&["ip-calendar"]), ("upstream-resolvers","Upstream Resolvers","Upstream Resolvers",&[]),
        ("row-info-tile","Row Info Tile","Row Info Tile",&["row-info-tile"]), ("dropdowns","Dropdowns","Click-to-Open Dropdowns",&["select"]),
        ("slider","Slider","Slider Component",&["slider"]), ("textbox","Text Box","TextBox Component",&["text-box"]),
        ("upload-components","Upload Components","Upload Components",&["breadcrumbs","file-input"]),
        ("progress-bar","Progress Bar","Progress Bar",&["progress-bar"]),
        ("skeleton-slot","Skeleton & Slot","Skeleton & Slot",&["skeleton","slot"]), ("table","Table","Table",&["table"]),
        ("collapsible","Collapsible","Collapsible",&["collapsible"]), ("modals","Modals","Modals",&["modal"]),
        ("stat-tiles","Stat Tiles","Stat Tiles",&["stat-tile"]),
        ("status-tiles","Status Tiles","Status Tiles",&["status-tile"]),
        ("sub-tabs","Sub-tabs","In-place Sub-tabs",&["sub-tabs"]),
        ("switches","Switches","Switch Rows",&["switch"]), ("empty-states","Empty States","Empty States",&["empty-state"]),
        ("update-cards","Update Cards","Update Cards",&["update-card"]), ("usage-bars","Usage Bars","Usage Bars",&["usage-bar"]),
    ];
    let chips=TabStrip{label:"Component showcase categories".into(),depth:TabDepth::Category,categories:true,tabs:categories.iter().enumerate().map(|(i,(id,name,_,_))|TabChoice{id:(*id).into(),label:(*name).into(),active:i==0}).collect()};
    let section_order=["tabs","inputs","cards","badges","checkboxes","utilities","visibility-system","lan-ip-calendar","upstream-resolvers","calendar-time","row-info-tile","dropdowns","slider","textbox","upload-components","progress-bar","skeleton-slot","table","collapsible","modals","stat-tiles","status-tiles","sub-tabs","switches","empty-states","update-cards","usage-bars"];
    let sections=html!{@for id in section_order { @let (_,_,heading,_)=categories.iter().find(|(key,_,_,_)|*key==id).expect("category"); (ShowcaseCategory{id:id.into(),title:(*heading).into(),active:id=="tabs",content:categories_a::section(id).or_else(|| categories_b::section(id)).or_else(|| categories_c::section(id)).expect("named showcase category")})}};
    let showcase=Panel{id:"showcase".into(),active:false,kind:PanelKind::Basic,catalog_specimen:None,content:html!{(Showcase{scope:"showcase".into(),identity:"og-test".into(),tabs:chips,categories:sections})}};
    let guide=room("how-it-works",true,Guide{lines:vec![
        GuideLine{lead:None,text:"This site is built from a small set of pieces that repeat from tab to tab. A tab is those pieces arranged for one job. This tab shows the pieces on their own.".into()},
        GuideLine{lead:Some("Portals".into()),text:" is a grid of portal tiles. Each tile is a door to a service on your HomeServer, or to a link you keep there. Press a tile and it opens in a new browser tab. The tile is in the Tiles room.".into()},
        GuideLine{lead:Some("Admin Mode".into()),text:" adds controls to the pieces. Press Enter Admin Mode at the top right and give your PIN: pieces that can be changed then show their controls. Press Exit Admin Mode and they are read-only again.".into()},
        GuideLine{lead:Some("Buttons and switches".into()),text:" are how you act on the site. A button is for one action; a switch shows whether one thing is on or off. They are in the Buttons & switches room, with the icon button.".into()},
        GuideLine{lead:Some("Messages".into()),text:" are how the site answers you. A toast tells you how something went; a spinner tells you to wait. They are in the Messages room.".into()},
    ]}.render());
    let portal=PortalCard{name:"Home Server".into(),description:"Primary appliance web service".into(),image:"/api/portals/images/Home Server.png".into(),state:PortalState::Up,visible:true,show_name:false,link:None,chrome:html!{(eye())},controls:actions(),seats:PortalSeats::default()};
    let tile_order = [("setting-rows", "Setting Rows"), ("secret-tiles", "Secret Tiles")];
    let selected_tile = "secret-tiles";
    let tile_showcase = Showcase {
        scope: "tiles-showcase".into(),
        identity: "tile-catalog".into(),
        tabs: TabStrip {
            label: "Tile categories".into(),
            depth: TabDepth::Category,
            categories: true,
            tabs: tile_order.iter().map(|&(id, label)| TabChoice {
                id: id.into(), label: label.into(), active: id == selected_tile,
            }).collect(),
        },
        categories: html! {
            @for (id, title) in tile_order {
                @let specimen = categories_c::section(id).expect("named tile category");
                @let content = if id == "secret-tiles" {
                    html! { div data-catalog-specimen="anatomy" {
                        p { "Examples only. Nothing is saved here." }
                        (specimen)
                    } }
                } else { specimen };
                (ShowcaseCategory { id: id.into(), title: title.into(), active: id == selected_tile, content })
            }
        },
    };
    let tiles=room("tiles",false,html! {
        (Exhibit{name:"Portal tile".into(),piece:portal.render(),admin:true,words:vec![
            "A door to a service on your HomeServer, or to a link. On the Portals tab, press a tile and it opens in a new browser tab. The one shown here is for looking at: it opens nothing and its controls change nothing.".into(),
            "The coloured edge tells you whether the service is running: green when it is up, red when it is down, amber when only part of it is up, and a muted edge when it cannot be checked, as with a plain link.".into(),
            "Guests see the portal's name and a larger picture, and no controls. In Admin Mode a service's tile shows the controls drawn here. The eye at the top left hides the tile from guests or shows it again; a hidden tile stays on your Admin Mode page, dimmed, with a dashed edge. The six keys start, stop and restart the service, turn it on or off at startup, and show its status.".into(),
            "A service run by a script shows a note in place of the keys: it is managed by its script, and a change to it needs a system restart. A plain link has no keys.".into(),
            "You will meet it on the Portals tab.".into(),
        ]})
        (tile_showcase)
    });
    let buttons_switches=room("buttons-switches",false,html! {
        (Exhibit{name:"Button".into(),admin:false,piece:html!{div class="ui-exhibit__variants" {
            @for (label,tone,disabled) in [("Save",ButtonTone::Primary,false),("Cancel",ButtonTone::Secondary,false),("Remove",ButtonTone::Danger,false),("Upload Selected Files",ButtonTone::Primary,true)] {
                (Button{text:label.into(),tone:Some(tone),disabled,button_type:true,..Button::default()})
            }
        }},words:vec![
            "A button is for one action, and its label names that action. The ones shown here are for looking at: pressing them does nothing.".into(),
            "A filled button is the main action where it stands, such as Save, Apply or Upload Selected Files. An outlined button is a lesser action, such as Cancel, Refresh or Check for updates. Red marks an action to think twice about, such as Remove this name or Unpin. Not every button that removes something is red, so read the label.".into(),
            "A dimmed button cannot be pressed for now. It may be waiting for something, such as a file to upload or a change that is still being saved.".into(),
            "You will meet it on the Upload tab, and in Admin Mode on the Backups, DHCP, Firewall, DNS and Admin tabs.".into(),
        ]})
        (Exhibit{name:"On/off switch".into(),admin:false,piece:html!{div class="ui-exhibit__variants" {
            @for (label,checked) in [("Ad blocking",true),("Anonymize",false)] {
                (Toggle{label:Some(label.into()),size:Some(Size::Medium),checked,..Toggle::default()})
            }
        }},words:vec![
            "A switch shows whether one thing is on or off. Press it to flip it. The ones shown here flip and change nothing else.".into(),
            "On the DNS tab, Ad blocking takes effect as soon as you press it. Encrypted DNS, further down the same tab, takes effect when you press Save resolver. On the DHCP tab, Anonymize hides device names and part of each ID tag on your screen, for screenshots; nothing changes on your network.".into(),
            "In Admin Mode you will meet it on the DNS and DHCP tabs.".into(),
        ]})
        (Exhibit{name:"Icon button".into(),admin:false,piece:html!{div class="ui-exhibit__variants" {
            (IconButton{icon:Icon::Text("×"),size:Size::Small,shape:Shape::Circle,label:"Remove upload".into(),disabled:false,ghost:true,danger:true,hx:Default::default()})
            (IconButton{icon:Icon::Text("›"),size:Size::Large,shape:Shape::Square,label:"Device details".into(),disabled:false,ghost:true,danger:false,hx:Default::default()})
        }},words:vec![
            "An icon button is a button that shows a symbol and no words. The ones shown here do nothing.".into(),
            "On the Upload tab the × beside a file takes that upload off the list. In Admin Mode the × beside a blacklist entry takes it out of the list, and Submit saves the change. On the DHCP tab the › at the end of a device's row opens that device's details.".into(),
            "You will meet it on the Upload tab, and in Admin Mode on the DHCP tab.".into(),
        ]})
    });
    let messages=room("messages",false,html! {
        (Exhibit{name:"Toast".into(),admin:false,piece:html!{div class="ui-exhibit__variants" {
            (VisibleToast{kind:ToastKind::Success,message:"Successfully uploaded 2 file(s)".into()}.render_inert())
            (VisibleToast{kind:ToastKind::Warning,message:"Uploaded 2 file(s), 1 failed".into()}.render_inert())
        }},words:vec![
            "A toast is a short message that appears near the top of the page after something happens. It stays a few seconds, longer while the pointer rests on it, and then goes away. The ones shown here stay put.".into(),
            "Its colour is a hint: green for good news, orange for a warning, red for a failure. After an upload that worked you get one saying how many files went up.".into(),
            "You will meet it on the Upload tab, and in Admin Mode on tabs such as DNS and DHCP.".into(),
        ]})
        (Exhibit{name:"Loading spinner".into(),admin:false,piece:html!{div class="ui-exhibit__variants" {
            @for (size,label) in [(Size::Medium,"Loading network activity"),(Size::Small,"Loading")] {
                (LoadingSpinner{size,label:Some(label.into()),inline:true,hidden:false,hx:Default::default()})
            }
        }},words:vec![
            "A spinner is a small turning ring. It means the site is fetching or doing something and is not finished yet.".into(),
            "On the Stats tab it turns beside the words Loading network activity... for as long as that panel has no list to show. In Admin Mode it turns on the DHCP tab, beside Loading device identities…, until the device list has been read.".into(),
            "You will meet it on the Stats tab, and in Admin Mode on the DHCP tab.".into(),
        ]})
    });
    let readbacks=[("Capabilities",[("Component catalog","Enabled"),("Health checks","Enabled")]),("Settings",[("Default section","Component Showcase"),("Refresh interval","30 seconds")])];
    let editor=ConfigEditor{name_label:"Display name".into(),name:"Test catalog".into(),selection_label:"Default section".into(),choices:["Component Showcase","How it works"].into_iter().map(|label|ConfigChoice{label:label.into()}).collect(),toggle_label:"Automatic health checks".into(),checked:true,reset_label:"Reset".into(),save_label:"Save".into()};
    let table=SettingsTable{headings:["Setting".into(),"Value".into(),"Source".into(),"Status".into()],rows:[("Default tab","showcase","shell","Active"),("Health interval","30s","profile","Active"),("External actions","disabled","catalog","Specimen")].into_iter().map(|(setting,value,source,status)|SettingsRow{setting:setting.into(),value:value.into(),source:source.into(),status:status.into()}).collect()};
    let config=Panel{id:"config".into(),active:false,kind:PanelKind::Configuration,catalog_specimen:Some("test-configuration-patterns".into()),content:html!{
      (SubpanelHeader{kicker:"Configuration specimens".into(),title:"Configuration".into(),description:"Three appliance settings patterns.".into(),trailing:Badge::new("3 patterns").render()})
      (ViewLayout{kind:ViewLayoutKind::Configuration,content:html!{
       (ConfigPattern{pattern:"readback".into(),marker:"Pattern A".into(),title:"Sectioned Readback".into(),kind:ConfigKind::Readback,body:html!{@for (title,items) in readbacks{(ReadbackSection{title:title.into(),entries:items.into_iter().map(|(label,value)|ReadbackEntry{label:label.into(),value:value.into()}).collect()})}}})
       (ConfigPattern{pattern:"form".into(),marker:"Pattern B".into(),title:"Form Editor".into(),kind:ConfigKind::Form,body:editor.render()})
       (ConfigPattern{pattern:"grid".into(),marker:"Pattern C".into(),title:"Settings Grid".into(),kind:ConfigKind::Table,body:table.render()})
      }})
    }};
    let health = Panel {
        id: "health".into(),
        active: false,
        kind: PanelKind::Basic,
        catalog_specimen: None,
        content: HealthBoard {
            check_label: "Check again".into(),
            idle_caption: "Not checked yet".into(),
        }.render(),
    };
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
    let tabs=TabStrip{label:"Test sections".into(),depth:TabDepth::Panel,categories:false,tabs:[("how-it-works","How it works"),("tiles","Tiles"),("buttons-switches","Buttons & switches"),("messages","Messages"),("animations","Motion"),("showcase","Component Showcase"),("config","Configuration"),("health","Health Status")].into_iter().enumerate().map(|(i,(id,label))|TabChoice{id:id.into(),label:label.into(),active:i==0}).collect()};
    let tablet=Tablet{scope:"test".into(),tabs,panels:html!{(guide)(tiles)(buttons_switches)(messages)(animations)(showcase)(config)(health)}};
    Fragment::new("test",html!{(tablet)}).render()
}