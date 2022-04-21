#![feature(decl_macro)]
#[macro_use] extern crate rocket;

extern crate reqwest;
//use reqwest::blocking::Response;
use rocket::response::content::Html;
use rocket::http::RawStr;
use rocket_contrib::serve::StaticFiles;

extern crate url;
use url::Url;

extern crate serde;


struct Index
{
    name : String,
    url : Url
}

use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubmedResponse {
    pub q: String,
    #[serde(rename = "num_hits")]
    pub num_hits: i64,
    pub hits: Vec<Hit>,
    //pub timings: Timings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit {
    pub score: f64,
    pub doc: Doc,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Doc {
    pub body: Vec<String>,
    pub journal: Option<Vec<String>>,
    pub title: Vec<String>,
    pub url: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timings {
    pub timings: Vec<Timing>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
    pub name: String,
    pub duration: i64,
    pub depth: i64,
}


#[get("/?<q>")]
fn index(q:Option<&RawStr>) -> Html<String>
{
    let indexes = vec![
        Index{name: String::from("pubmed"),   url : Url::parse("https://osf.creative-memory.eu/osf/api/0.0/index/cm/pubmed/en/tantivy/api/0.0").unwrap()},
        Index{name: String::from("wikipedia"), url : Url::parse("https://osf.creative-memory.eu/osf/api/0.0/index/cm/wikipedia/en/tantivy/api/0.0").unwrap()},
        Index{name: String::from("wikipedia-de-abstract"), url : Url::parse("https://osf.creative-memory.eu/osf/api/0.0/index/cm/wikipedia/de-abstract/tantivy/api/0.0").unwrap()},
    ];

    let mut query_value = String::new();

    let mut results = String::new();
    let mut labels = String::new();
    let mut tabs = String::new();
    if let Some(query) = q
    {
        let mut query_str = String::from("?q=");
        query_str.push_str(&query.html_escape());
        query_value.push_str(&query.html_escape());
        for (i, index) in indexes.iter().enumerate()
        {
            if let Ok(response) = reqwest::blocking::get(index.url.join(&query_str).unwrap().to_string())
            {
                if let Ok(json) = response.json::<PubmedResponse>()
                {
                    tabs.push_str(&format!(
                            r#"<input id="tab{}" type="radio" name="tabs" />"#,
                            i + 1));
            
                    labels.push_str(&format!(
                            r#"<label for="tab{}">
                                 <span class="engine">{}</span>
                                 <span class="hits">{}</span>
                               </label>"#,
                               i + 1, index.name, json.num_hits));

                    let mut hits_list = String::new();
                    for (rank,hit) in json.hits.iter().enumerate()
                    {
                        let title = RawStr::from_str(hit.doc.title.first().unwrap()).html_escape();
                        let body = hit.doc.body.iter().map(|b| RawStr::from_str(b).html_escape()).collect::<Vec<_>>().join("<br>");
                        let mut journal = &index.name;
                        if let Some(journals) = &hit.doc.journal
                        {
                            journal = journals.first().unwrap();
                        }
                        let url = hit.doc.url.as_ref().unwrap_or(&vec![]).join("; ");
                        hits_list.push_str(
                           &format!(
                            r##"
                            <span class="result">
                              <h4><a href="{url}">{title}</a></h4>
                              <div><small>{journal}</small></div>
                              <a href="#result-long-{i}-{rank}">
                                {body}
                              </a>
                              <div id="result-long-{i}-{rank}" class="result-long">
                              <a href="#" class="overlay-close"></a>
                              <div>
                                <a href="#" title="close" class="modal-close">close</a>
                                <h4>{title}</h4>
                                {body}
                              </div>
                              </div>
                            </span>
                            "##
                           ));
                    }
                    results.push_str(&format!(r#"<div class="tab{}">{hits_list}</div>"#, i + 1));
                }
                else
                {
                    labels.push_str(&format!(r#"<label for="tab{}"><span class="engine">{}</span> <span class="hits">{}</span></label>"#, i+1, index.name, 0));
                    results.push_str(&format!(r#"<div class="tab{}">no hits</div>"#, i + 1));
                }
            }
            else
            {
                results.push_str(&format!(r#"<div class="tab{}">no response from index</div>"#, i + 1));
            }
        }

    }
    Html(format!(
    r#"
<!DOCTYPE html>
<html>
    <head>
        <title>OSF Search</title>
        <meta charset="UTF-8">
        <meta name="author" content="github.com/ahcm">
        <meta name="description" content="OSF Search">
        <meta name="keywords" content="Open Search Foundation Search">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <link href="img/favicon.ico" rel="shortcut icon" type="image/x-icon">
        <link href="css/base.css" rel="stylesheet">
        <link href="css/framework.css" rel="stylesheet">
        <link href="css/responsive.css" rel="stylesheet">
    </head>
    <body>
        <div id="header"><div>
            <div id="col-logo">
                <img src="img/osf-logo.png" alt="OSF Search" srcset="img/osf-logo.svg">
            </div>
            <div id="col-search">
                <form action="" method="get">
                    <span aria-hidden="true">&#128269;</span>
                    <input name="q" id="q" type="search" value="{query_value}" placeholder="Search..." />
                    <button name="search" id="search" type="submit">Go</button>  
                </form>
            </div>
            <div id="col-link">
                <a href="https://opensearchfoundation.org/" target="_blank">OpenSearchFoundation.org</a>
            </div>
        </div></div>
        
        <div id="content">
            {tabs}

            <nav id="provider">
                {labels}
            </nav>
            
            <div id="results">
                {results}
            </div>
        </div>
    </body>
</html>
"#
    ))
}

//#[launch]
fn rocket() -> rocket::Rocket
{
    rocket::ignite()
        .mount("/css", StaticFiles::from("css"))
        .mount("/img", StaticFiles::from("img"))
        .mount("/", routes![index])
}

fn main()
{
    rocket().launch();
}
