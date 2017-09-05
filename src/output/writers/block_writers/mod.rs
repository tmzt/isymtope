use std::io;

use processing::*;
use scope::*;
use output::*;


pub trait BlockWriter {
    fn write_block(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, block: &Block, enclosing_tag: Option<&str>, with_events: bool) -> Result;
}

impl<O: OutputWriter + ElementOpsWriter + ElementOpsStreamWriter + EventsWriter> BlockWriter for O {
    fn write_block(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, block: &Block, enclosing_tag: Option<&str>, _with_events: bool) -> Result {
        // let complete_key = ctx.join_path_with(Some("."), block.id());

        // Render opening enclosing tag
        if let Some(enclosing_tag) = enclosing_tag {
            self.write_op_element_open(w, ctx, bindings, enclosing_tag, block.id(), false, None, None, None)?;
        };

        // Push block scope
        // ctx.push_child_scope();
        // ctx.append_path_str(block.id());

        // Render ops
        if let Some(ops_iter) = block.ops_iter() {
            self.write_element_ops(w, doc, ctx, bindings, ops_iter)?;
        };

        // // Bind events
        // if with_events {
        //     if let Some(events_iter) = block.events_iter() {
        //         self.write_event_bindings(w, ctx, bindings, events_iter)?;
        //     };
        // };

        // Render closing enclosing tag
        if let Some(enclosing_tag) = enclosing_tag {
            self.write_op_element_close(w, ctx, bindings, enclosing_tag)?;
        };
        // ctx.pop_scope();

        Ok(())
    }
}