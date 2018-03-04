(function(_global) {
    let Isymtope = _global.Isymtope = _global.Isymtope || {}

    function IsymtopeApp() {
        this._beforeRoutingHooks = []
        this._defaultRoute = '/'
        this._alwaysNavigateToDefaultRoute = false
    }

    IsymtopeApp.prototype.registerBeforeRoutingHook = function Isymtope_registerBeforeRoutingHook(hookFn) {
        this._beforeRoutingHooks.push(hookFn)
    }

    IsymtopeApp.prototype.setDefaultRoute = function Isymtope_setDefaultRoute(route) {
        this._defaultRoute = route
    }

    IsymtopeApp.prototype.alwaysNavigateToDefaultRoute= function Isymtope_alwaysNavigateToDefaultRoute() {
        this._alwaysNavigateToDefaultRoute = true
    }

    IsymtopeApp.prototype.beforeRouting = function Isymtope_beforeRouting(store) {
        if (this._beforeRoutingHooks.length) {
            this._beforeRoutingHooks.forEach(hook => {
                hook(store)
            })
        }
    }

    IsymtopeApp.prototype.routingStarted = function Isymtope_routingStarted(store, navigate) {
        if (this._alwaysNavigateToDefaultRoute && this._defaultRoute) {
            store.dispatch(navigate(this._defaultRoute))
        }
    }

    let _app
    Isymtope.app = function() {
        if (!_app) {
            _app = new IsymtopeApp()
        }

        return _app
    }
}(window));