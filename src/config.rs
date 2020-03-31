use std::time::Duration;

use markup5ever_rcdom::NodeData::{Element, Text};
use markup5ever_rcdom::{Handle, RcDom};

use xml5ever::driver::parse_document;
use xml5ever::tendril::TendrilSink;

const SECONDS: u64 = 60 * 60 * 24 * 365 * 3;

pub struct Config {
    dom: RcDom,
}

impl Config {
    pub fn new(config: &str) -> Self {
        let sink = RcDom::default();
        let opts = Default::default();

        let xml = parse_document(sink, opts);
        let dom = xml.one(config);

        Config { dom }
    }

    pub fn duration(&self) -> Duration {
        if self.should_extend() {
            Duration::from_secs(SECONDS + 1)
        } else {
            Duration::from_secs(SECONDS)
        }
    }

    fn should_extend(&self) -> bool {
        let document = &self.dom.document;
        let children = document.children.borrow();

        children
            .iter()
            .filter_map(Self::find_extend_node)
            .filter_map(Self::find_extend_text)
            .map(|text| text.trim().to_lowercase())
            .any(|text| text == "true")
    }

    fn find_extend_node(handle: &Handle) -> Option<Handle> {
        for node in &*handle.children.borrow() {
            if let Element { name, .. } = &node.data {
                if &name.local == "extend" {
                    return Some(node.clone());
                }
            }
        }

        None
    }

    fn find_extend_text(handle: Handle) -> Option<String> {
        for node in &*handle.children.borrow() {
            if let Text { contents } = &node.data {
                return Some(contents.borrow().to_string());
            }
        }

        None
    }
}
