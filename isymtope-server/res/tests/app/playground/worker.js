
importScripts('https://www.hellorust.com/demos/bundle.js')

self.onmessage = ({data}) => {
    switch(data.topic) {
        case '/compiler/updatePreview':
            let output = compileTemplate(data.source)
            postMessage({ topic: '/main/updatePreview', output })
            break;
    }
}

function randomValue(n) {
    return Math.floor(Math.random() * n)
}

function logToJs(ptr) {
    let result = copyCStr(Module, ptr)
    console.log('[rust] ' + result)
}

const memory = new WebAssembly.Memory({initial: 20000})
const imports = {
    env: {
        memory: memory, rand_value: randomValue, log_to_js: logToJs,
        fmod: function(x, y) { return x % y; },
    }
}

function compileTemplate(src) {
    console.log('[compiler worker] allocating source buffer')
    let buf = newString(Module, src);
    console.log('[compiler worker] compiling template')
    let outptr = Module.compile_template(buf)
    console.log('[compiler worker] converting output to string')
    let result = copyCStr(Module, outptr)
    return result
}

async function setupCompiler() {
    self.Module = {}
    let mod = await fetchAndInstantiate('/app/playground/isymtope-small.wasm', imports)
    console.log('[compiler worker] loaded module')
    Module.memory = memory
    Module.compile_template = mod.exports.compile_template
    Module.alloc   = mod.exports.alloc
    Module.dealloc = mod.exports.dealloc
    Module.dealloc_str = mod.exports.dealloc_str
}

async function main() {
    console.log('[compiler worker] initializing compiler')
    await setupCompiler()
}

main().then(() => console.log('[compiler worker] setup complete'))