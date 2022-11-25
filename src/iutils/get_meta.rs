use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct SiteMeta {
    pub title: String,
    pub description: String,
    pub icon: String,
    pub url: String,
}

pub async fn get_meta(url: &str) -> Result<SiteMeta> {
    let content = reqwest::get(url).await?.text().await?;

    let doc = Document::from(content.as_str());
    let title = doc
        .find(Name("title"))
        .next()
        .map(|node| node.text())
        .unwrap_or_default();

    let desc = doc
        .find(Name("meta"))
        .filter_map(|n| {
            let attr = n.attr("name");
            let is_desc = match attr {
                Some(v) => {
                    if v.eq("description") {
                        n.attr("content")
                    } else {
                        None
                    }
                }
                None => None,
            };
            is_desc
        })
        .next()
        .unwrap_or_default();

    let icon = doc
        .find(Name("link"))
        .filter_map(|n| {
            let attr = n.attr("rel");
            let is_desc = match attr {
                Some(v) => {
                    if v.eq("icon") {
                        n.attr("href")
                    } else {
                        None
                    }
                }
                None => None,
            };
            is_desc
        })
        .next()
        .unwrap_or_default();

    Ok(SiteMeta {
        title,
        description: String::from(desc),
        icon: String::from(icon),
        url: String::from(url),
    })
}
