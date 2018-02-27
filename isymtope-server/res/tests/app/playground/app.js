
let baseUrl = !!document.baseURI ? new URL(document.baseURI).pathname.replace(/\/+$/, '') : ''
let mapRoute = href => (baseUrl.length ? baseUrl + '/' : '') + href.replace(/^\/+/, '').replace(/\/+$/, '')

let cache = new Map

async function loadFiles(files) {
    for(let file of files) {
        let url = mapRoute(file.path)
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
        window.fetch(window.origin + '/' + path).then(resp => {
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

function loadPrerenderReducer(state, action) {
    switch(action.type) {
        case 'LOADPRERENDER.LOADPRERENDER':
            setPreview(`/resources/app/${action.id}`, true)
    }
    return true
}

function loadWorkspaceReducer(state, action) {
    switch(action.type) {
        case 'LOADWORKSPACE.LOADWORKSPACE':
            let workspaces = new Map(action.workspaces.map(w => [w.id, w]))
            let workspace = workspaces.get(action.id)
            let files = workspace.files
            let mainFile = files.filter(f => !!f.main)[0]

            setupPreview(workspace.name, true)
            loadFiles(files)
                .then(() => switchEditor(mainFile))
            return true
        default: return null
    }
}

function editorContentReducer(state, action) {
    switch(action.type) {
        case 'EDITORCONTENT.LOAD':
            let url = mapRoute(action.path)
            fetchContent(url); return true;
        default: return null
    }
}

function compilerReducer(state, action) {
    switch(action.type) {
        case 'COMPILER.COMPILE':
            let source = editor.getValue()
            let appName = action.id
            let baseUrl = window.origin + `/resources/app/${appName}/`
            let templatePath = '/app.ism'
            let path = '/'

            setCompiling(true)
            startCompilation(source, appName, baseUrl, templatePath, path)
            return true;
        default: return null
    }
}

async function startCompilation(source, app_name, base_url, template_path, path) {
    let compilerWorker = getOrRegisterCompilerWorker()
    let resourceWorker = await getOrRegisterResourceWorker()

    let compilerToResourceWorker = new MessageChannel()
    let resourceWorkerToMainWindow = new MessageChannel()

    resourceWorkerToMainWindow.port1.onmessage = ({data}) => {
        switch (data.topic) {
            case '/mainWindow/refreshPreview':
                setCompiling(false)
                setPreview('/app/playground/preview-1bcx1/', false)
        }
    }

    let compileReq = {
        topic: '/compilerWorker/startCompilation',
        source,
        pathname: '',
        mimeType: 'text/html',
        app_name,
        base_url,
        template_path,
        path
    }

    resourceWorker.postMessage({ topic: '/resourceWorker/compilationStarted' }, [compilerToResourceWorker.port2, resourceWorkerToMainWindow.port2])
    compilerWorker.postMessage(compileReq, [compilerToResourceWorker.port1])
    return true
}

let compiler
function getOrRegisterCompilerWorker() {
    if (!compiler) {
        compiler = new Worker('/app/playground/worker.js')
    }
    return compiler
}

async function getOrRegisterResourceWorker() {
    if (navigator.serviceWorker.controller) {
        return navigator.serviceWorker.controller
    }

    let reg = await navigator.serviceWorker.register('/app/playground/serviceWorker.js', { scope: '/app/playground/' })
    return reg.active
}

function setupEditor() {
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
    editorDiv.setAttribute('id', 'editorDiv')
    editorComponentDiv.appendChild(editorDiv)

    return new Promise(resolve => {
        require(["vs/editor/editor.main"], function () {
            editor = monaco.editor.create(editorDiv, {
                value: '',
                theme: 'vs-dark'
            });

            window.fetch(window.origin + '/app/todomvc/app.ism').then(resp => {
                resp.text().then(body => {
                    editor.setValue(body)
                })
            })

            window._editor = editor

            resolve()
        });
    })
}

function setCompiling(v) {
    let component = document.querySelector('#previewComponent')
    component.classList.toggle('showLoading', v)
}

function setPreview(path, isPrerender) {
    setCompiling(false)
    let wrapper = document.querySelector('#previewWrap')
    wrapper.classList.toggle('isBlank', false)
    wrapper.classList.toggle('isPrerender', !!isPrerender)
    let iframe = document.querySelector('iframe#preview')
    iframe.src = window.origin + path
}

document.addEventListener('DOMContentLoaded', async () => {
    getOrRegisterCompilerWorker()
    await getOrRegisterResourceWorker()
    await setupEditor()

    // Go to default route in router
    window._go('/')
})
