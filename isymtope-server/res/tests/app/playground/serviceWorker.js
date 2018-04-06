const WORKER_ROOT_CACHE = 'playground-worker-cache'
const WORKER_UPSTREAM_CACHE = 'playground-worker-upstream-cache'
const OTHER_UPSTREAM_CACHE = 'playground-other-upstream-cache'

const WORKER_ROOT = '/app/playground/_worker/'
const WORKER_ROOT_REGEX = /^\/app\/playground\/_worker\//;

self.addEventListener('install', event => {
    console.log('% install')
    console.log('% requesting immediate activation')
    return self.skipWaiting()
})

self.addEventListener('activate', event => {
    console.log('% activate')
    console.log('% claiming controller for all clients')
    return self.clients.claim()
})

function injectScript(content, frameId) {
    frameId = frameId.replace(/[^a-zA-Z0-9]/g, '_')
    let script = `<script data-isymtope-inject data-_inject${frameId}="true">(function _inject${frameId} () { 
        function removeScripts(src) {
            let scr = [];
            [].forEach.call(src.querySelectorAll('script'), sc => {
                if (!sc.hasAttribute('src')) {
                    let scn = sc.parentNode.removeChild(sc);
                    if (!scn.hasAttribute('data-_inject${frameId}') && !scn.hasAttribute('data-isymtope-inject')) { scr.push(scn) }
                }
            })
            return scr
        }

        function mergeScripts(scr, dest, add) {
            if (add) { var sc = dest.ownerDocument.createElement('script'); sc.setAttribute('data-isymtope-inject', 'true'); sc.setAttribute('data-_inject${frameId}', 'true'); sc.textContent =  '(' + _inject${frameId}.toString() + '())'; scr.push(sc) }
            scr.forEach(osn => { var tC = osn.textContent; var scn = dest.ownerDocument.createElement('script'); scn.setAttribute('data-ts', new Date().getTime()); scn.textContent += tC;; dest.appendChild(scn) })
        }
        
        var _onmessage = window.onmessage; window.onmessage = function (e) { var msg = ('string' == typeof e.data) ? JSON.parse(e.data): e.data; if (msg && 'object' == typeof msg && msg._previewIframe${frameId} == true) {
            if (msg._newLocation) { location.href = msg._newLocation }
            else if (e.data._setInnerHTML) { document.body.innerHTML = e.data._setInnerHTML }
            else if (msg._mergeDoc) {
                var doc = document.implementation.createHTMLDocument(""); doc.documentElement.innerHTML = msg._mergeDoc;
                var headScr = removeScripts(doc.head);
                var bodyScr = removeScripts(doc.body);
                document.body.innerHTML = doc.body.innerHTML;
                mergeScripts(headScr, document.head, true);
                mergeScripts(bodyScr, document.body, false);

                Isymtope.app().update().then(() => e.ports[0].postMessage({}))
            } } else { if (_onmessage) { _onmessage.apply(window, [].slice.call(arguments)) } } }}())</script>`
    return content.replace(/<\/head>/, script)
}

const cached = async request => {
    let worker_cache = await caches.open(WORKER_ROOT_CACHE)
    let worker_upstream = await caches.open(WORKER_UPSTREAM_CACHE)

    let original_url = new URL(request.url);
    let original_url_path = original_url.pathname

    let worker_root_uri = self.origin + WORKER_ROOT

    if (original_url.origin === self.origin) {
        console.log(`[service worker] got request ${original_url_path}`)
        if (WORKER_ROOT_REGEX.test(original_url_path)) {
            let resource_path = original_url_path.replace(WORKER_ROOT_REGEX, '')
            console.log('[worker root] checking for cached preview resource: ' +resource_path)
            let worker_resource_req = new Request(worker_root_uri + resource_path)
            let worker_resource_match = await worker_cache.match(worker_resource_req)
            if (worker_resource_match) {
                console.log('[worker root] serving resource for request')
                return worker_resource_match
            }

            console.log(`[worker root] attempting to serve resource ${resource_path} from upstream cache`)
            let upstream_match = await worker_upstream.match(request)
            if (upstream_match) {
                console.log(`[worker root] serving resource ${resource_path} (${original_url_path}) from upstream cache`)
                return upstream_match
            }

            let upstream_resource_url = `${self.origin}/resources/${resource_path}`
            console.log(`[worker root] attempting to fetch upstream resource ${resource_path} from upstream url ${upstream_resource_url} and cache`)
            let upstreamReq= new Request(upstream_resource_url)
            let upstreamResp = await fetch(upstreamReq)
            await worker_upstream.put(request, upstreamResp.clone())
            return upstreamResp
        }

        // Pass other origin requests upstream
        return fetch(request)

        // Fail other _worker requests
        // let failedResp = new Response('Missing _worker resource')
        // failedResp.status = '404'
        // failedResp.statusText = 'File not found'
        // return failedResp
    }

    // Pass other requests upstream
    let upstreamResp = await fetch(request)
    // await upstream.put(request, upstreamResp.clone())

    return upstreamResp
}

async function cacheInitialPreview(appPath, frameId, isPrerender, completion) {
    let cache = await caches.open(WORKER_ROOT_CACHE)
    let upstreamPath
    if (isPrerender) {
        upstreamPath = self.origin + `/resources${appPath}/index.html`
    } else {
        upstreamPath = self.origin + `${appPath}`
    }
    let previewPath = `/app/playground/_worker${appPath}`
    let previewReq = new Request(self.origin + previewPath)

    console.log(`[worker root] fetching upstream resource ${upstreamPath}`)
    let upstreamResp = await fetch(new Request(upstreamPath));
    let content = injectScript(await upstreamResp.text(), frameId)
    let cacheResp = new Response([content], { contentType: 'text/html' })
    cacheResp.headers.set('Content-type', 'text/html')
    await cache.put(previewReq, cacheResp)

    completion.postMessage({})
}

async function cacheResource(resource_path, content, mimeType, completion) {
    let cache = await caches.open(WORKER_ROOT_CACHE)
    let worker_root_uri = self.origin + WORKER_ROOT
    let request = new Request(`${worker_root_uri}/${resource_path}`)
    let response = new Response([content], { contentType: mimeType })
    response.headers.set('content-type', mimeType)
    await cache.put(request, response)

    completion.postMessage({})
}

self.onmessage = async ({data, ports}) => {
    switch (data.topic) {
        case '/resourceWorker/initializePreviewFrame':
            console.info(`[resource worker] initializePreviewFrame: frameId = ${data.frameId}`)
            await cacheInitialPreview(data.appPath, data.frameId, data.isPrerender, ports[0])
            break;

        case '/resourceWorker/updateResource':
            console.info('[resource worker] received updateResource request')
            await updateResource(data.path, data.content, data.mimeType, ports[0])
            break;
    }
}

const resourceFetch = request => request.method != 'GET' ? fetch(request) : cached(request)
self.addEventListener('fetch', event => event.respondWith(resourceFetch(event.request)))
