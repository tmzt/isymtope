(function(_global) {
    let util = _global.IsymtopeUtil = {};

    let toEntry = (obj) => [obj.id, obj]
    let orTrue = (func) => 'function' === typeof func ? func : _item => true
    let orSelf = (item, func, cond) => cond(item) ? func(item) : item
    let orMember = (func) => 'function' === typeof func ? func : _item => _item[func];

    class IsymtopeUtilWrapper {
        constructor(m) {
            this.m = m
        }

        map(func, cond = undefined) {
            let arr = Array.from(this.m.entries());
            func = orMember(func)
            cond = orMember(orTrue(cond))
            let entries = arr.map(([_idx, _item]) => toEntry(orSelf(_item, func, cond)));
            this.m = new Map(entries);

            return this;
        }

        enumerateWithKeys() {
            let arr = Array.from(this.m.entries());
            let entries = arr.map(([_key, _item], _idx) => [_idx, [_item, _key]]);
            this.m = new Map(entries);

            return this;
        }

        reduce(func, initial = undefined) {
            func = orMember(func)
            cond = orMember(orTrue(cond))
            let arr = Array.from(this.m.entries());
            let entries = arr.reduce((acc, item) => orSelf(item, func, cond), initial);

            this.m = new Map(entries)
            return this
            // return this.reduceIf(func, initial);
        }

        reduceIf(func, initial = undefined, cond = undefined) {
            func = orMember(func)
            cond = orMember(orTrue(cond))
            let arr = Array.from(this.m.entries());
            let entries = arr.reduce((acc, item) => orSelf(item, func, cond), initial);

            this.m = new Map(entries)
            return this
        }

        min(func) {
            func = orMember(func)
            let arr = Array.from(this.m.entries());

            let res = arr.reduce((acc, [_idx, _item]) => {
                let v = func(_item)
                return (v < acc) ? v : acc
            }, 0);

            this.m = res
            return this
        }

        max(func) {
            func = orMember(func)
            let arr = Array.from(this.m.entries());

            let res = arr.reduce((acc, [_idx, _item]) => {
                let v = func(_item)
                return (v > acc) ? v : acc
            }, 0);

            this.m = res
            return this
        }

        count(func = undefined) {
            func = orTrue(orMember(func))
            let arr = Array.from(this.m.entries());

            let res = arr.reduce((acc, [_idx, _item]) => {
                let v = func(_item)
                return (!!v) ? (acc + 1) : acc
            }, 0)

            this.m = res
            return this
        }

        // Pipeline utilities

        addObject(values) {
            let arr = Array.from(this.m.entries())
            let entry = toEntry(values)
            let entries = arr.concat([entry]);

            this.m = new Map(entries);
            return this;
        }

        setObject(values, cond = undefined) {
            let setValues = (obj, values) => Object.assign({}, obj, values)

            return this.map(setValues, cond)
        }

        whereObject(func) {
            func = orMember(func)

            let arr = Array.from(this.m.entries())
            let entries = arr.filter(([_idx, _item]) => func(_item, _idx))

            this.m = new Map(entries)
            return this

        }

        filter(func) { return this.whereObject(func); }

        removeObject() {

        }

        unique(func = undefined) {
            func = 'undefined' === typeof func ? 'id' : func
            func = orMember(func)

            let keys = new Set();
            let arr = Array.from(this.m.entries())

            let entries = arr.filter(([_idx, _item]) => {
                let id = func(_item)
                if (keys.has(id)) { return false }
                keys.add(id)

                return true
            })

            this.m = new Map(entries)
            return this
        }

        get value() {
            return this.m;
        }
    }

    _global.__IsymtopeUtil = m => new IsymtopeUtilWrapper(m)

    let classes = obj => Object.entries(obj).map(([_key, _value]) => !!_value ? _key : undefined).filter(Boolean).join(' ')

    _global.__IsymtopeUtil__classes = classes

}(window));