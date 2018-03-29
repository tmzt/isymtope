
const baseUrl = !!document.baseURI ? new URL(document.baseURI).pathname.replace(/\/+$/, '') : ''
const rootPath = baseUrl.length ? baseUrl + '/' : ''
const mapRoute = href => rootPath + href.replace(/^\/+/, '').replace(/\/+$/, '')
const navigate = href => ({ type: '@@redux-routing/navigate', href: mapRoute(href) })

function buildRoutes(routes) {
    return  Object.keys(routes).map(pattern =>({ regex: new RegExp('^' + mapRoute(pattern.replace(/\//g, '\/') + '$')), handler: routes[pattern].handler }))
}

function createHistory(_window, store) {
    _window.addEventListener('popstate', function(event) {
        store.dispatch({type: '@@redux-routing/replace', href: event.state})
    })

    return {
        update(action) {
            switch (action.type) {
                case '@@redux-routing/navigate':
                    _window.history.pushState(action.href, null, action.href); break
                case '@@redux-routing/replace':
                    _window.history.replaceState(action.href, null, action.href); break
                default: break
            }
        }
    }
}

function createMiddleware(router) {
    return function(store) {
        const history = createHistory(window, store)

        return function(next) {
            return function(action) {
                if (!/^@@redux-routing/.test(action.type)) {
                    return next(action)
                }

                router.handle(action, store)
                history.update(action)
            }
        }
    }
}

class RouterRuntime {
    constructor() {
        this._routes = {}
    }

    set routes(routes) {
        this._routes = buildRoutes(routes)
    }

    handle(action, store) {
        var href = action.href;
        var match = this._routes.filter(function(route) { return !!route.regex.exec(href); })[0];

        if (match && match.handler) {
            match.handler(href, store);
        }
    }
}

class IsymtopeAppRouter {
    constructor() {
        this._routes = {}
        this._router = null
        this._middleware = null
    }

    get router() {
        if (!this._router) {
            this._router = new RouterRuntime()
        }
        return this._router
    }

    get middleware() {
        if (!this._middleware) {
            this._middleware = createMiddleware(this.router)
        }
        return this._middleware
    }

    set routes(routes) {
        this.router.routes = routes
    }

    static get navigate() {
        return navigate
    }
}

exports.IsymtopeAppRouter = IsymtopeAppRouter
