use maud::{Markup, html};
use x_post::*;

pub trait ToFormat{
    fn from_v_to_html(v_markdown: Vec<MarkDown>, posts: XPostingList)
}

impl ToFormat for maud{
    // vector가 들어 있는 데이터 구조를 HTML로 변환하는 메서드
    fn from_v_to_html(posts: XPostingList) -> Vec<MarkDown> {

        for post in posts.postings{

            // from Url struct to html markup on Html 
            let link= Url::get_url(&post.url); 
            let markup= html!{
                a href=(link) { "x_post" }
            };

            let markdown= MarkDown::new(markup);
            
            v_markdown.push(markdown);
        }

        v_markdown
    }
}

fn main(){
    let str_url= "/jds_invoker/123456789";
    let x_post= XPosting::new(str_url);

    // Box<Vec<XPostingList>>    
    let mut x_posts= XPostingList::new();
    x_posts.add_x_post(x_post);

    // tansfer html struct
    let mut v_markdown: Vec<MarkDown>= Vec::new();
    let html_posts: Vec<MarkDown>= HtmlPost::from_v_to_html(x_posts);
}