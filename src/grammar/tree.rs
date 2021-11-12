pub enum LlkTree {
    Node(LlkTreeNode),
    Leaf(char),
}

pub struct LlkTreeNode {
    symbol: char,
    children: Vec<LlkTree>,
}

impl LlkTree {
    pub(super) fn new(root_symbol: char) -> LlkTree {
        LlkTree::Node(LlkTreeNode {
            symbol: root_symbol,
            children: Vec::new(),
        })
    }

    pub(super) fn push_node(&mut self, symbol: char) -> *mut LlkTree {
        if let LlkTree::Node(node) = self {
            let new_node = LlkTree::Node(LlkTreeNode {
                symbol,
                children: Vec::new(),
            });
            node.children.push(new_node);
            node.children.last_mut().unwrap() as *mut LlkTree
        } else {
            panic!(
                "LlkTree fatal error:\
                 unexpected push_node method on LlkTree::Leaf enum item"
            )
        }
    }

    pub(super) fn push_leaf(&mut self, symbol: char) {
        if let LlkTree::Node(node) = self {
            let new_node = LlkTree::Leaf(symbol);
            node.children.push(new_node);
        } else {
            panic!(
                "LlkTree fatal error:\
                 unexpected push_leaf method on LlkTree::Leaf enum item"
            )
        }
    }

    pub fn iter(&self) -> LlkTreeIter {
        let mut iter = LlkTreeIter {
            unvisited: Vec::new(),
        };
        iter.lnr(self, 0);
        iter
    }
}

impl<'a> IntoIterator for &'a LlkTree {
    type Item = &'a char;
    type IntoIter = LlkTreeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct LlkTreeIter<'a> {
    unvisited: Vec<(&'a LlkTree, usize)>,
}

impl<'a> LlkTreeIter<'a> {
    fn lnr(&mut self, tree: &'a LlkTree, child_idx: usize) {
        if let LlkTree::Node(node) = tree {
            self.unvisited.push((tree, child_idx));

            if let Some(child) = node.children.get(self.unvisited.last().unwrap().1) {
                self.unvisited.last_mut().unwrap().1 += 1;
                self.lnr(child, 0);
            }
        } else {
            self.unvisited.push((tree, 0));
        }
    }
}

impl<'a> Iterator for LlkTreeIter<'a> {
    type Item = &'a char;

    fn next(&mut self) -> Option<&'a char> {
        let tree_node = self.unvisited.pop()?.0;

        match tree_node {
            LlkTree::Leaf(symbol) => {
                let (parent_tree, child_idx) = self
                    .unvisited
                    .pop()
                    .expect("LlkTreeIter fatal error: tree leaf without parent node");
                self.lnr(parent_tree, child_idx);
                Some(symbol)
            }
            LlkTree::Node(node) => {
                if let Some((parent_tree, child_idx)) = self.unvisited.pop() {
                    self.lnr(parent_tree, child_idx);
                    Some(&node.symbol)
                } else {
                    None
                }
            }
        }
    }
}
