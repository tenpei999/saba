use crate::renderer::dom::node::Node;
use crate::renderer::dom::node::Window;
use crate::renderer::html::token::HtmlTokeneizer;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::str::FromStr;

#[derive(Debug, Clone)]
pub struct HtmlParser {
  window::Rc<RefCell<Window>>,
  mode: InsertionMode,
  /// https://html.spec.whatwg.org/multipage/parsing.html#original-insertion-mode
  original_insertion_mode: InsertionMode,
  /// https://html.spec.whatwg.org/multipage/parsing.html#the-stack-of-open-elements
  stack_of_open_elements: Vec<Rc<RefCell<Node>>>,
  t: HtmlTokeneizer,
}

impl HtmlParser {
  pub fn new(t: HtmlTokeneizer) -> Self {
    Self {
      window: Rc::new(RefCell::new(Window::new())),
      mode: InsertionMode::Initial,
      original_insertion_mode: InsertionMode::Initial,
      stack_of_open_elements: Vec::new(),
      t,
    }
  }
}

/// https://html.spec.whatwg.org/multipage/parsing.html#the-insertion-mode
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InsertionMode {
  Initial,
  BeforeHtml,
  BeforeHead,
  InHead,
  AfterHead,
  InBody,
  Text,
  AfterBody,
  AfterAfterBody,
}

pub fn construct_tree(&mut self) -> Rc<RefCell<Window>> {
  let mut token = self.t.next();

  while token.is_some() {
    match self.mode {
      InsertionMode::Initial => {}
      InsertionMode::BeforeHtml => {}
      InsertionMode::BeforeHead => {}
      InsertionMode::InHead => {}
      InsertionMode::AfterHead => {}
      InsertionMode::InBody => {}
      InsertionMode::Text => {}
      InsertionMode::AfterBody => {}
      InsertionMode::AfterAfterBody = {}
    }
  }
}

impl HtmlParser {
  pub fn construct_tree(&mut self) -> Rc<RefCell<Window>> {
    let mut token = self.t.next();

    while token.is_some() {
      match self.mode {
        InsertionMode::Initial => {
          //DOCTYPEトークンをサポートしていないため、
          //<!doctype html>のようなトークンは文字トークンとして表される。
          //文字トークンは無視する。
          if let Some(HtmlToken::Char(_)) = token {
            token = self.t.next();
            continue;
          }

          self.mode = InsertionMode::BeforeHtml;
          continue;
        }

        InsertionMode::BeforeHtml => {
          match token {
            Some(HtmlToken::Char(c)) => {
              if c == ' ' || c == '\n' {
                token = self.t.next();
                continue;
              }
            }
            Some(HtmlToken::StartTag {
              ref tag,
              self_closing: _,
              ref attributes,
            }) => {
              if tag == "html" {
                self.insert_element(tag, attributes.to_vec());
                self.mode = InsertionMode::BeforeHead;
                token = self.t.next();
                continue;
              }
            }
            Some(HtmlToken::Eof) | None => {
              return self.window.clone();
            }
            _=> {}
          }
          self.insert_element("html", Vec::new());
          self.mode = InsertionMode::BeforeHead;
          continue;
        }

        InsertionMode::BeforeHead => {
          match token {
            Some(HtmlToken::Char(c)) => {
              if c == ' ' || c == '\n' {
                token = self.t.next();
                continue;
              }
            }
            Some(HtmlToken::StartTag {
              ref tag,
              self_closing: _,
              ref attributes,
            }) => {
              if tag == "head" {
                self.insert_element(tag, attributes.to_vec());
                self.mode = InsertionMode::InHead;
                token = self.t.next();
                continue;
              }
            }
            Some(HtmlToken::Eof) | None => {
              return.self.window.clone();
            }
            _ => {}
          }
          self.insert_element("head", Vec::new());
          self.mode = InsertionMode::InHead;
          continue;
        }

        InsertionMode::InHead => {
          match token {
            Some(HtmlToken::Char(c)) => {
              if c == ' ' || c == '\n' {
                token = self.t.next();
                continue;
              }
            }
            Some(HtmlToken::StartTag {
              ref tag,
              self_closing: _,
              ref attributes,
            }) => {
              if tag == "style" || tag == "script" {
                self.insert_element(tag, attributes.to_vec());
                self.original_insertion_mode = self.mode;
                self.mode = InsertionMode::Text;
                token = self.t.next();
                continue;
              }
              // 仕様書には定められていないが、このブラウザは仕様を
              // すべて実装しているわけではないので、<head>が省略
              // されているhtml文書を扱うために必要。これがないと
              // <head>が省略されているHTML文書で無限ループが発生
              if tag == "body" {
                self.pop_until(ElementKind::Head);
                self.mode = InsertionMode::AfterHead;
                continue;
              }
              if let Ok(_element_kind) = ElementKind::from_str(tag) {
                self.pop_until(ElementKind::Head);
                self.mode = InsertionMode::AfterHead;
                continue;
              }
            }
            Some(HtmlToken::EndTag { ref tag }) => {
              if tag == "head" {
                self.mode = InsertionMode::AfterHead;
                token = self.t.next();
                self.pop_until(ElementKind::Head);
                continue;
              }
            }
            Some(HtmlToken::Eof) | None => {
              return self.window.clone();
            }
          }
          // <meta>や<title>などのサポートしていないタグは無視する
          token = self.t.next();
          continue;
        }

        InsertionMode::InBody => {
          match token {
            Some(HtmlToken::EndTag { ref tag }) => {
              match tag.as_str() {
                "body" => {
                  self.mode = InsertionMode::AfterBody;
                  token = self.t.next();
                  if !self.contain_in_stack(ElementKind::Body) {
                    // パースの試合。トークンを無視する
                    continue;
                  }
                }
                "html" => {
                  if self.pop_current_node(ElementKind::Body) {
                    self.,mode = InsertionMode::AfterBody;
                    assert!(self.pop_current_node(ElementKind::Html));
                  } else {
                    token = self.t.next();
                  }
                  continue;
                }
              }
              _ => {
                token = self.t.next();
              }
            }
            Some(HtmlToken::Eof) | None {
              return self.window.clone();
            }
            _ => {}
          }
        }

        InsertionMode::Text => {
          match token {
            Some(HtmlToken::Eof) | None {
              return self.window.clone();
            }
            Some(HtmlToken::EndTag { ref tag }) => {
              if tag == "style" {
                self.pop_until(ElementKind::Style);
                self.mode = self.original_insertion_mode;
                token = self.t.next();
                continue;
              }
              if tag == "script" {
                self.pop_until(ElementKind::Script);
                self.mode = self.original_insertion_mode;
                token = self.t.next();
                continue;
              }
            }
            Some(HtmlToken::Char(c)) => {
              self.insert_char(c);
              token = self.t.next();
              continue;
            }
            _ => {}
          }

          self.mode = self.original_insertion_mode;
        }

        InsertionMode::AfterBody => {
          match token {
            Some(HtmlToken::Char(_c)) => {
              token = self.t.next();
              continue;
            }
            Some(HtmlToken::EndTag { ref tag }) => {
              if tag == "html" {
                self.mode = InsertionMode::AfterAfterBody;
                token = self.t.next();
                continue;
              }
            }
            Some(HtmlToken::Eof) | None {
              return self.window.clone();
            }
            _ => {}
          }

          self.mode = InsertionMode::InBody;
        }

        InsertionMode::AfterAfterBody => {
          match token {
            Some(HtmlToken::Char(_c)) => {
              token = self.t.next();
              continue;
            }
            Some(HtmlToken::Eof) | None => {
              return self.window.clone();
            }
            _ => {}
          }

          //パースの失敗
          self.mode = InsertionMode::InBody
        }
      }
    }

    self.window.clone()
  }
}