(function(_global) {
    _global.Isymtope = _global.Isymtope || {}
    let IsymtopeRouting = _global.Isymtope.Routing = {}

    function createHistory(_window) {
        return {
            init: function(store, dispatchCurrentRoute) {
                dispatchCurrentRoute = !!dispatchCurrentRoute

                _window.addEventListener('popstate', function(event) {
                    store.dispatch({type: '@@redux-routing/replace', href: event.state});
                });

                if (dispatchCurrentRoute) {
                    store.dispatch({type: '@@redux-routing/replace', href: _window.location.pathname });
                }
            },
            update: function(action) {
                switch (action.type) {
                    case '@@redux-routing/navigate':
                        _window.history.pushState(action.href, null, action.href); break;
                    case '@@redux-routing/replace':
                        _window.history.replaceState(action.href, null, action.href); break;
                    default: break;
                }
            }
        };
    }

    function createRoutingMiddleware(routes, history) {
        var routes = Object.keys(routes).map(function(pattern) { return { regex: new RegExp('^' + pattern.replace(/\//g, '\/') + '$'), handler: routes[pattern].handler }; });

        return function(store) {
            return function(next) {
                return function(action) {
                    if (!/^@@redux-routing/.test(action.type)) {
                        return next(action);
                    }

                    var href = action.href;
                    var match = routes.filter(function(route) { return !!route.regex.exec(href); })[0];

                    if (match && match.handler) {
                        match.handler(href, store);
                    }

                    history.update(action);
                }
            }
        };
    }

    let navigate = href => ({ type: '@@redux-routing/navigate', href: href })

    function startRouting(history, store, dispatchCurrentRoute) {
            dispatchCurrentRoute = !!dispatchCurrentRoute

            // Start routing
            history.init(store, dispatchCurrentRoute);
    
            _global._go = function(href) { store.dispatch(navigate(href)); };
    }
    
    IsymtopeRouting.createHistory = createHistory
    IsymtopeRouting.createRoutingMiddleware = createRoutingMiddleware
    IsymtopeRouting.startRouting = startRouting
}(window));