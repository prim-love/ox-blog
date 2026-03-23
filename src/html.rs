use slugify::slugify;
use std::path::Path;
use crate::blog;

pub fn build_blog_index<'a, I>(blogs: I) -> String
where
    I: Iterator<Item = &'a blog::Blog>
{
    let index = {
        let str = blogs.map(|x| format!
                  {
                      "<li><a href='/{}/index.html'></a>{}</li>",
                      slugify!(&x.name),
                      x.name,
                  })
                  .collect::<String>();

        format! { "<ul>{}</ul>", str }
    };

    format!
    {
        r#"<!DOCTYPE html>
<html>
    <head>
        <title>Index</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        index,
    }
}

pub fn build_blog_page(blog: &blog::Blog) -> String
{
    format!
    {
        r#"<!DOCTYPE html>
<html>
    <head>
        <title>{}</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        blog.name,
        blog.html,
    }
}

pub fn write<'a, I>(
    path: impl AsRef<Path>,
    blogs: I,
) -> std::io::Result<()>
where
    I: Iterator<Item = &'a blog::Blog> + Clone
{
    let loc = path.as_ref();

    let idx = build_blog_index(blogs.clone());

    std::fs::write(
        loc.join("index.html"),
        idx,
    )?;

    for blog in blogs
    {
        let slug = slugify!(&blog.name);

        let page = build_blog_page(blog);

        let path = loc
            .join(slug);

        std::fs::create_dir_all(&path)?;

        let path = path
            .join("index.html");

        std::fs::write(path, page)?;
    }

    Ok(())
}
