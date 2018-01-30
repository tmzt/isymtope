
// function fetchAndInstantiate(url, importObject) {
//   return fetch(url).then(response => response.arrayBuffer())
//     .then(bytes => WebAssembly.instantiate(bytes, importObject))
//     .then(results => results.instance );
// }

function randomValue(n) {
    return Math.floor(Math.random() * n)
}

const memory = new WebAssembly.Memory({initial: 2000})
const imports = {
    env: { memory: memory, rand_value: randomValue }
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
    let outptr = Module.roundtrip(buf)
    let result = copyCStr(Module, outptr)
    return result
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
            return true;
        default: return null
    }
}

async function setupCompiler() {
    window.Module = {}
    let mod = await fetchAndInstantiate('/isymtope-small.wasm', imports)
    Module.memory = memory
    // Module.compile_template_source_to_html = mod.exports.compile_template_source_to_html
    Module.roundtrip = mod.exports.roundtrip
    Module.alloc   = mod.exports.alloc
    Module.dealloc = mod.exports.dealloc
    Module.dealloc_str = mod.exports.dealloc_str
    // Module.memory  = new Uint8Array(mod.exports.memory.buffer)
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

    let editorComponentDiv = document.getElementById('editorComponent')
    let editorDiv = document.createElement('div')
    editorDiv.setAttribute('id', 'editor')
    editorComponentDiv.appendChild(editorDiv)

    await setupCompiler()

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
    });
})