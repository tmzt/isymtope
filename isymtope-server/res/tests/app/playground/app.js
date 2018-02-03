
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

// window.addEventListener('message', ({data}) => {
//     switch(data.topic) {
//         case '/main/updatePreview':
//             let output = data.output
//             let uri = URL.createObjectURL(new Blob([output], {type: 'text/html'}));
//             let iframe = document.querySelector('iframe#preview')
//             iframe.src = uri
//             break;
//     }
// })

function compilerReducer(state, action) {
    switch(action.type) {
        case 'COMPILER.COMPILE':
            let source = editor.getValue();
            if (compiler)
            {
                compiler.postMessage({ topic: '/compiler/updatePreview', source })
            }
            return true;
        default: return null
    }
}

let resourceWorker
async function setupResourceWorker() {
    let iframe = document.querySelector('iframe#preview')
    let reg = await navigator.serviceWorker.register('/app/playground/serviceWorker.js', { scope: '/app/playground/' })
    resourceWorker = reg.active
    // await navigator.serviceWorker.ready
    return true
}

let compiler
function setupCompilerWorker() {
    compiler = new Worker('/app/playground/worker.js')
    compiler.onmessage = ({data}) => {
        switch(data.topic) {
            case '/main/updatePreview':
                if (resourceWorker) {
                    resourceWorker.postMessage({ topic: '/serviceWorker/cachePreviewObject', content: data.output })
                }
                // let output = data.output
                // let uri = URL.createObjectURL(new Blob([output], {type: 'text/html'}));
                // let iframe = document.querySelector('iframe#preview')
                // iframe.src = uri
                break;
        }
    }
}

navigator.serviceWorker.onmessage = ({data}) => {
    switch (data.topic) {
        case '/mainWindow/cachePreviewObjectUpdated':
            let iframe = document.querySelector('iframe#preview')
            iframe.src = window.origin + '/app/playground/preview-1bcx1/'
            break;
    }
}

function setupPreview() {
    let previewId = '/app/playground/preview-1bcx1/'
    let iframe = document.querySelector('iframe#preview')
    iframe.src = previewId
}

function setupPreviewProxy() {
    let origin = window.origin
    let proxy = URL.createObjectURL(new Blob([`<html><head><script>
        navigator.serviceWorker.register(window.origin + '/app/playground/serviceWorker.js', { scope: window.location.href + '/' })
        navigator.serviceWorker.ready.then(reg => console.log('[preview frame proxy] Resource service worker ready'))
    </script></head><body>(preview)</body></html>`], { type: 'text/html' }))
    let iframe = document.querySelector('iframe#preview')
    iframe.src = proxy
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

            window.fetch('/app/playground/playground.ism').then(resp => {
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
    await setupResourceWorker()
    setupPreview()
    // setupPreviewProxy()
    await setupEditor()
    // await setupCompiler()
})
