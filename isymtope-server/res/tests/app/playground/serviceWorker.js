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
    let script = `
(function __isymtope_inject_() {
    function processScripts(dest, src, add) {
        let keyedScripts = new Map();
        let scriptsToRestore = new Map();

        /* Get metadata for current scripts */
        [].forEach.call(dest.querySelectorAll('script'), script => {
            let key = script.getAttribute('data-isymtope-script-key');
            if (key) {
                let ts = script.getAttribute('data-isymtope-script-ts');
                keyedScripts.set(key, { ts: ts, script: script });
            }
        });

        /* Compare to new scripts */
        [].forEach.call(src.querySelectorAll('script'), scriptNode => {
            let key = scriptNode.getAttribute('data-isymtope-script-key');
            if (key) {
                let script = scriptNode.parentNode.removeChild(scriptNode);
                let ts = parseInt(script.getAttribute('data-isymtope-script-ts'));
                let old = keyedScripts.get(key);
                if (old == null || (old != null && !isNaN(ts) && ts > old.ts)) {
                    scriptsToRestore.set(key, { ts: ts, script: script });
                }
            }
        })

        dest.innerHTML = src.innerHTML

        /* Inject ourself */
        if (add && !scriptsToRestore.has('inject')) {
            let node = dest.ownerDocument.createElement('script');
            node.setAttribute('data-isymtope-script-key', 'inject')
            scriptsToRestore.set('inject', { ts: NaN, script: node })
        }

        /* Restore scripts */
        for (let meta in scriptsToRestore.entries()) {
            console.log('[inject] Restoring script with key ' + meta[0], meta[1])
            dest.appendChild(meta[1].script)
        }
    }

    let worker;
    let _onmessage = window.onmessage; window.onmessage = function (e) {
        let msg = ('string' == typeof e.data) ? JSON.parse(e.data) : e.data;
        let completion = e.ports.length && e.ports[0];
        msg = msg.__isymtopePlaygroundFrameMsg;
        switch (msg.type) {
            case 'setLocation': location.href = msg.location; break;
            case 'setInnerHTML': document.body.innerHTML = msg.content;
            case 'mergeDoc':
                console.log('[inject] Requested content update (mergeDoc)', msg);
                var doc = document.implementation.createHTMLDocument(""); doc.documentElement.innerHTML = msg.content;
                    processScripts(document.head, doc.head, true);
                    processScripts(document.body, doc.body, false);
                    Isymtope.app().update().then(() => completion.postMessage({}));
                    break;
            case 'registerWorker':
                console.log('[inject] Registering serviceWorker within iframe at origin ' + window.origin);
                navigator.serviceWorker.register(window.origin + '/serviceWorker.js')
                    .then(() => { completion.postMessage({}) })
                break;
            case 'forwardMessage':
                resourceWorker.postMessage(msg.msg, completion); break;
        }
    }
}())
`
    let tag = `<script data-isymtope-script-key="inject">${script}</script></head>`
    return content.replace(/<\/head>/, tag)
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
    let content = await upstreamResp.text()
    content = !content.match(/\(function __isymtope_inject_\( {/) ? injectScript(content, frameId) : content
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
