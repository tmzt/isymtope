let apiUrl = window.origin + '/app/playground/api/'
let baseUrl = !!document.baseURI ? new URL(document.baseURI).pathname.replace(/\/+$/, '') : ''
let mapRoute = href => (baseUrl.length ? baseUrl + '/' : '') + href.replace(/^\/+/, '').replace(/\/+$/, '')


class CompilerService
{
    async prepareService() {
    }

    startCompilation(opts, cb) {
        throw new Error('Unimplemented')
    }
}

class WasmCompilerService extends CompilerService
{
    prepareService() {
        return getOrRegisterCompilerWorker()
    }

    async startCompilation(opts, cb) {
        let compilerWorker = await getOrRegisterCompilerWorker()
        let completion = new MessageChannel()
        let compileReq = {
            topic: '/compilerWorker/compile',
            source,
            pathname: '',
            mimeType: 'text/html',
            app_name,
            baseUrl,
            template_path,
            path
        }

        completion.onmessage = data => cb(data.content)
    }
}

class RemoteCompilerService extends CompilerService
{
    async startCompilation(opts, cb) {
        return fetch(apiUrl + 'compile', {
                method: 'POST',
                body: opts.source
            })
            .then(resp => resp.text())
            .then(cb)
    }
}

let _compilerServices = new Map
let getCompilerService = () => getOrCache(_compilerServices, 'remote', (getOrRegisterCompilerWorker) => new RemoteCompilerService())

let _workspaces
let _currentWorkspaceId
let _currentFileId

let _frameId = 'xxxx_xxxx_xxxx_xxxx'.replace(/x/g, () => Math.floor(Math.random() * 10))

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
    const model = await getEditorModel()
    const content = await getOrCache(_contentCache, key, async () => fetchContent(workspaceId, file))

    _isChangingContent = true
    model.setValue(content)
    _isChangingContent = false
    monaco.editor.setModelLanguage(model, file.language === 'isymtope'  ?  'rust' : file.language)
    window._editor.setModel(model)
}

// function attachEditorEvents(contentChanged) {
//     _editor.onDidChangeModelContent(event => {
//         console.log('Content changed', event)
//         contentChanged()
//     })
// }

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

async function getEditorModel() {
    await getEditor()
    const key = cacheKey(_currentWorkspaceId, _currentFileId)
    return getOrCache(_editorModels, key, async () => monaco.editor.createModel(""))
}

async function compileCurrent() {
    const [ workspaceId, fileId ] = [ _currentWorkspaceId, _currentFileId ]
    const key = cacheKey(workspaceId, fileId)
    // const model = await getOrCache(_editorModels, key, async () => monaco.editor.createModel(""))
    const model = await getEditorModel()
    const source = model.getValue()
    const appName = workspaceId

    let baseUrl = window.origin + `/resources/app/${appName}/`
    let templatePath = '/app.ism'
    let path = '/'

    setCompiling(true)

    let opts = { source, appName, baseUrl, templatePath, path }
    return startCompilation(opts)
}

function debounce(fn, delay) {
    let timer = null
    return function(args) {
        clearTimeout(timer)
        setTimeout(() => fn(args), delay)
    }
}

const _compileCurrent = debounce(compileCurrent, 2000)

async function loadDefault(workspaceId) {
    _currentWorkspaceId = workspaceId
    _currentFileId = undefined
    return useModel()
    // let fileData = { id: 'app.ism', path: 'app.ism', name: 'app.ism', language: 'isymtope', main: true}
    // return setEditorFileData(workspaceId, fileData)
}

async function initializePreviewFrame(appPath, frameId) {
    const resourceWorker = await getOrRegisterResourceWorker()
    const completion = new MessageChannel()
    resourceWorker.postMessage({ topic: '/resourceWorker/initializePreviewFrame', appPath, frameId: _frameId }, [completion.port2])
    completion.port1.onmessage = () => {
        console.log('[initialize preview frame] got completion message from worker')
        const wrapper = document.querySelector('#previewWrap')
        wrapper.classList.toggle('isBlank', false)
        wrapper.classList.toggle('isPrerender', true)
        const iframe = document.querySelector('iframe#preview')
        iframe.src = `/app/playground/_worker${appPath}`
        _shouldUpdate = true
    }
}

function externAppReducer(state, action) {
    switch(action.type) {
        case 'EXTERNAPP.INIT':
            let { workspaces, workspaceId, fileId } = action
            // setPreview(`/resources/app/${workspaceId}`, true)
            _workspaces = workspaces
            _currentWorkspaceId = workspaceId
            _currentFileId = fileId

            useModel()
            initializePreviewFrame(`/app/${workspaceId}`)
            // _editor.onDidChangeModelContent(event => {
            //     if (!_isChangingContent) {
            //         let { activeWorkspaceId, activeFileId  } = getState()
            //         console.log('Content changed', event)
            //         dispatch({ type: 'EDITOREVENTS.CONTENTCHANGED', activeWorkspaceId, activeFileId })
            //     }
            // })
            break;
        case 'EXTERNAPP.SWITCHWORKSPACE':
            switchWorkspace(action.workspaceId); break
        case 'EXTERNAPP.SWITCHFILE':
            switchFile(action.fileId); break
        case 'EXTERNAPP.COMPILECURRENT':
            compileCurrent(); break
        case 'EXTERNAPP.UPDATERESOURCE':
            updatePreviewResource(action.pathname, action.fileId); break
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

async function startCompilation(opts) {
    let compilerService = await getCompilerService()

    setCompiling(true)
    compilerService.startCompilation(opts, content => {
                    setCompiling(false)
                    if (_shouldUpdate) {
                        setPreviewContent(content)
                    }
    })
}

async function updatePreviewResource(pathname, fileId) {
    const resourceWorker = await getOrRegisterResourceWorker()
    const resourceWorkerToMainWindow = new MessageChannel()
    const model = await getEditorModel()
    const content = model.getValue()
    let mimeType = 'text/plain'
    if (pathname.match(/\.html$/)) { mimeType = 'text/html' }
    if (pathname.match(/\.css$/)) { mimeType = 'text/css' }
    if (pathname.match(/\.js$/)) { mimeType = 'text/javascript' }
    if (pathname.match(/\.png$/)) { mimeType = 'image/png' }
    if (pathname.match(/\.jpg$/)) { mimeType = 'image/jpeg' }
    if (pathname.match(/\.gif$/)) { mimeType = 'image/gif' }
    const msg = { topic: '/resourceWorker/cacheResource', pathname, mimeType, content }
    resourceWorker.postMessage(msg, [resourceWorkerToMainWindow.port2])
}

let compiler
async function getOrRegisterCompilerWorker() {
    if (!compiler) {
        compiler = new Worker('/app/playground/worker.js')
    }
    return compiler
}

async function getOrRegisterResourceWorker() {
    if (navigator.serviceWorker.controller) {
        return navigator.serviceWorker.controller
    }

    let reg = await navigator.serviceWorker.register('/app/playground/serviceWorker.js', { scope: '/app/playground/_worker' })
    return reg.active
}

async function getEditor() {
    if (window._editor) {
        return window._editor
    }

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

            editor.onDidChangeModelContent(async e => {
                console.log('Changed editor content', e)
                if (!_isChangingContent) {
                    return _compileCurrent()
                }
            })

            // window.fetch(window.origin + '/app/todomvc/app.ism').then(resp => {
            //     resp.text().then(body => {
            //         _isChangingContent = true
            //         editor.setValue(body)
            //         _isChangingContent = false
            //     })
            // })

            window._editor = editor

            resolve(editor)
        });
    })
}

function setCompiling(v) {
    let component = document.querySelector('#previewComponent')
    component.classList.toggle('showLoading', v)
}

let _shouldUpdate = false

function setPreviewContent(content) {
    let iframe = document.querySelector('iframe#preview')
    // iframe.contentWindow.postMessage({type: 'replaceHtml', body}, '*')
    let msg = { _mergeDoc: content }
    msg[`_previewIframe${_frameId}`] = true
    iframe.contentWindow.postMessage(msg, window.origin)

    document.querySelector('#previewWrap').classList.remove('isPrerender')
}

function injectScript(content) {
    let script = '<script src="inject.js"></script></head>'
    return content.replace(/<\/head>/, script)
}

// function setPreview(path, isPrerender) {
//     setCompiling(false)
//     let wrapper = document.querySelector('#previewWrap')
//     wrapper.classList.toggle('isBlank', false)
//     wrapper.classList.toggle('isPrerender', !!isPrerender)
//     let iframe = document.querySelector('iframe#preview')
//     iframe.src = window.origin + path

//     _shouldUpdate = true

//     // setTimeout(() => {
//     //     window._editor.focus()
//     // }, 1000)
// }

// async function setPreview(appName, isPrerender) {
//     setCompiling(false)
//     let wrapper = document.querySelector('#previewWrap')
//     wrapper.classList.toggle('isBlank', false)
//     wrapper.classList.toggle('isPrerender', !!isPrerender)
//     let iframe = document.querySelector('iframe#preview')

//     // let content = await fetch(window.origin + path)
//     // content = injectScript(content)
//     // iframe.src = '/app/playground/preview-1bcx1/'
//     iframe.src = '/app/playground/_worker/app/${appName}'
//     _shouldUpdate = true
// }

const navigate = Isymtope.navigate

Isymtope.app()
    .setDefaultRoute('/')
    .alwaysNavigateToDefaultRoute(false)
    .registerBeforeRoutingHook(async store => {
        store.dispatch(async (dispatch, getState) =>
            getCompilerService()
                .then(compilerService => compilerService.prepareService())
                .then(() => getOrRegisterResourceWorker())
                .then(() => getEditor())
                .then(() => dispatch(navigate('/'))))
    })

// document.addEventListener('DOMContentLoaded', async () => {
//     // getOrRegisterCompilerWorker()
//     // await getOrRegisterResourceWorker()
//     // await setupEditor()

//     // Go to default route in router
//     // window._go('/')
// })
