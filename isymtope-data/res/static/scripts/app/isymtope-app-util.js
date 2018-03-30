
const classes = obj => Object.entries(obj).map(([_key, _value]) => !!_value ? _key : undefined).filter(Boolean).join(' ')

function pipe(...fns){
    return xf => {
        for(let l=fns.length, i=0; i<l; i++ ) {
            xf = fns[i](xf)
        }
        return xf
    }
}

function pipeGen(...fns){
    return function* (xf) {
        for(let l=fns.length, i=0; i<l; i++ ) {
            xf = fns[i](xf)
        }
        yield xf
    }
}

function first(arr) {
    for(let value of arr) {
        return value
    }
    return undefined
}

function* take(n, arr) {
    let counter = n
    while(counter--) {
        yield arr.next()
    }
}

function* map(f, arr) {
    for(let value of arr) {
        yield f(value)
    }
}

function reduce (f, acc, gen) {
    let result = acc
    for(let value of gen) {
        result = f(result, value)
    }
    return result
}

function* toGen(arr) {
    for(let i=0; i<arr.length; i++) {
        yield arr[i]
    }
}

function* enumerate(arr) {
    let counter = 0
    for(const value of arr) {
        yield [counter++, value]
    }
}

function* filter(f, arr) {
    let counter = 0
    for(const value of arr) {
        if (f(value)) {
            yield value
        }
    }
}

function* uniq(arr) {
    let values = new Set()
    for(let value of arr) {
        if (!values.has(value)) {
            values.add(value)
            yield value
        }
    }
}

function setObject(values = {}, wc = () => true, arr) {
    const valueFunc = "function" === typeof values ? values : () => values
    return map(o => wc(o) ? Object.assign({}, valueFunc(o)) : o, arr)
}

function values(obj) {
    if (obj instanceof Map) {
        return obj.values()
    }

    if (obj instanceof Array) {
        return obj
    }

    if (obj instanceof Iterator) {
        return obj
    }

    return Array.of(obj)
}

const mapFunc = f => arr => map(f, arr)
const enumerateFunc = arr => enumerate(arr)
const takeFunc = n => arr => take(n, arr)
const filterFunc = f => arr => filter(f, arr)
const uniqFunc = arr => uniq(arr)
const min = arr => reduce(Math.min, 0, arr)
const max = arr => reduce(Math.max, 0, arr)
const count = arr => reduce(a => a + 1, 0, arr)
const countIf = (f, arr) => reduce((a, b) => a + (f(b) ? 1 : 0), 0, arr)
const countIfFunc = f => arr => countIf(f, arr)
const setObjectFunc = (values, wc = undefined) => arr => setObject(values, wc, arr)

const minBy = (f = o => o, arr) => reduce(Math.min, 0, map(f, arr))
const minByFunc = f => arr => minBy()
const maxBy = (f = o => o, arr) => reduce(Math.max, 0, map(f, arr))
const maxByFunc = f => arr => maxBy(f, arr)

Object.assign(exports, {
    classes,
    pipe,
    pipeGen,
    first,
    take,
    takeFunc,
    uniq,
    uniqFunc,
    map,
    mapFunc,
    reduce,
    toGen,
    enumerate,
    enumerateFunc,
    filter,
    filterFunc,
    min,
    minBy,
    minByFunc,
    max,
    maxBy,
    maxByFunc,
    count,
    countIf,
    countIfFunc,
    setObject,
    setObjectFunc,
    values
})
