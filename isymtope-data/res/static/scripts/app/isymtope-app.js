
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

        this._driver = driver
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

    addRoute(pattern, handler) {
        this._router.addRoute(pattern, handler)
        return this
    }

    configure(createRenderer, routes, createRootReducer, createEvents, extraMiddleware = []) {
        const rootDiv = document.querySelector('body')
        const generationId = rootDiv.getAttribute('key')

        if (this._generationId == generationId) {
            return
        }

        this._router.routes = routes
        this._createRenderer = createRenderer
        this._createRootReducer = createRootReducer
        this._createEvents = createEvents
        this._extraMiddleware = extraMiddleware
        this._eventKeys = null
    }

    get store() {
        if (this._store == null) {
            this._store = createStoreRuntime.apply(this)
            const render = this._createRenderer(this._store)
            this._store.subscribe(() => this._driver.update(() => render(this._store), this._store))
        }
        return this._store
    }

    async update() {
        const rootDiv = document.querySelector('body')
        const generationId = rootDiv.getAttribute('data-generation') || rootDiv.getAttribute('key')

        if (this._generationId == generationId) {
            return
        }
        this._lastGeneration = this._generationId
        this._generationId = generationId

        this._driver.bindRootDiv(rootDiv)

        const store = this.store
        const events = this._createEvents(store)
        const oldKeys = Object.keys(window._events)
        oldKeys.forEach(k => { delete window._events[k] })
        Object.assign(window._events, events)
    }

    async run(dispatchDefault = false) {
        await this.update()

        const store = this.store
        const hooks = this._opts.beforeRoutingHooks || []
        await Promise.all(hooks.map(fn => fn(this.store)))

        const navigate = IsymtopeAppRouter.navigate
        const optDispatch = this._opts.alwaysNavigateToDefaultRoute && this._opts.defaultRoute
        if (dispatchDefault || optDispatch) {
            store.dispatch(navigate(this._opts.defaultRoute))
        }
    }

}

let _app = null
let _driverFactory = null

class Isymtope {
    static setDriverFactory(driverFactory) {
        _driverFactory = driverFactory
    }

    static app() {
        if (_app == null) {
            const driver = _driverFactory()
            _app = new IsymtopeAppPrivate(driver)
        }

        return _app
    }

    static get navigate() {
        return IsymtopeAppRouter.navigate
    }
}

exports.Isymtope = Isymtope
