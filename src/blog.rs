use org::rowan::ast::AstNode;
use orgize as org;
use org::ast::*;

#[derive(PartialEq, Eq, Clone, Debug, derive_more::Deref)]
pub struct Path(pub Vec<Token>);

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Blog
{
    pub name: String,
    pub html: String,
    pub path: Path,
}

pub fn from_document(d: &Document) -> impl Iterator<Item = Blog> + use<>
{
    d.headlines()
        .filter(|x| x.tags().any(|x| x == "blog"))
        .map(|x| Blog::from_headline(&x))
        .flatten()
}

impl Blog
{
    pub fn from_headline(h: &Headline) -> Option<Blog>
    {
        let html = {
            let mut html = org::export::HtmlExport::default();
            html.render(h.syntax());
            html.finish()
        };

        let blog = Blog
        {
            path: get_blog_path(h)?,
            name: get_blog_name(h),
            html,
        };

        Some(blog)
    }
}

impl Path
{
    pub fn from_tags<I>(mut iter: I) -> Option<Path>
    where
        I: Iterator<Item = Token>,
    {
        if iter.next()? != "blog"
        {
            return None;
        }

        Some(Path(iter.collect()))
    }
}

pub fn get_blog_path(h: &Headline) -> Option<Path>
{
    Path::from_tags(h.tags())
}

pub fn get_blog_name(h: &Headline) -> String
{
    h.title_raw()
}
