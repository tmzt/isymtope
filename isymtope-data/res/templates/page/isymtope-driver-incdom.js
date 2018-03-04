(function(_global) {
    let Isymtope = _global.Isymtope = _global.Isymtope || {}

    function IsymtopeDriverIncDom(root, render, store) {
        this._root = root;
        this._render = render.bind(null, store);
    }

    ///
    /// For compatibility with IncDom's empty prototype objects.
    ///

    function Blank() {}
    Blank.prototype = Object.create(null);
    
    // let listeners = new Map();

    function markExisting(nodes) {
        nodes = 'string' === typeof nodes ? document.querySelectorAll(nodes) : nodes
        Array.prototype.slice.call(nodes).forEach(function(node) {
            IncrementalDOM.importNode(node);
            var data = node['__incrementalDOMData'];
            data.staticsApplied = true;
            data.newAttrs = new Blank();
        });
    }

    function classList() {
        return [].slice.call(arguments).filter(function(e) { return !!e; }).join(' ');
    }

    IsymtopeDriverIncDom.prototype.prepareExisting = function IsymtopeDriverIncDom_prepareExisting(nodes) {
        markExisting(nodes);
    }

    IsymtopeDriverIncDom.prototype.update = function IsymtopeDriverIncDom_update() {
        IncrementalDOM.patch(this._root, this._render);
    }

    // IsymtopeDriverIncDom.createDriverFactory = function IsymtopeDriverIncDom_createDriverFactory(opts, renderFunc) { 
    //     return function(store) {
    //         let render = renderFunc.bind(null, store);
    //         return new IsymtopeDriverIncDom(opts, render);
    //     }
    // }

    function createIsymtopeRouter(opts) {
        let history = opts.history = IsymtopeRouting.createHistory(window)
        let routes = opts.routes || {};
        let store = opts.store;

        return Isymtope.Routing.createRoutingMiddleware(routes, history);
    };
    
    function createStore(rootReducer, middleware) {
        return Redux.applyMiddleware.apply(null, middleware)(Redux.createStore)(rootReducer);
    };

    Isymtope.initialize = function IsymtopeDriverIncDom_initialize(root, render, rootReducer, routes, extraMiddleware, dispatchCurrentRoute) {
        dispatchCurrentRoute = !!dispatchCurrentRoute

        let history = Isymtope.Routing.createHistory(window)
        let routingMiddleware = Isymtope.Routing.createRoutingMiddleware(routes, history)
        let middleware = extraMiddleware ? extraMiddleware.concat(routingMiddleware) : [routingMiddleware]

        let store = createStore(rootReducer, middleware)
        let driver = new IsymtopeDriverIncDom(root, render, store)

        document.addEventListener("DOMContentLoaded", function(event) {
            // Mark nodes as existing
            driver.prepareExisting('*');

            // Subscribe
            let update = driver.update.bind(driver);
            store.subscribe(update);

            Isymtope.Routing.startRouting(history, store, dispatchCurrentRoute)
        });

        _global.Isymtope.store = store

        return store
    }

    Isymtope.IncDOM = IsymtopeDriverIncDom
}(window));