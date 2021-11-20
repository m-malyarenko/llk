pub enum LlkTree {
    Node(LlkTreeNode),
    Leaf(char),
}

pub struct LlkTreeNode {
    symbol: char,
    production_id: Option<usize>,
    children: Vec<LlkTree>,
}

impl LlkTree {
    pub(super) fn new(root_symbol: char) -> LlkTree {
        LlkTree::Node(LlkTreeNode {
            symbol: root_symbol,
            production_id: None,
            children: Vec::new(),
        })
    }

    pub(super) fn push_node(&mut self, symbol: char) -> *mut LlkTree {
        if let LlkTree::Node(node) = self {
            let new_node = LlkTree::Node(LlkTreeNode {
                symbol,
                production_id: None,
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

    pub(super) fn set_production_id(&mut self, id: usize) {
        if let LlkTree::Node(node) = self {
            node.production_id = Some(id)
        } else {
            panic!(
                "LlkTree fatal error:\
                 unexpected set_production_id method on LlkTree::Leaf enum item"
            )
        }
    }

    pub fn lrn(&self) -> LlkTreeLrnIter {
        let mut iter = LlkTreeLrnIter {
            unvisited: Vec::new(),
        };
        iter.lrn(self, 0);
        iter
    }

    fn pretty_print(&self) -> String {
        fn inner(tree: &LlkTree, buffer: &mut String, mut ident: String, is_last: bool) {
            buffer.push_str(&ident);
            
            if is_last {
                buffer.push_str("+-");
                ident.push_str("  ");
            } else {
                buffer.push_str("|-");
                ident.push_str("| ");
            }

            let symbol = match tree {
                LlkTree::Node(node) => {
                    node.symbol
                }
                LlkTree::Leaf(symbol) => *symbol
            };
            
            buffer.push_str(&format!("{}\n", symbol));

            if let LlkTree::Node(node) = tree {
                for (idx, child) in node.children.iter().enumerate() {
                    inner(child, buffer, ident.clone(), idx == node.children.len() - 1);
                }
            }

        }

        let mut buffer = String::new();
        inner(self, &mut buffer, "".to_string(), true);
        buffer
    }
}

pub struct LlkTreeLrnIter<'a> {
    unvisited: Vec<(&'a LlkTree, usize)>,
}

impl<'a> LlkTreeLrnIter<'a> {
    fn lrn(&mut self, tree: &'a LlkTree, child_idx: usize) {
        if let LlkTree::Node(node) = tree {
            self.unvisited.push((tree, child_idx));

            if let Some(child) = node.children.get(self.unvisited.last().unwrap().1) {
                self.unvisited.last_mut().unwrap().1 += 1;
                self.lrn(child, 0);
            }
        } else {
            self.unvisited.push((tree, 0));
        }
    }
}

impl<'a> Iterator for LlkTreeLrnIter<'a> {
    type Item = (char, Option<usize>);

    fn next(&mut self) -> Option<(char, Option<usize>)> {
        let tree_node = self.unvisited.pop()?.0;

        match tree_node {
            LlkTree::Leaf(symbol) => {
                let (parent_tree, child_idx) = self
                    .unvisited
                    .pop()
                    .expect("LlkTreeIter fatal error: tree leaf without parent node");
                self.lrn(parent_tree, child_idx);
                Some((*symbol, None))
            }
            LlkTree::Node(node) => {
                if let Some((parent_tree, child_idx)) = self.unvisited.pop() {
                    self.lrn(parent_tree, child_idx);
                }

                Some((node.symbol, node.production_id))
            }
        }
    }
}

impl std::fmt::Display for LlkTree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.pretty_print())
    }
}
