use crate::alloc::string::ToString;
use crate::browser::Browser;
use crate::http::HttpResponse;
use crate::renderer::dom::node::Window;
use crate::renderer::html::parser::HtmlParser;
use crate::renderer::html::token::HtmlTokeneizer;
use crate::utils::convert_dom_to_string;
use alloc::rc::Rc;
use alloc::rc::Weak;
use alloc::string::String;
use core::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Page {
  browser: Weak<RefCell<Browser>>,
  frame: Option<Rc<RefCell<Window>>>,
}

impl Page {
  pub fn new() -> Self {
    Self {
      browser: Weak::new(),
      frame: None,
    }
  }
  
  pub fn set_browser(&mut self, browser: Weak<RefCell<Browser>>) {
    self.browser = browser;
  }

  pub fn receive_response(&mut self, response: HttpResponse) -> String {
    self.create_frame(response.body());

    //デバック用にDOMツリーを文字列として返す
    if let Some(frame) = &self.frame {
      let dom = frame.borrow().document().clone();
      let debug = convert_dom_to_string(&Some(dom));
      return debug;
    }

    "".to_string()
  }

  fn create_frame(&mut self, html: String) {
    let html_tokenizer = HtmlTokeneizer::new(html);
    let frame = HtmlParser::new(html_tokenizer).construct_tree();
    self.frame = Some(frame);
  }
}