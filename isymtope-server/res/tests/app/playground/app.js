
let baseUrl = !!document.baseURI ? new URL(document.baseURI).pathname.replace(/\/+$/, '') : ''
let mapRoute = href => (baseUrl.length ? baseUrl + '/' : '') + href.replace(/^\/+/, '').replace(/\/+$/, '')

let _workspaces
let _currentWorkspaceId
let _currentFileId

let _contentCache = new Map
let _editorModels = new Map

let _isChangingContent = false

let cacheKey = (workspaceId, fileId) => `[workspaceId=${workspaceId} fileId=${fileId}]`
let fetchContent = async (workspaceId, fileData) => fetch(`${window.origin}/resources/app/${workspaceId}/${fileData.path}`).then(resp => resp.text())

async function getOrCache(cache, key, func) {
    if (cache.has(key)) { return cache.get(key) }
    let value = func()
    cache.set(key, value)
    return value
}

async function useModel() {
    const [ workspaceId, fileId ] = [ _currentWorkspaceId, _currentFileId ]
    const workspace = _workspaces.get(workspaceId)
    const file = workspace.files.filter(f => f.id === fileId)[0]

    const key = cacheKey(workspaceId, fileId)
    const content = await getOrCache(_contentCache, key, async () => fetchContent(workspaceId, file))
    const model = await getOrCache(_editorModels, key, async () => monaco.editor.createModel(content, file.language))
    model.setValue(content)
    monaco.editor.setModelLanguage(model, file.language === 'isymtope'  ?  'rust' : file.language)
    window._editor.setModel(model)
}

 function switchWorkspace(workspaceId) {
    if (_currentWorkspaceId === workspaceId) {  return }
    setPreview(`/resources/app/${workspaceId}`, true)
    let workspace = _workspaces.get(workspaceId)
    _currentFileId = workspace.index
    _currentWorkspaceId = workspaceId
    return useModel()
}

 function switchFile(fileId) {
    if (_currentFileId === fileId) { return }
    _currentFileId = fileId
    return useModel()
}

async function compileCurrent() {
    const [ workspaceId, fileId ] = [ _currentWorkspaceId, _currentFileId ]
    const key = cacheKey(workspaceId, fileId)
    const model = await getOrCache(_editorModels, key, async () => monaco.editor.createModel(""))
    const source = model.getValue()
    const appName = workspaceId

    let baseUrl = window.origin + `/resources/app/${appName}/`
    let templatePath = '/app.ism'
    let path = '/'

    setCompiling(true)

    return startCompilation(source, appName, baseUrl, templatePath, path)
}

async function loadDefault(workspaceId) {
    _currentWorkspaceId = workspaceId
    _currentFileId = undefined
    return useModel()
    // let fileData = { id: 'app.ism', path: 'app.ism', name: 'app.ism', language: 'isymtope', main: true}
    // return setEditorFileData(workspaceId, fileData)
}

function externAppReducer(state, action) {
    switch(action.type) {
        case 'EXTERNAPP.INIT':
            let { workspaces, workspaceId, fileId } = action
            setPreview(`/resources/app/${workspaceId}`, true)
            _workspaces = workspaces
            _currentWorkspaceId = workspaceId
            _currentFileId = fileId
            useModel()
            break;
        case 'EXTERNAPP.SWITCHWORKSPACE':
            switchWorkspace(action.workspaceId); break
        case 'EXTERNAPP.SWITCHFILE':
            switchFile(action.fileId); break
        case 'EXTERNAPP.COMPILECURRENT':
            compileCurrent(); break
    }
    return true
}

function externWorkspaceReducer(state, action) {
    switch(action.type) {
        case 'EXTERNWORKSPACE.SWITCHWORKSPACE':
            _workspaces = new Map(action.workspaces.map(w => [w.id, w]))
    }
    return true
}

function externFileReducer(state, action) {
    switch(action.type) {
        case 'EXTERNFILE.SWITCHFILE':
            _workspaces = new Map(action.workspaces.map(w => [w.id, w]))
    }
    return true
}

function defaultWorkspaceReducer(state, action) {
    switch(action.type) {
        case 'DEFAULTWORKSPACE.LOADDEFAULT':
            setPreview(`/resources/app/${action.id}`, true)
            loadDefaultEditor(action.id)
    }
    return true
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

            editor.onDidChangeModelContent(e => {
                console.log('Changed editor content', e)
            })

            window.fetch(window.origin + '/app/todomvc/app.ism').then(resp => {
                resp.text().then(body => {
                    _isChangingContent = true
                    editor.setValue(body)
                    _isChangingContent = false
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
