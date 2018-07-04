use std::collections::HashMap;
use regex::{Regex, RegexSet};

use isymtope_ast_common::*;
use super::*;

lazy_static! {
    pub static ref URL_TOKEN: Regex = Regex::new(r"/:(?P<tok>[^/]+)").unwrap();
}

pub enum RouteMatch {
    Route(Route),
    RouteWithParams(Route, HashMap<String, ExpressionValue<ProcessedExpression>>)
}

impl RouteMatch {
    pub fn route<'r>(&'r self) ->&'r Route {
        match self {
            RouteMatch::Route(ref r) => r,
            RouteMatch::RouteWithParams(ref r, _) => r
        }
    }
}

#[derive(Debug, Default)]
pub struct RouteMatcher {}

impl RouteMatcher {
    pub fn get_matching_route(&self, doc: &Document, path: &str) -> Option<RouteMatch> {
        let routes: Vec<_> = doc.routes().map(|r| r.to_owned()).collect();
        let regexes: Vec<_> = routes.iter().map(|r| URL_TOKEN.replace_all(r.pattern(), r"/(?P<$tok>[a-zA-Z0-9]+)")).collect();
        let matcher = RegexSet::new(&regexes).unwrap();
        let regexes: Vec<_> = regexes.into_iter().map(|s| Regex::new(&s).unwrap()).collect();
        let idx = matcher.matches(path).into_iter().nth(0);
        if let Some(idx) = idx {
            let route = routes[idx].to_owned();
            let regex = &regexes[idx];
            if let Some(params) = self.path_to_params(regex, &path) {
                return Some(RouteMatch::RouteWithParams(route, params));
            }
            return Some(RouteMatch::Route(route));
        }
        None
    }

    fn path_to_params(&self, regex: &Regex, path: &str) -> Option<HashMap<String, ExpressionValue<ProcessedExpression>>> {
        if let Some(captures) = regex.captures(path) {
            let mut params = HashMap::new();
            for name in regex.capture_names() {
                if let Some(name) = name {
                    if let Some(value) = captures.name(name) {
                        let value = ExpressionValue::Primitive(Primitive::StringVal(value.as_str().to_owned()));
                        params.insert(name.to_owned(), value);
                    }
                }
            }
            return Some(params);
        }

        None
    }

    fn use_route_params(
        &self,
        ctx: &mut OutputContext,
        route: &RouteMatch,
    ) -> IsymtopeGenerateResult<()> {
        if let RouteMatch::RouteWithParams(_, params) = route {
            for (key, value) in params {
                // TODO: replace with new binding type
                let binding =
                    CommonBindings::NamedReducerActionParam(key.to_owned(), Default::default());
                eprintln!(
                    "[server/executor] adding binding [{:?}] for key {} with value [{:?}]",
                    binding, key, value
                );
                ctx.bind_value(binding, value.to_owned())?;
            }
        };

        // Ok(route)
        Ok(())
    }

    pub fn with_route(
        &self,
        doc: &Document,
        ctx: &mut OutputContext,
        path: &str,
    ) -> IsymtopeGenerateResult<RouteMatch> {
        let matcher = RouteMatcher::default();
        let route = matcher.get_matching_route(doc, path);
        if route.is_none() {
            return Err(try_eval_from_err!("Invalid route"))?;
        }
        let route = route.unwrap();

        matcher.use_route_params(
            ctx,
            &route
        )?;

        Ok(route)
    }
}
