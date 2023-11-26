use derive_more::Constructor;
use serde::Serialize;

pub trait PageContext {
    fn title(&self) -> &str;
    fn template_path(&self) -> &str;
    fn parent(&self) -> &str;
}

#[derive(Debug, Serialize, Default)]
pub struct Home {}

impl PageContext for Home {
    fn title(&self) -> &str {
        "Share Your Clipboard"
    }

    fn template_path(&self) -> &str {
        "home"
    }

    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct ViewClip {
    pub clip: crate::Clip,
}

impl PageContext for ViewClip {
    fn title(&self) -> &str {
        "View Clip"
    }

    fn template_path(&self) -> &str {
        "clip"
    }

    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct PasswordRequired {
    short_code: crate::ShortCode,
}

impl PageContext for PasswordRequired {
    fn title(&self) -> &str {
        "Password Required"
    }

    fn template_path(&self) -> &str {
        "clip_need_password"
    }

    fn parent(&self) -> &str {
        "base"
    }
}
