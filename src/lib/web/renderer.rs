use crate::web::ctx;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("rendering error: {0}")]
    Render(#[from] handlebars::RenderError),
}

pub struct Renderer<'a>(handlebars::Handlebars<'a>);

impl<'a> Renderer<'a> {
    pub fn new(template_dir: std::path::PathBuf) -> Self {
        let mut renderer = handlebars::Handlebars::new();
        renderer
            .register_templates_directory(".hbs", &template_dir)
            .expect("failed to register handlebars templates");

        Self(renderer)
    }

    fn convert_to_value<S>(serializable: &S) -> serde_json::Value
    where
        S: serde::Serialize + std::fmt::Debug,
    {
        // serde_json::to_value(&serializable).expect("failed to convert structure to value")
        // the code above is not wrong but according to: https://rust-lang.github.io/rust-clippy/master/index.html#/needless_borrows_for_generic_args
        // The lint cannot tell when the implementation of a trait for &T and T do different things. Removing a borrow in such a case can change the semantics of the code.
        serde_json::to_value(serializable).expect("failed to convert structure to value")
    }

    pub fn render<PageCtx>(&self, context: PageCtx, errors: &[&str]) -> String
    where
        PageCtx: ctx::PageContext + serde::Serialize + std::fmt::Debug,
    {
        let mut value = Self::convert_to_value(&context);

        if let Some(value) = value.as_object_mut() {
            value.insert("_errors".into(), errors.into());
            value.insert("_title".into(), context.title().into());
            value.insert("_base".into(), context.parent().into());
        }

        self.do_render(context.template_path(), value)
    }

    pub fn render_with_data<PageCtx, D>(
        &self,
        context: PageCtx,
        data: (&str, D),
        errors: &[&str],
    ) -> String
    where
        PageCtx: ctx::PageContext + serde::Serialize + std::fmt::Debug,
        D: serde::Serialize + std::fmt::Debug,
    {
        use handlebars::to_json;

        let mut value = Self::convert_to_value(&context);

        if let Some(value) = value.as_object_mut() {
            value.insert("_errors".into(), errors.into());
            value.insert("_title".into(), context.title().into());
            value.insert("_base".into(), context.parent().into());
            value.insert(data.0.into(), to_json(data.1));
        }

        self.do_render(context.template_path(), value)
    }

    fn do_render(&self, path: &str, ctx: serde_json::Value) -> String {
        self.0.render(path, &ctx).expect("error rendering template")
    }
}
