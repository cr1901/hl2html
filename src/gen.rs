mod html;
mod tiddler_json;

// Re-exports
pub use html::emit as emit_hotlist_as_html;
pub use tiddler_json::emit as emit_hotlist_as_tiddler_json;

// Imports
use crate::ast::{EntryKind, Folder, Hotlist, Note};
use crate::error::Error;

trait Visitor<'ast, 'input> {
    fn visit_folder_empty(&mut self, folder: &'ast Folder<'input>) -> Result<(), Error<'static>>;
    fn visit_folder_pre(&mut self, folder: &'ast Folder<'input>) -> Result<(), Error<'static>>;
    fn visit_folder_post(&mut self, folder: &'ast Folder<'input>) -> Result<(), Error<'static>>;
    fn visit_note(&mut self, note: &'ast Note<'input>) -> Result<(), Error<'static>>;
    fn visit_root_pre(&mut self, hotlist: &'ast Hotlist<'input>) -> Result<(), Error<'static>>;
    fn visit_root_post(&mut self, hotlist: &'ast Hotlist<'input>) -> Result<(), Error<'static>>;
}

fn traverse_hotlist<'ast, 'input, V: Visitor<'ast, 'input>>(
    hl: &'ast Hotlist<'input>,
    visitor: &mut V,
) -> Result<(), Error<'static>> {
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

fn nodes_equal<'input>(a: Option<&EntryKind<'input>>, b: Option<&EntryKind<'input>>) -> bool {
    if a.is_none() || b.is_none() {
        return false;
    } else {
        let a_ref = a.unwrap();
        let b_ref = b.unwrap();

        return std::ptr::eq(a_ref, b_ref);
    }
}
