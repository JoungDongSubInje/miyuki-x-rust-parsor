use maud::{Markup, html};
use x_post::*;

pub struct HtmlPost;

impl HtmlPost{
    // vector가 들어 있는 데이터 구조를 HTML로 변환하는 메서드
    pub fn from_v_to_html(posts: XPostingList) -> Vec<MarkDown> {
        let mut v_markdown: Vec<MarkDown>= Vec::new();

        for post in posts{
            // HTMl에 링크를 주입
            let link= Url::get_url();
            let markup= html!{
                a href=(link) { "x_post" }
            }
            let opt_markup= Option::from(markup);
            let parsed= match opt_markup {
                Some(markup) => markup,
                _ => panic!("Parsing err on post struct to Html struct")
            }

            let markdown= MarkDown::new(parsed);
            
            v_markdown.push(markdown);
        }

        v_markdown
    }
}

struct MarkDown{
    htmls: Vec<maud::PreEscaped<String>>,
}

impl MarkDown{
    pub fn new(html: Html) -> Self{
        MarkDown { html }
    }

    // HTML을 마크다운으로 변환하는 메서드
    // fn to_markdown(&self, post: T) -> String {
    //     let html_string = HtmlPosting::to_html();
    //     // 간단한 변환 예시 (실제 변환 로직은 더 복잡할 수 있음)
        
    // }
}

fn main(){
    // let v:Vec<usize>= Vec::new();
    // let html: HtmlPosting<usize> = HtmlPosting::new(v);

    let posts= x_post::XPostingList::new();
    // let post_query
    
    HtmlPost::from_v_to_html(
        x_post::KoreaXPosting::push_x_posting(posts, "/dongsub/123145648")
    );

}