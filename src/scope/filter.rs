// #![allow(dead_code)]

use parser::*;
use scope::*;


pub struct FilterPipelineReduceIter<'ctx, 'head, 'a, S: Iterator<Item = &'a FilterPipelineComponent>> {
    ctx: &'ctx mut Context,
    head: Option<&'head ExprValue>,
    last_state: FilterPipelineReduceIterState,
    buf: Option<Vec<String>>,
    iter: S
}

#[derive(Debug, Clone, PartialEq)]
enum FilterPipelineReduceIterState {
    NoState,
    Symbol,
    PipelineOp
}

impl<'ctx, 'head, 'a, S: Iterator<Item = &'a FilterPipelineComponent>> FilterPipelineReduceIter<'ctx, 'head, 'a, S>
{
    #[allow(dead_code)]
    pub fn new(ctx: &'ctx mut Context, head: Option<&'head ExprValue>, iter: S) -> Self {
        FilterPipelineReduceIter {
            ctx: ctx,
            head: head,
            last_state: FilterPipelineReduceIterState::NoState,
            buf: None,
            iter: iter
        }
    }

    #[allow(dead_code)]
    fn push_s(&mut self, s: &str, clear: bool) {
        if clear { self.buf = None; }

        if let Some(ref mut buf) = self.buf {
            buf.push(s.to_owned());
        } else {
            self.buf = Some(vec![s.to_owned()]);
        };
    }
}

impl<'ctx, 'head, 'a, S: Iterator<Item = &'a FilterPipelineComponent>> Iterator for FilterPipelineReduceIter<'ctx, 'head, 'a, S>
{
    type Item = Option<ReducedPipelineComponent>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next();

        if let Some(next) = next {
            let res: Self::Item;

            self.ctx.push_child_scope();
            let op = match next {
                &FilterPipelineComponent::Set(ref props, ref where_clause) => {
                    // Map
                    self.ctx.add_sym("item", Symbol::binding(&BindingType::MapItemBinding));
                    self.ctx.add_sym("index", Symbol::binding(&BindingType::MapIndexBinding));

                    // let props: Option<Vec<_>> = props.as_ref().map(|v| v.iter().map(|p| (p.0.to_owned(), p.1.as_ref().map(|s| s.to_owned()))).collect());
                    let props: Option<Vec<_>> = props.as_ref().map(|props|
                        props.iter().map(|&(ref key, ref expr)| (key.to_owned(), expr.as_ref().map(|e| self.ctx.reduce_expr_or_return_same(e)))).collect()
                    );

                    let src_object = ExprValue::Binding(BindingType::MapItemBinding);
                    let set_object = ExprValue::LiteralObject(props);
                    let set_expr = ExprValue::Expr(ExprOp::Add, src_object.into(), set_object.into());

                    let where_cond = where_clause.as_ref().map(|where_clause| where_clause.as_expr());

                    if let Some(ref cond) = where_cond {
                        let cond = self.ctx.reduce_expr_or_return_same(cond);
                        Some(ReducedMethodType::MapIf(set_expr, cond))
                    } else {
                        Some(ReducedMethodType::Map(set_expr))
                    }
                }

                &FilterPipelineComponent::Unique(ref sym) => {
                    // self.ctx.add_sym("item", Symbol::binding(&BindingType::MapItemBinding));
                    // self.ctx.add_sym("index", Symbol::binding(&BindingType::MapIndexBinding));

                    let item = Symbol::binding(&BindingType::MapItemBinding);

                    match sym.sym_ref() {
                        &SymbolReferenceType::UnresolvedReference(ref key) => {
                            let member_path = Symbol::member_path(item, key);
                            Some(ReducedMethodType::UniqMember(member_path))
                        }

                        _ => None
                    }
                }

                _ => None
            };
            self.ctx.pop_scope();

            if let Some(op) = op {
                res = Some(ReducedPipelineComponent::PipelineOp(op));
            } else {
                res = None;
            };

            return Some(res);
        };

        None
    }
}
