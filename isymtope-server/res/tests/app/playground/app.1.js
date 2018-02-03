
// function fetchAndInstantiate(url, importObject) {
//   return fetch(url).then(response => response.arrayBuffer())
//     .then(bytes => WebAssembly.instantiate(bytes, importObject))
//     .then(results => results.instance );
// }

function randomValue(n) {
    return Math.floor(Math.random() * n)
}

function logToJs(ptr) {
    let result = copyCStr(Module, ptr)
    console.log('[rust] ' + result)
}

const memory = new WebAssembly.Memory({initial: 6000})
const imports = {
    env: {
        memory: memory, rand_value: randomValue, log_to_js: logToJs,
        fmod: function(x, y) { return x % y; },
    }
}

let cache = new Map

async function loadFiles(files) {
    for(let file of files) {
        await window.fetch(file.path)
            .then(async resp => {
                let body = await resp.text()
                cache.set(file.path, body)
                return body
            })
    }
}

function fetchContent(path) {
    let editor = window._editor
    if (editor) {
        window.fetch('/' + path).then(resp => {
            resp.text().then(body => {
                editor.setValue(body)
            })
        })
    }
}

function compileTemplate(src) {
    let buf = newString(Module, src);
    // let outptr = Module.compile_template_source_to_html(buf)
    let outptr = Module.compile_template(buf)
    let result = copyCStr(Module, outptr)
    return result
}

function switchEditor(file) {
    let contents = cache.get(file.path)
    editor.setValue(contents)
}

function loadFileReducer(state, action, store) {
    switch(action.type) {
        default: return null
    }
}

function loadWorkspaceReducer(state, action) {
    switch(action.type) {
        case 'LOADWORKSPACE.LOADWORKSPACE':
            let workspaces = new Map(action.workspaces.map(w => [w.id, w]))
            let workspace = workspaces.get(action.id)
            let files = workspace.files
            let mainFile = files.filter(f => !!f.main)[0]
            loadFiles(files)
                .then(() => switchEditor(mainFile))
            return true
        default: return null
    }
}

function editorContentReducer(state, action) {
    switch(action.type) {
        case 'EDITORCONTENT.LOAD':
            fetchContent(action.name); return true;
        default: return null
    }
}

function compilerReducer(state, action) {
    switch(action.type) {
        case 'COMPILER.COMPILE':
            let source = editor.getValue();
            let output = compileTemplate(source);
            let uri = URL.createObjectURL(new Blob([output], {type: 'text/html'}));
            let iframe = document.querySelector('iframe#preview')
            iframe.src = uri
            return true;
        default: return null
    }
}

async function setupCompiler() {
    window.Module = {}
    let mod = await fetchAndInstantiate('/app/playground/isymtope-small.wasm', imports)
    Module.memory = memory
    Module.compile_template = mod.exports.compile_template
    Module.alloc   = mod.exports.alloc
    Module.dealloc = mod.exports.dealloc
    Module.dealloc_str = mod.exports.dealloc_str
    // Module.memory  = new Uint8Array(mod.exports.memory.buffer)
}

let compiler

function setupCompilerWorker() {
    compiler = new Worker('/app/playground/worker.js')
}

function setupEditor() {
    let editorComponentDiv = document.getElementById('editorComponent')
    let editorDiv = document.createElement('div')
    editorDiv.setAttribute('id', 'editorDiv')
    editorComponentDiv.appendChild(editorDiv)

    return new Promise(resolve => {
        require(["vs/editor/editor.main"], function () {
            editor = monaco.editor.create(editorDiv, {
                value: '',
                theme: 'vs-dark'
            });

            editor.addListener('didType', () => {
                console.log(editor.getValue());
            });

            window.fetch('/playground.ism').then(resp => {
                resp.text().then(body => {
                    editor.setValue(body)
                })
            })

            window._editor = editor

            resolve()
        });
    })
}

document.addEventListener('DOMContentLoaded', async () => {
    require.config({ paths: { 'vs': 'https://unpkg.com/monaco-editor@0.8.3/min/vs' }});
    window.MonacoEnvironment = { getWorkerUrl: () => proxy };

    let proxy = URL.createObjectURL(new Blob([`
        self.MonacoEnvironment = {
            baseUrl: 'https://unpkg.com/monaco-editor@0.8.3/min/'
        };
        importScripts('https://unpkg.com/monaco-editor@0.8.3/min/vs/base/worker/workerMain.js');
    `], { type: 'text/javascript' }));

    setupCompilerWorker()
    await setupEditor()
    // await setupCompiler()
})
