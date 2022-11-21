use rowan::GreenNode;
use syntax::SyntaxNode;

use crate::parse_error::ParseError;

pub struct Output {
    pub green_node: GreenNode,
    pub errors: Vec<ParseError>,
}

impl Output {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);
        formatted[0..formatted.len() - 1].to_string()
    }
}
