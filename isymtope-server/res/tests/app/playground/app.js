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

let _frameId = 'xxxx_xxxx_xxxx_xxxx'.replace(/x/g, () => Math.floor(Math.random() * 10))

class CompilerService
{
    async prepareService() {
    }

    async startCompilation(opts) {
        throw new Error('Unimplemented')
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
        this._frameOrigin = null
    }

    loadOrigin(origin) {
        return new Promise(resolve => {
            const iframe = this._iframe
            const load = () => resolve()
            iframe.addEventListener('load', () => {
                load()
                iframe.removeEventListener('load', load)
            })
            iframe.src = origin
        })
    }

    async sendMessage(msg, origin, load = false) {
        if (load) { await this.loadOrigin(origin) }
        return new Promise(resolve => {
            const completion = new MessageChannel()
            const iframe = this._iframe
            completion.port1.onmessage = () => {
                console.log(`[PreviewFrameService] [${msg.type}] got completion message from iframe`)
                resolve()
            }
            console.log(`[PreviewFrameService] [${msg.type}] sending message to iframe`)
            iframe.contentWindow.postMessage({  __isymtopePlaygroundFrameMsg: msg }, origin, [completion.port2])
        })
    }

    async setupInitialPreview() {
        const workspace = await app.getCurrentWorkspace()
        const isPrerender = workspace.data.prerender !== false

        const opts = { frameId: _frameId, appName: workspace.id, isPrerender }
        return this.initializePreviewFrame(opts)
    }

    async initializePreviewFrame(opts) {
        this._frameOrigin = `http://s1234.${opts.appName}.app.isymtope.ws.localhost:3000/`

        const wrapper = document.querySelector('#previewWrap')
        return this.sendMessage({ type: 'registerWorker' }, this._frameOrigin, true)
        .then(() => {
            console.log('[initializePreviewFrame] got completion message from iframe')
            wrapper.classList.toggle('isBlank', false)
            wrapper.classList.toggle('isPrerender', true)
        })
    }

    async forwardWorkerMessage(workerMsg) {
        return this.sendMessage({ type: 'forwardMessage', msg: workerMsg }, this._frameOrigin)
    }

    async updatePreview(workspaceId, frameId, content) {
        return this.sendMessage({ type: 'mergeDoc', content }, this._frameOrigin)
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

    async switchFile(fileId) {
        if (this._currentFileId === fileId) { return }
        this._currentFileId = fileId
        return (await getService(EditorService)).useModel()
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
            require(["vs/editor/editor.main"], () => {
                const editor = monaco.editor.create(editorDiv, {
                    value: '',
                    theme: 'vs-dark'
                });

                editor.onDidChangeModelContent(async e => {
                    console.log('Changed editor content', e)
                    if (!this._settingContent) {
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

    async getCachedContent() {
        const model = await this.getEditorModel()
        const workspace = await app.getCurrentWorkspace()
        const file = workspace.currentFile
        const key = cacheKey(workspace.id, file.id)
        return getOrCache(_contentCache, key, async () => fetchContent(workspace.id, file))
    }

    async setContent(content) {
        const model = await this.getEditorModel()
        this._settingContent = true
        model.setValue(content)
        this._settingContent = false
    }

    async useModel() {
        const workspace = await app.getCurrentWorkspace()
        const file = workspace.currentFile
        const model = await this.getEditorModel()
        const content = await this.getCachedContent()

        await this.setContent(content)
        monaco.editor.setModelLanguage(model, file.language === 'isymtope'  ?  'rust' : file.language)
        window._editor.setModel(model)
    }

}

class App {
    constructor() {
        this._initialized = false
        this._currentWorkspaceId = null
    }

    get currentWorkspaceId() { return this._currentWorkspaceId }

    async getCurrentWorkspace() {
        return getWorkspace(this._currentWorkspaceId)
    }

    async initialize(workspaces, startingWorkspaceId = null) {
        if (this._initialized || this._currentWorkspaceId) {
            throw new Error('Invalid state: already initialized')
        }
        _workspaces = workspaces
        this._currentWorkspaceId = startingWorkspaceId

        await (await getService(EditorService)).useModel()
        await (await getService(PreviewFrameService)).setupInitialPreview()

        this._initialized = true
    }

    async switchWorkspace(workspaceId) {
        if (_currentWorkspaceId === workspaceId) {  return }
        this._currentWorkspaceId = workspaceId

        await (await getService(EditorService)).useModel()
        await (await app.getCurrentWorkspace()).updatePreview()
    }

    async switchFile(fileId) {
        return (await this.getCurrentWorkspace()).switchFile(fileId)
    }

    async compileCurrent() {
        return (await this.getCurrentWorkspace()).compile()
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
const getCompilerService = () => getOrCache(_compilerServices, 'remote', () => new RemoteCompilerService())

const _contentCache = new Map
const _editorModels = new Map

const cacheKey = (workspaceId, fileId) => `[workspaceId=${workspaceId} fileId=${fileId}]`
const fetchContent = async (workspaceId, fileData) => fetch(`${window.origin}/resources/app/${workspaceId}/${fileData.path}`).then(resp => resp.text())

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

const navigate = Isymtope.navigate

Isymtope.app()
    .setDefaultRoute('/')
    .alwaysNavigateToDefaultRoute(false)
    .registerBeforeRoutingHook(async store => {
        store.dispatch(async (dispatch, getState) =>
            getCompilerService()
                .then(compilerService => compilerService.prepareService())
                .then(() => getService(EditorService))
                .then(editorService => editorService.getEditor())
                .then(() => dispatch(navigate('/'))))
    })
