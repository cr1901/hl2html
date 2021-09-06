mod html;

// Re-exports
pub use html::emit as emit_hotlist_as_html;

// Imports
use crate::ast::{EntryKind, Hotlist, Folder, Note};

use std::error::Error;

trait Visitor {
    fn visit_folder_empty(&mut self, folder: &Folder) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
    fn visit_folder_pre(&mut self, folder: &Folder) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
    fn visit_folder_post(&mut self, folder: &Folder) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
    fn visit_note(&mut self, note: &Note) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
    fn visit_root_pre(&mut self, hotlist: &Hotlist) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
    fn visit_root_post(&mut self, hotlist: &Hotlist) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
}

fn traverse_hotlist<V: Visitor>(hl: &Hotlist, mut visitor: V) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    visitor.visit_root_pre(&hl)?;

    let mut stack = Vec::<&EntryKind>::new();
    for e in hl.entries.iter().rev() {
        stack.push(e)
    }

    let mut last_visited: Option<&EntryKind> = None;

    loop {
        let curr = stack.last();

        if let None = curr {
            break;
        }

        let curr = curr.unwrap();

        match curr {
            EntryKind::Folder(f) => {
                if f.entries.len() != 0 && !nodes_equal(f.entries.last(), last_visited) {
                    visitor.visit_folder_pre(f)?;
                    for e in f.entries.iter().rev() {
                        stack.push(e);
                    }
                } else {
                    if f.entries.len() == 0 {
                        visitor.visit_folder_empty(f)?;
                    } else {
                        visitor.visit_folder_post(f)?;
                    }

                    last_visited = Some(curr);
                    stack.pop();
                }
            }
            EntryKind::Note(n) => {
                visitor.visit_note(n)?;

                last_visited = Some(curr);
                stack.pop();
            }
        }
    }

    visitor.visit_root_post(&hl)?;

    Ok(())
}

fn nodes_equal<'a>(a: Option<&EntryKind<'a>>, b: Option<&EntryKind<'a>>) -> bool {
    if a.is_none() || b.is_none() {
        return false;
    } else {
        let a_ref = a.unwrap();
        let b_ref = b.unwrap();

        return std::ptr::eq(a_ref, b_ref);
    }
}
