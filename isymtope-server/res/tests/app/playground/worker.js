
importScripts('https://www.hellorust.com/demos/bundle.js')

self.onmessage = ({data}) => {
    switch(data.topic) {
        case '/compiler/updatePreview':
            let output = compileTemplate(data.source)
            postMessage({topic: '/main/updatePreview', output }, '*')
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

const memory = new WebAssembly.Memory({initial: 10000})
const imports = {
    env: {
        memory: memory, rand_value: randomValue, log_to_js: logToJs,
        fmod: function(x, y) { return x % y; },
    }
}

function compileTemplate(src) {
    let buf = newString(Module, src);
    let outptr = Module.compile_template(buf)
    let result = copyCStr(Module, outptr)
    return result
}

async function setupCompiler() {
    self.Module = {}
    let mod = await fetchAndInstantiate('/app/playground/isymtope-small.wasm', imports)
    Module.memory = memory
    Module.compile_template = mod.exports.compile_template
    Module.alloc   = mod.exports.alloc
    Module.dealloc = mod.exports.dealloc
    Module.dealloc_str = mod.exports.dealloc_str
}

async function main() {
    await setupCompiler()
}

main().then(() => console.log('[compiler worker] setup complete'))
