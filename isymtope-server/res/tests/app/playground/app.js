const _services = new Map()

async function getOrCache(cache, key, func) {
    if (cache.has(key)) { return cache.get(key) }
    let value = func()
    cache.set(key, value)
    return value
}

function debounce(fn, delay) {
    let timer = null
    return function(args) {
        clearTimeout(timer)
        setTimeout(() => fn(args), delay)
    }
}

const getService = async (cls) => getOrCache(_services, cls.name, () => new cls())

let apiUrl = window.origin + '/app/playground/api/'
let baseUrl = !!document.baseURI ? new URL(document.baseURI).pathname.replace(/\/+$/, '') : ''
let mapRoute = href => (baseUrl.length ? baseUrl + '/' : '') + href.replace(/^\/+/, '').replace(/\/+$/, '')

// Globals
let _workspaces
let _currentWorkspaceId
let _setContentCalled = false

let _frameId = 'xxxx_xxxx_xxxx_xxxx'.replace(/x/g, () => Math.floor(Math.random() * 10))

class CompilerService
{
    async prepareService() {
    }

    async startCompilation(opts) {
        throw new Error('Unimplemented')
    }
}

class WasmCompilerService extends CompilerService
{
    prepareService() {
        return getOrRegisterCompilerWorker()
    }

    async startCompilation(opts) {
        const compilerWorker = await getOrRegisterCompilerWorker()

        return new Promise((resolve, reject) => {
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

            completion.onmessage = data => {
                if (data.error) {
                    resolve(data.content)
                } else {
                    reject(data.error)
                }
            }
        })
    }
}

class CompilationError extends Error {
    constructor(err) {
        super()
    }
}

class RemoteCompilerService extends CompilerService
{
    startCompilation(opts) {
        return new Promise((resolve, reject) =>
            fetch(apiUrl +'compile', { method: 'POST', body: opts.source })
                .then(async resp => {
                    if (!resp.ok) {
                        return reject(await resp.text())
                        // return reject(new CompilationError(await resp.text()))
                    }

                    return resolve(resp.text())
                }))
    }
}

class PreviewFrameService {
    constructor() {
        this._wrapper = document.querySelector('#previewWrap')
        this._iframe = this._wrapper.querySelector('#preview')
    }

    async setupInitialPreview() {
        const contentWorkerService = await getService(ContentWorkerService)

        const workspace = await app.getCurrentWorkspace()
        const isPrerender = workspace.data.prerender !== false
        const appPath = `/app/${workspace.id}`

        const opts = { appPath, frameId: _frameId, isPrerender }
        return contentWorkerService.initializePreviewFrame(opts)
    }

    async updatePreview(workspaceId, frameId, content) {
        return new Promise(resolve => {
            const completion = new MessageChannel()
            const wrapper = this._wrapper
            const iframe = this._iframe
    
            const msg = { _mergeDoc: content, [`_previewIframe${_frameId}`]: true }
            completion.port1.onmessage = () => {
                console.log('[setPreviewContent] got completion message from iframe')
                wrapper.classList.remove('isPrerender')
                resolve()
            }
            iframe.contentWindow.postMessage(msg, window.origin, [completion.port2])
        })
    }
}

const determineMimeType = (pathname) => {
    let mimeType = 'text/plain'
    if (pathname.match(/\.html$/)) { mimeType = 'text/html' }
    if (pathname.match(/\.css$/)) { mimeType = 'text/css' }
    if (pathname.match(/\.js$/)) { mimeType = 'text/javascript' }
    if (pathname.match(/\.png$/)) { mimeType = 'image/png' }
    if (pathname.match(/\.jpg$/)) { mimeType = 'image/jpeg' }
    if (pathname.match(/\.gif$/)) { mimeType = 'image/gif' }
    return mimeType
}

class ContentWorkerService {
    constructor() {
    }

    async initializePreviewFrame(opts) {
        const resourceWorker = await getOrRegisterResourceWorker()
        return new Promise(resolve => {
            const completion = new MessageChannel()
            completion.port1.onmessage = () => {
                console.log('[initialize preview frame] got completion message from worker')
                const wrapper = document.querySelector('#previewWrap')
                wrapper.classList.toggle('isBlank', false)
                wrapper.classList.toggle('isPrerender', true)
                const iframe = document.querySelector('iframe#preview')
                iframe.src = `/app/playground/_worker${appPath}`
                // _shouldUpdate = true
                resolve()
            }
            const { appPath, frameId, isPrerender } = opts
            resourceWorker.postMessage({ topic: '/resourceWorker/initializePreviewFrame', appPath, frameId, isPrerender }, [completion.port2])
        })
    }

    async updateResource(pathname, content, mimeType = undefined) {
        const resourceWorker = await getOrRegisterResourceWorker()
        mimeType = mimeType || determineMimeType(pathname)

        return new Promise(async resolve => {
            const completion = new MessageChannel()
            const model = await getEditorModel()
            const content = model.getValue()
            const msg = { topic: '/resourceWorker/cacheResource', pathname, mimeType, content }
            completion.port1.onmessage = () => {
                console.log('[ContentWorkerService updateResource] got completion message from worker')
                resolve()
            }
            resourceWorker.postMessage(msg, [completion.port2])
        })
    }
}

class Workspace {
    constructor(workspaceData) {
        this._name = workspaceData.name
        this._id = workspaceData.id
        this._workspaceData = workspaceData
        this._currentFileId = workspaceData.index
        this._content = null

        this._previewRendered = false
    }

    get id() { return this._id }
    get data() { return this._workspaceData }

    get currentFileId() { return this._currentFileId }
    get currentFile() { return this._workspaceData.files.filter(f => f.id === this._currentFileId)[0] }

    async loadInitial() {
    }

    async load() {
    }

    async switchFile(fileId) {
        if (this._currentFileId === fileId) { return }
        this._currentFileId = fileId
        return useModel()
    }

    async setupPreview() {
        const previewFrameService = await getService(PreviewFrameService)
        await previewFrameService.setupInitialPreview()
        this._previewRendered = true
    }

    async updatePreview() {
        if (!this._previewRendered || !this._content) {
            return this.setupPreview()
        }
        
        const previewFrameService = await getService(PreviewFrameService)
        await previewFrameService.updatePreview(this.id, _frameId, this._content)
    }

    async compile() {
        const editorService = await getService(EditorService)
        const previewFrameService = await getService(PreviewFrameService)
        const contentWorkerService = await getService(ContentWorkerService)
        const model = await editorService.getEditorModel()

        const source = model.getValue()
        const appName = this.id
        const baseUrl = window.origin + `/app/playground/_worker/app/${appName}/`
        const templatePath = '/app.ism'
        const path = '/'
        const opts = { source, appName, baseUrl, templatePath, path }

        app.setCompiling(true)
        app.setCompilationStatus(true, "")

        const compilerService = await getCompilerService()

        return compilerService.startCompilation(opts)
            .then(async content => {
                        app.setCompiling(false)
                        app.setCompilationStatus(true, "")

                        this._content = content
                        await this.updatePreview()
                        // if (_shouldUpdate) {
                        //     await previewFrameService.updatePreview(content)
                        // } else {
                        //     await this.updatePreview()
                        // }
            })
            .catch(err => {
                console.warn(`[compiler] Remote compiler error: ${err}`)
                app.setCompiling(false)
                app.setCompilationStatus(false, err)
            })
    }
}

class EditorService {
    constructor() {
        this._editor = null
        this._editorComponentDiv = document.querySelector('#editorComponent')
        this._editorDiv = this._editorComponentDiv.querySelector('#editorDiv')
    }

    async getEditor() {
        if (this._editor) { return this._editor }

        const proxy = URL.createObjectURL(new Blob([`
            self.MonacoEnvironment = { baseUrl: 'https://unpkg.com/monaco-editor@0.8.3/min/' };
            importScripts('https://unpkg.com/monaco-editor@0.8.3/min/vs/base/worker/workerMain.js');
        `], { type: 'text/javascript' }));

        require.config({ paths: { 'vs': 'https://unpkg.com/monaco-editor@0.8.3/min/vs' }});
        window.MonacoEnvironment = { getWorkerUrl: () => proxy };

        return new Promise(resolve => {
            const editorDiv = this._editorDiv
            require(["vs/editor/editor.main"], function () {
                const editor = monaco.editor.create(editorDiv, {
                    value: '',
                    theme: 'vs-dark'
                });

                editor.onDidChangeModelContent(async e => {
                    console.log('Changed editor content', e)
                    if (!_setContentCalled) {
                        return _triggerCompileCurrent()
                    }
                })
                window._editor = editor

                resolve(editor)
            });
        })
    }

    async getEditorModel() {
        const workspace = app.getCurrentWorkspace()
        const key = `${workspace.id}_${workspace.currentFileId}`
        return getOrCache(_editorModels, key, async () => monaco.editor.createModel(""))
    }
}

class App {
    constructor() {
        this._initialized = false
        this._currentWorkspaceId = null
    }

    get currentWorkspaceId() { return this._currentWorkspaceId }

    async initialize(workspaces, startingWorkspaceId = null) {
        if (this._initialized || this._currentWorkspaceId) {
            throw new Error('Invalid state')
        }
        _workspaces = workspaces
        this._currentWorkspaceId = startingWorkspaceId

        const previewFrameService = await getService(PreviewFrameService)
        await previewFrameService.setupInitialPreview()

        await useModel()
        this._initialized = true
    }

    async getCurrentWorkspace() {
        return getWorkspace(this._currentWorkspaceId)
    }

    async switchWorkspace(workspaceId) {
        if (_currentWorkspaceId === workspaceId) {  return }

        this._currentWorkspaceId = workspaceId
        await useModel()
        const workspace = await app.getCurrentWorkspace()
        return workspace.updatePreview()
    }

    async switchFile(fileId) {
        const workspace = await this.getCurrentWorkspace()
        return workspace.switchFile(fileId)
    }

    async compileCurrent() {
        const workspace = await this.getCurrentWorkspace()

        return workspace.compile()
    }

    setCompiling(v) {
        const component = document.querySelector('#previewComponent')
        component.classList.toggle('showLoading', v)
    }

    setCompilationStatus(successful, err_text = '') {
        const compilation = document.querySelector('#compilationErrorOverlay')
        compilation.classList.toggle('show', !successful)
        if (!successful) {
            compilation.querySelector('.err_text').innerText = err_text
        }
    }
}

const _workspaceObjects = new Map
const getWorkspace = async (workspaceId) => getOrCache(_workspaceObjects, workspaceId, async () => new Workspace(_workspaces.get(workspaceId)))

const _compilerServices = new Map
const getCompilerService = () => getOrCache(_compilerServices, 'remote', (getOrRegisterCompilerWorker) => new RemoteCompilerService())

const _contentCache = new Map
const _editorModels = new Map

const cacheKey = (workspaceId, fileId) => `[workspaceId=${workspaceId} fileId=${fileId}]`
const fetchContent = async (workspaceId, fileData) => fetch(`${window.origin}/resources/app/${workspaceId}/${fileData.path}`).then(resp => resp.text())

async function useModel() {
    const editorService = await getService(EditorService)

    const workspace = await app.getCurrentWorkspace()
    const fileId = workspace.currentFileId
    const file = workspace.currentFile

    const key = cacheKey(workspace.id, fileId)
    const model = await editorService.getEditorModel()
    const content = await getOrCache(_contentCache, key, async () => fetchContent(workspace.id, file))

    _setContentCalled = true
    model.setValue(content)
    _setContentCalled = false
    monaco.editor.setModelLanguage(model, file.language === 'isymtope'  ?  'rust' : file.language)
    window._editor.setModel(model)
}

const _triggerCompileCurrent = debounce(() => app.compileCurrent(), 2000)

const app = new App()

function externAppReducer(state, action) {
    switch(action.type) {
        case 'EXTERNAPP.INIT':
            app.initialize(action.workspaces, action.workspaceId); break
        case 'EXTERNAPP.SWITCHWORKSPACE':
            app.switchWorkspace(action.workspaceId); break
        case 'EXTERNAPP.SWITCHFILE':
            app.switchFile(action.fileId); break
        case 'EXTERNAPP.COMPILECURRENT':
            _triggerCompileCurrent(); break
        case 'EXTERNAPP.UPDATERESOURCE':
            updatePreviewResource(action.pathname, action.fileId); break
    }
    return true
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


let _shouldUpdate = false

function setPreviewContent(content) {
    return new Promise(resolve => {
        const completion = new MessageChannel()
        const wrapper = document.querySelector('#previewWrap')
        const iframe = document.querySelector('iframe#preview')

        const msg = { _mergeDoc: content, [`_previewIframe${_frameId}`]: true }
        iframe.contentWindow.postMessage(msg, window.origin, [completion.port2])

        completion.port1.onmessage = () => {
            console.log('[setPreviewContent] got completion message from iframe')
            wrapper.classList.remove('isPrerender')
            resolve()
        }
    })
}

const navigate = Isymtope.navigate

Isymtope.app()
    .setDefaultRoute('/')
    .alwaysNavigateToDefaultRoute(false)
    .registerBeforeRoutingHook(async store => {
        store.dispatch(async (dispatch, getState) =>
            getCompilerService()
                .then(compilerService => compilerService.prepareService())
                .then(() => getOrRegisterResourceWorker())
                .then(() => getService(EditorService))
                .then(editorService => editorService.getEditor())
                .then(() => dispatch(navigate('/'))))
    })
