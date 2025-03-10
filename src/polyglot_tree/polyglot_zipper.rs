use tree_sitter::{Node, TreeCursor};

use super::util::{InvalidArgumentError, Language};

use super::PolyglotTree;

/// A PolyglotZipper is an object based on a PolyglotTree, which contains one of the tree's nodes.
/// Zippers allow navigation of the tree and retrieval of node properties for analysis tasks.

#[derive(Default)]
pub struct Test {
    n: u32,
}

pub struct PolyglotZipper<'a> {
    tree: &'a PolyglotTree,
    node: TreeCursor<'a>,
}

impl std::fmt::Debug for PolyglotZipper<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.tree).finish()
    }
}

impl PolyglotZipper<'_> {
    /// Returns a new zipper for the given tree, located at the root.
    pub fn from(tree: &'_ PolyglotTree) -> PolyglotZipper<'_> {
        Self::from_impl(tree, tree.root_node())
    }

    fn from_impl<'a>(tree: &'a PolyglotTree, node: Node<'a>) -> PolyglotZipper<'a> {
        PolyglotZipper {
            tree,
            node: node.walk(),
        }
    }

    fn node(&self) -> Node {
        self.node.node()
    }

    /// Returns true if the contained node is a polyglot eval call.
    pub fn is_polyglot_eval_call(&self) -> bool {
        self.tree.is_polyglot_eval_call(self.node())
    }

    /// Returns true if the contained node is a polyglot import call.
    pub fn is_polyglot_import_call(&self) -> bool {
        //println!("zipper - passage dans la fonction import");
        self.tree.is_polyglot_import_call(self.node())
    }

    /// Returns true if the contained node is a polyglot export call.
    pub fn is_polyglot_export_call(&self) -> bool {
        //println!("zipper - passage dans la fonction export");
        self.tree.is_polyglot_export_call(self.node())
    }

    /// Get the contained node's type as a string.
    ///
    /// For polyglot nodes, this is one of either `"polyglot_eval_call"`, `"polyglot_import_call"` or `"polyglot_export_call"`.
    pub fn kind(&self) -> &str {
        if self.is_polyglot_eval_call() {
            return "polyglot_eval_call";
        } else if self.is_polyglot_import_call() {
            return "polyglot_import_call";
        } else if self.is_polyglot_export_call() {
            return "polyglot_export_call";
        }
        self.node().kind()
    }
    /// Get the contained node's source code as a string.
    pub fn code(&self) -> &str {
        self.tree.node_to_code(self.node())
    }

    /// Get the contained node's start position in terms of rows and columns.
    pub fn start_position(&self) -> tree_sitter::Point {
        self.node().start_position()
    }

    /// Get the contained node's end position in terms of rows and columns.
    pub fn end_position(&self) -> tree_sitter::Point {
        self.node().end_position()
    }

    pub fn get_binding_name(&self) -> Result<String, InvalidArgumentError> {
        if self.is_polyglot_import_call() || self.is_polyglot_export_call() {
            return match self.get_lang() {
                Language::Python => match self.get_python_binding() {
                    Some(s) => Ok(s),
                    None => Err(InvalidArgumentError), // todo: make this into a proper error enum
                },
                Language::JavaScript => todo!(),
                Language::Java => todo!(),
            };
        }
        Err(InvalidArgumentError)
    }

    fn get_python_binding(&self) -> Option<String> {
        Some(String::from(self.child(1)?.child(1)?.code()))
    }

    /// Get the Language associated with the contained node.
    pub fn get_lang(&self) -> &Language {
        //println!("zipper - vérification du langage");
        &self.tree.language
    }

    /// Move this zipper to the first child of the contained node.
    /// Returns `true` if there were any children, otherwise returns `false` and does not move.
    pub fn goto_first_child(&mut self) -> bool {
        //println!("zipper - goto first child");
        let my_id = self.node().id();
        let subtree = self.tree.node_to_subtrees_map.get(&my_id);

        match subtree {
            Some(t) => {
                self.tree = t;
                self.node = t.root_node().walk();
                true
            }

            None => self.node.goto_first_child(),
        }
    }

    /// Move this zipper to the first sibling of the contained node.
    /// Returns `true` if there were any siblings, otherwise returns `false` and does not move.
    pub fn goto_next_sibling(&mut self) -> bool {
        //println!("zipper - goto next sibling");
        self.node.goto_next_sibling()
    }

    /// Get the zipper for the child at the given index, where zero represents the first child.
    pub fn child(&self, i: usize) -> Option<PolyglotZipper> {
        if self.is_polyglot_eval_call() {
            // if we are an eval call, we actually want to jump to the corresponding subtree
            let my_id = self.node().id();
            let subtree = self.tree.node_to_subtrees_map.get(&my_id)?;
            return Some(Self::from(subtree));
        }

        Some(Self::from_impl(self.tree, self.node.node().child(i)?))
    }

    /// Get the zipper for the next sibling node.
    pub fn next_sibling(&self) -> Option<PolyglotZipper> {
        Some(Self::from_impl(self.tree, self.node().next_sibling()?))
    }

    /// Get the zipper for the previous sibling node.
    pub fn prev_sibling(&self) -> Option<PolyglotZipper> {
        Some(Self::from_impl(self.tree, self.node().prev_sibling()?))
    }
}
