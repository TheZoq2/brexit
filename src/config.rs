use std::time::Duration;

use markup5ever_rcdom::{Handle, NodeData, RcDom};

use xml5ever::driver::parse_document;
use xml5ever::tendril::TendrilSink;

pub struct Config {
    duration: Duration,
}

impl Config {
    pub fn parse(config: &str) -> Self {
        let parser = parse_document(RcDom::default(), Default::default());

        let dom = parser.one(config);
        let doc = dom.document.clone();

        let duration = if Self::is_extended(doc) {
            Duration::from_secs(60 * 60 * 24 * 365 * 3 + 1)
        } else {
            Duration::from_secs(60 * 60 * 24 * 365 * 3)
        };

        Config { duration }
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    fn is_extended(doc: Handle) -> bool {
        for node in doc.children.borrow().iter() {
            if let Some(node) = Self::find_extended_node(node.clone()) {
                if let Some(text) = Self::find_extended_value(node) {
                    return text == "true";
                }
            };
        }

        false
    }

    fn find_extended_node(config: Handle) -> Option<Handle> {
        let children = &*config.children.borrow();

        let matches = children.iter().find(|node| {
            if let NodeData::Element { name, .. } = &node.data {
                &name.local == "extended"
            } else {
                false
            }
        });

        matches.map(|rc| rc.clone())
    }

    fn find_extended_value(node: Handle) -> Option<String> {
        let children = &*node.children.borrow();

        let mut iter = children.iter().filter_map(|node| {
            if let NodeData::Text { contents } = &node.data {
                Some(contents)
            } else {
                None
            }
        });

        iter.next().map(|c| c.borrow().to_string())
    }
}
