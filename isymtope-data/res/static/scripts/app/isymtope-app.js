
const IsymtopeAppRouter = exports.IsymtopeAppRouter

function createDefaultOpts() {
    return {
        beforeRoutingHooks: [],
        defaultRoute: '/',
        alwaysNavigateToDefaultRoute: false
    }
}

function createStore(rootReducer, middleware) {
    return Redux.applyMiddleware.apply(null, middleware)(Redux.createStore)(rootReducer);
}

function createStoreRuntime() {
    const routingMiddleware = this._router.middleware
    const extraMiddleware = this._extraMiddleware
    const middleware = [routingMiddleware].concat(extraMiddleware)
    const rootReducer = this._createRootReducer()

    return createStore(rootReducer, middleware)
}

class IsymtopeAppPrivate {
    constructor(driver) {
        this._generationId = null

        this._driver = new driver()
        this._router = new IsymtopeAppRouter()
        this._store = null
        this._opts = createDefaultOpts()
    }

    registerBeforeRoutingHook(fn) {
        this._opts.beforeRoutingHooks.push(fn)
        return this
    }

    setDefaultRoute(defaultRoute) {
        this._opts.defaultRoute = defaultRoute
        return this
    }

    alwaysNavigateToDefaultRoute(val = true) {
        this._opts.alwaysNavigateToDefaultRoute = val
        return this
    }

    configure(generationId, rootDiv, render, routes, createRootReducer, createEvents, extraMiddleware = []) {
        const lastGeneration = this._generationId
        if (lastGeneration == generationId) {
            return
        }
        this._lastGeneration = lastGeneration
        this._generationId = generationId

        this._driver.bindRootDiv(rootDiv)
        this._router.routes = routes
        this._createRootReducer = createRootReducer
        this._createEvents = createEvents
        this._extraMiddleware = extraMiddleware
        this._eventKeys = null
    }

    get store() {
        if (this._store == null) {
            this._store = createStoreRuntime.apply(this)
            this._store.subscribe(() => this._driver.update(() => render(this._store), this._store))
        }
        return this._store
    }

    async run(dispatchDefault = false) {
        const store = this.store

        if (this._lastGeneration !== this._generationId) {
            const events = this._createEvents(store)
            this._eventKeys = Object.keys(events)
            if (this._eventKeys) {
                this._eventKeys.forEach(k => window[k] = undefined)
            }
            Object.assign(window, events)
            Object.assign(window, { _events: events })
        }

        const hooks = this._opts.beforeRoutingHooks || []
        await Promise.all(hooks.map(fn => fn(this._store)))

        const navigate = this._router.navigate
        const optDispatch = this._opts.alwaysNavigateToDefaultRoute && this._opts.defaultRoute
        if (dispatchDefault || optDispatch) {
            store.dispatch(navigate(this._opts.defaultRoute))
        }
    }

}

let _app = null
let _driver = null

class Isymtope {
    static setDriver(driver) {
        _driver = driver
    }

    static app() {
        if (_app == null) {
            _app = new IsymtopeAppPrivate(_driver)
        }

        return _app
    }

    static get navigate() {
        return IsymtopeAppRouter.navigate
    }
}

exports.Isymtope = Isymtope
