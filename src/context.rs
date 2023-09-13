use std::path::PathBuf;
use std::{collections::HashMap, path::Path};

use crate::{Language, PolyglotTree, RawParseResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Handle {
    path: SrcOrPath,
    lang: Language,
} // TODO rename into SourceUniqIdentifier

impl From<(Language, &Path)> for Handle {
    fn from((lang, path): (Language, &Path)) -> Self {
        Handle {
            path: SrcOrPath::Path(path.into()),
            lang,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InternalHandle(usize); // TODO rename into Handle

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SrcOrPath {
    Source(std::sync::Arc<str>),
    Path(PathBuf),
}
pub struct TopoOrder(usize);
pub struct GlobalContext {
    pwd: PathBuf,
    root: InternalHandle,
    sources: Vec<(Handle, RawParseResult, Vec<(TopoOrder, InternalHandle)>)>, // TODO refactoring into separarted struct with a vec backend and multiple indexes ((path|source)+lang?pwd, usize)
    queue: Vec<InternalHandle>,
}

impl GlobalContext {
    pub fn new(path: PathBuf, root: RawParseResult) -> Self {
        let root_dir = path.parent().unwrap().into();
        Self::with_root_dir((path, root), root_dir)
    }
    pub fn with_root_dir((path, root): (PathBuf, RawParseResult), root_dir: PathBuf) -> Self {
        GlobalContext {
            pwd: root_dir,
            root: InternalHandle(0),
            sources: vec![(
                Handle {
                    path: SrcOrPath::Path(path.clone()),
                    lang: root.language,
                },
                root,
                Default::default(),
            )],
            queue: vec![],
        }
    }

    pub fn get(&self, handle: &Handle) -> Option<&PolyglotTree> {
        // self.sources
        //     .iter()
        //     .find(|(h, _, _)| h == handle)
        //     .map(|(_, tree, _)| tree)
        todo!()
    }

    pub fn get_raw(&self, handle: &InternalHandle) -> Option<&RawParseResult> {
        self.sources.get(handle.0).map(|(_, tree, _)| tree)
    }
    pub fn add_polyglot_tree(&mut self, handle: Handle, tree: RawParseResult) -> InternalHandle {
        let h = InternalHandle(self.sources.len());
        self.sources.push((handle.clone(), tree, Default::default()));
        self.queue.push(h.clone());
        h
    }

    pub fn set_solved(&mut self, root: InternalHandle) {
        self.queue.retain(|h| h != &root);
    }

    pub fn next_partial_polyglot_tree(&mut self) -> Option<(InternalHandle, RawParseResult)> {
        let handle = self.queue.pop()?;
        let (_, tree, _) = &self.sources[handle.0];
        Some((handle.clone(), tree.clone()))
        //todo!()
    }
}
