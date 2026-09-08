//! The shallow author face; no second theme catalog.
macro_rules! author_face {
    ($($variant:ident => $name:literal),+ $(,)?) => {
        #[derive(Clone,Copy,Debug,Eq,PartialEq)]
        pub enum Theme {$($variant),+}
        impl Theme {
            pub const ALL: &'static [Self]=&[$(Self::$variant),+];
            pub const fn name(self)->&'static str {match self {$(Self::$variant=>$name),+}}
            pub fn from_name(name:&str)->Option<Self>{match name {$($name=>Some(Self::$variant)),+, _=>None}}
            pub fn variable(self)->String {format!("var(--{})",self.name())}
        }
    }
}
author_face! {
    Background=>"background", BackgroundAlt=>"background-alt", BackgroundAltHover=>"background-alt-hover", Surface=>"surface", SurfaceSoft=>"surface-soft",
    Text=>"text", TextSecondary=>"text-secondary", TextMuted=>"text-muted", Secondary=>"secondary",
    Primary=>"primary", PrimaryHover=>"primaryHover", Accent=>"accent", AccentSoft=>"accent-soft",
    Border=>"border", Error=>"error", Warning=>"warning", Success=>"success", Info=>"info",
    StatusUp=>"status-up", StatusDown=>"status-down", StatusPartial=>"status-partial", StatusUnknown=>"status-unknown",
    HiddenTabBackground=>"hiddenTabBackground", HiddenTabText=>"hiddenTabText", Shadow=>"shadow",
    PrimaryRgb=>"primary-rgb", BackgroundRgb=>"background-rgb",
    LegacyStatusUp=>"statusUp", LegacyStatusDown=>"statusDown", LegacyStatusPartial=>"statusPartial", LegacyStatusUnknown=>"statusUnknown",
    BorderRadius=>"border-radius", FontSizeSm=>"font-size-sm", TransitionFast=>"transition-fast", MonoFont=>"monoFont",
    TabBorder=>"tabBorder", TabContentBackground=>"tabContentBackground", Disabled=>"disabled", BorderColor=>"border-color"
}
/// CSS tokenization decodes escapes and ignores strings/comments. Both custom
/// property declarations and references pass through the same name check.
pub fn lint(css: &str) -> Result<(), String> {
    use cssparser::{ParseError, Parser, ParserInput, Token};
    fn walk<'i, 't>(parser: &mut Parser<'i, 't>) -> Result<(), ParseError<'i, String>> {
        while !parser.is_exhausted() {
            let token = parser.next_including_whitespace_and_comments()?.clone();
            match token {
                Token::Ident(name) if name.starts_with("--") => {
                    if Theme::from_name(&name[2..]).is_none() {
                        return Err(parser.new_custom_error(format!(
                            "custom property outside author face: {name}"
                        )));
                    }
                }
                Token::Function(_)
                | Token::ParenthesisBlock
                | Token::SquareBracketBlock
                | Token::CurlyBracketBlock => parser.parse_nested_block(walk)?,
                Token::BadString(_)
                | Token::BadUrl(_)
                | Token::CloseParenthesis
                | Token::CloseSquareBracket
                | Token::CloseCurlyBracket => {
                    return Err(parser.new_custom_error("malformed CSS token"))
                }
                _ => {}
            }
        }
        Ok(())
    }
    let mut input = ParserInput::new(css);
    walk(&mut Parser::new(&mut input)).map_err(|e| format!("{e:?}"))
}
