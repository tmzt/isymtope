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

let apiUrl = window.origin + '/api/'
let baseUrl = !!document.baseURI ? new URL(document.baseURI).pathname.replace(/\/+$/, '') : ''
let mapRoute = href => (baseUrl.length ? baseUrl + '/' : '') + href.replace(/^\/+/, '').replace(/\/+$/, '')

const cachedLoadedAppState = slug => getOrCache(_slugCache, slug, async () => {
    let data = await (await getService(PlaygroundApiService)).getApp(slug)
    const loadedApp = new LoadedApp(data)
    return new LoadedAppState(loadedApp)
})

class LoadedApp {
    constructor(data) {
        this._data = data
        this._fileCache = new Map
    }

    get data() { return this._data }
    get previewBaseUrl() { return this._data.iframe_base }

    async loadFile(fileId) {
        let cache = this._fileCache.get(fileId)
        if (cache) { return cache }

        const slug = this._data.static_template || this._data.slug
        return fetch(`${window.origin}/resources/app/${slug}/${fileId}`).then(async resp => {
            const content = await resp.text()
            this._fileCache.set(fileId, content)
            return content
        })
    }
}

class LoadedAppState {
    constructor(loadedApp) {
        this._loadedApp = loadedApp
        this._currentFileId = null
    }

    get loadedApp() { return this._loadedApp }
    get currentFileId() { return this._currentFileId }
    get slug() { return this._loadedApp.data.slug }
    get currentFileKey() { return `${this._loadedApp.data.slug}/${this._currentFileId}` }

    async switchFile(fileId) {
        if (this._currentFileId == fileId) { return }
        this._currentFileId = fileId
        const content = await this._loadedApp.loadFile(fileId)
        const editorService = await getService(EditorService)
        await editorService.setContent(content)
        await editorService.useModel()
        await (await getService(EditorService)).setContent(content)
    }
}














// Globals
let _workspaces
let _slugCache = new Map()

let _activeSlug
let _currentWorkspaceId

let _iframe_base
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

class PlaygroundApiService {
    async createExample(exampleName) {
        const req = { template_name: exampleName }
        return fetch('/api/create_example', { method: 'POST', headers: { 'content-type': 'application/json'}, body: JSON.stringify(req) })
            .then(async resp => {
                if (!resp.ok) {
                    return reject(new Error(await resp.text()))
                }
                return resp.json()

                // let data = await resp.json()
                // if (location.href !== data.redirect) {
                //     history.pushState({ slug: data.slug, template_name: data.template_name, pathname: data.path }, null, data.redirect)
                // }

                // _iframe_base =data.iframe_base
                // return data
            })
    }

    async getApp(slug) {
        return fetch(`/api/apps/${slug}`)
            .then(async resp => {
                if (!resp.ok) {
                    return reject(new Error(await resp.text()))
                }
                return resp.json()
            })
        }
}

class RemoteCompilerService extends CompilerService {
    startCompilation(opts) {
        return new Promise((resolve, reject) =>
            fetch(`/api/apps/${opts.slug}/compile`, { method: 'POST', body: opts.source })
                .then(async resp => {
                    if (!resp.ok) {
                        return reject(new Error(await resp.text()))
                        // return reject(new CompilationError(await resp.text()))
                    }

                    return resolve(resp.text())
                }))
    }
}

class PreviewFrame {
    constructor() {
        this._wrapper = document.querySelector('#previewWrap')
        this._iframe = this._wrapper.querySelector('#preview')
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

    sendMessage(msg, origin) {
        return new Promise(resolve => {
            const completion = new MessageChannel()
            const iframe = this._iframe
            completion.port1.onmessage = () => {
                console.log(`[PreviewFrame] [${msg.type}] got completion message from iframe`)
                resolve()
            }
            console.log(`[PreviewFrame] [${msg.type}] sending message to iframe`)
            iframe.contentWindow.postMessage({  __isymtopePlaygroundFrameMsg: msg }, origin, [completion.port2])
        })
    }
}

class PreviewService {
    constructor() {
    }

    async showPreview() {
        const frame = await getService(PreviewFrame)
        const origin = app.loadedAppState.loadedApp.previewBaseUrl
        await frame.loadOrigin(origin)
        return frame.sendMessage({ type: 'registerWorker' }, origin)
    }

    async mergeContent(content) {
        const frame = await getService(PreviewFrame)
        const origin = app.loadedAppState.loadedApp.previewBaseUrl
        return frame.sendMessage({ type: 'mergeDoc', content }, origin)
    }
}

class RealtimeCompiler {
    async compile() {
        const editorService = await getService(EditorService)
        const previewService = await getService(PreviewService)
        const source = await editorService.getContent()
        const slug = app.loadedAppState.slug
        const loadedApp = app.loadedAppState.loadedApp
        const opts = { source, slug: slug, appName: loadedApp.data.static_template, baseUrl: loadedApp.previewBaseUrl, templatePath: '/app.ism', path: '/' }

        app.setCompiling(true)
        app.setCompilationStatus(true, "")

        const compilerService = await getCompilerService()

        return compilerService.startCompilation(opts)
            .then(async content => {
                        app.setCompiling(false)
                        app.setCompilationStatus(true, "")
                        return previewService.mergeContent(content)
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
                this._editor = editor
                window._editor = editor

                resolve(editor)
            });
        })
    }

    async getEditorModel() {
        return getOrCache(_editorModels, app.loadedAppState.currentFileKey, async () => monaco.editor.createModel(""))
    }

    async getCachedContent() {
        return getOrCache(_contentCache, app.loadedAppState.currentFileKey, async () => fetchContent(workspace.id, file))
    }

    async getContent() {
        const model = await this.getEditorModel()
        return model.getValue()
    }

    async setContent(content) {
        const model = await this.getEditorModel()
        this._settingContent = true
        model.setValue(content)
        this._settingContent = false
    }

    async useModel() {
        const model = await this.getEditorModel()
        this._editor.setModel(model)
    }

}

class App {
    constructor() {
        this._initialized = false
        this._slug = null
        this._loadedAppState = null
    }

    get loadedAppState() { return this._loadedAppState }

    /** Load an example workspace, generating a new slug (id) and pushing the new pathname onto the router */
    async loadExample(exampleName) {
        const apiService = await getService(PlaygroundApiService)

        // if (!this._initialized) {
        //     await this.initialize()
        // }

        let data = await apiService.createExample(exampleName)
        history.pushState({ slug: data.slug, pathname: data.path, href: data.href }, null, data.path)
        return this.loadApp(data.slug)
    }

    /** Load a workspace given a slug, such as in response to a navigation event. */
    async loadApp(slug) {
        this._slug = slug
        this._loadedAppState = await cachedLoadedAppState(slug)
        await this._loadedAppState.switchFile('app.ism')

        return (await getService(PreviewService)).showPreview()

        // const content = await this._loadedAppState.loadedApp.loadFile(this._loadedAppState.currentFileId)
        // await (await getService(EditorService)).setContent(content)

        // const slugObj = getSlug(slug)
        // await (await getService(EditorService)).useModel()
        // await (await app.getLoadedApp()).updatePreview()
    }

    async initialize(workspaces = undefined) {
        if (workspaces) { _workspaces = workspaces }
        this._initialized = true
    }

    async switchFile(fileId) {
        return (await this.getLoadedApp()).switchFile(fileId)
    }

    async compileCurrent() {
        return (await this.getLoadedApp()).compile()
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

const _compilerServices = new Map
const getCompilerService = () => getOrCache(_compilerServices, 'remote', () => new RemoteCompilerService())

const _contentCache = new Map
const _editorModels = new Map

const cacheKey = (workspaceId, fileId) => `[workspaceId=${workspaceId} fileId=${fileId}]`
const fetchContent = async (workspaceId, fileData) => fetch(`${window.origin}/resources/app/${workspaceId}/${fileData.path}`).then(resp => resp.text())

const _triggerCompileCurrent = debounce(async () => (await getService(RealtimeCompiler)).compile(), 2000)

const app = new App()

function externAppReducer(state, action) {
    switch(action.type) {
        case 'EXTERNAPP.INIT':
            app.initialize(action.workspaces, action.activeWorkspaceId, action.activeFileId); break
        case 'EXTERNAPP.LOADEXAMPLE':
            app.loadExample(action.exampleName); break
        case 'EXTERNAPP.LOADAPP':
            app.loadApp(action.slug); break

        case 'EXTERNAPP.SWITCHWORKSPACE':
            app.switchWorkspace(action.workspaceId); break
        case 'EXTERNAPP.SWITCHFILE':
            app.switchFile(action.fileId); break
        case 'EXTERNAPP.COMPILECURRENT':
            _triggerCompileCurrent(); break
        case 'EXTERNAPP.UPDATERESOURCE':
            updatePreviewResource(action.pathname, action.fileId); break

        case 'EXTERNAPP.SAVE':
            getService(GistService).then(s => s.save()); break
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
