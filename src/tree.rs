pub struct LlkTree {
    root: LlkTreeNode,
}

struct LlkTreeNode {
    symbol: char,
    children: Vec<LlkTree>,
}

impl LlkTree {
    pub(super) fn new(start_symbol: char) -> LlkTree {
        LlkTree {
            root: LlkTreeNode::new(start_symbol),
        }
    }

    pub(super) fn push(&mut self, symbol: char) -> &mut LlkTree {
        unimplemented!()
    }
}

impl LlkTreeNode {
    fn new(symbol: char) -> LlkTreeNode {
        LlkTreeNode {
            symbol,
            children: Vec::new(),
        }
    }
}
