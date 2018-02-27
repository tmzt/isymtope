
importScripts('https://www.hellorust.com/demos/bundle.js')

let moduleLoaded = false

self.onmessage = async ({data, ports}) => {
    switch(data.topic) {        
        case '/compilerWorker/startCompilation':
            console.info('[compiler worker] received startCompilation, will send message to resourceWorker')
            let { source, pathname, mimeType, app_name, base_url, template_path, path } = data
            let resourceWorkerPort = ports[0]

            await setupCompiler()
            let content = await compileTemplate(source, app_name, base_url, template_path, path)
            resourceWorkerPort.postMessage({ topic: '/resourceWorker/compilationComplete', content, pathname, mimeType })
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

const memory = new WebAssembly.Memory({initial: 16000})
const imports = {
    env: {
        memory: memory, rand_value: randomValue, log_to_js: logToJs,
        fmod: function(x, y) { return x % y; },
    }
}

async function compileTemplate(src, app_name, base_url, template_path, path) {
    await setupCompiler()
    console.log('[compiler worker] allocating source buffer')
    let buf = newString(Module, src)

    let app_name_str = newString(Module, app_name)
    let base_url_str = newString(Module, base_url)
    let template_path_str = newString(Module, template_path)
    let path_str = newString(Module, path)

    console.log('[compiler worker] compiling template')
    let outptr = Module.compile_template(buf, app_name_str, base_url_str, template_path_str, path_str)
    console.log('[compiler worker] converting output to string')
    let result = copyCStr(Module, outptr)
    return result
}

async function setupCompiler() {
    if (!moduleLoaded) {
        self.Module = {}
        let mod = await fetchAndInstantiate('/app/playground/isymtope-small.wasm', imports)
        console.log('[compiler worker] loaded module')
        Module.memory = memory
        Module.compile_template = mod.exports.compile_template
        Module.alloc   = mod.exports.alloc
        Module.dealloc = mod.exports.dealloc
        Module.dealloc_str = mod.exports.dealloc_str
        moduleLoaded = true
    }
}

async function main() {
    console.log('[compiler worker] initializing compiler')
    await setupCompiler()
}

main().then(() => console.log('[compiler worker] setup complete'))
