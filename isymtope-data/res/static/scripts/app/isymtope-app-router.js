
const baseUrl = !!document.baseURI ? new URL(document.baseURI).pathname.replace(/\/+$/, '') : ''
const rootPath = baseUrl.length ? baseUrl + '/' : ''
const mapRoute = href => rootPath + href.replace(/^\/+/, '').replace(/\/+$/, '')
const navigate = pathname => ({ type: '@@redux-routing/navigate', state: { href: mapRoute(pathname) }, href: mapRoute(pathname), pathname: pathname })

const SEGMENTS = /\/([^/]+)/g

function buildRoutes(routes) {
    return  Object.keys(routes).map(pattern =>{
        let names = []
        const regex = mapRoute(pattern).replace(SEGMENTS, (m, p) => {
            if (/^:/.test(p)) {
                names.push(p.slice(1))
                return '\/([^/]+)'
            }
            return '\/' + p
        })
        const route = routes[pattern]
        return {
            regex: new RegExp('^\/' + regex + '$'),
            names: names,
            handler: route.handler,
            content: route.content
        }
    })
}

function createHistory(_window, store) {
    _window.addEventListener('popstate', function(event) {
        store.dispatch({type: '@@redux-routing/replace', state: event.state})
    })

    return {
        update(action) {
            switch (action.type) {
                case '@@redux-routing/navigate':
                    _window.history.pushState(action.state, null, action.href); break
                case '@@redux-routing/replace':
                    _window.history.replaceState(action.state, null, action.href); break
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
                if (!action.hasOwnProperty('href')) { action.href = location.href }
                if (!action.hasOwnProperty('pathname')) { action.pathname = location.pathname }

                const route = router.matchRoute(action)
                // Render content first
                if (route && route.content) {
                    route.content(store)
                }

                if (route && route.handle) {
                    route.handle(store)
                }
                history.update(action)

                return { state: action.state || {}, href: action.href || location.pathname }
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

    matchRoute(action) {
        const pathname = action.pathname || action.href
        for (let route of this._routes) {
            let m = route.regex.exec(pathname)
            if (m && m.length) {
                let matches = m.slice(1)
                let params = {}
                matches.forEach((v, i) => { let name = route.names[i]; params[name] = v })
                return {
                    handle: (store) => route.handler(pathname, store, params),
                    content: (store) => route.content(pathname, store, params)
                }
                // return route.handler(pathname, store, params)
            }
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

    addRoute(pattern, handler) {
        this.router.routes[pattern] = { handler: handler }
    }

    static get navigate() {
        return navigate
    }

    renderContent(store) {
        this.router.renderContent(store)
    }
}

exports.IsymtopeAppRouter = IsymtopeAppRouter
