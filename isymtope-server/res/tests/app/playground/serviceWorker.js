const UPSTREAM_CACHE = 'playground-upstream-cache'
const RESOURCE_CACHE = 'playground-resource-cache'
const CACHE = 'playground-preview-cache'

const PREVIEW_PATH = '/app/playground/preview-1bcx1/'
const PREVIEW_PART = 'preview-1bcx1/'

const UPSTREAM_PATH = '/app/playground/'

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

const PREVIEW_PATH_REGEX = /^\/app\/playground\/preview-1bcx1\//;

const cached = async request => {
    let upstream = await caches.open(UPSTREAM_CACHE)
    let resources = await caches.open(RESOURCE_CACHE)
    let cache = await caches.open(CACHE)

    let preview_match = await cache.match(request)
    if (preview_match) {
        return preview_match
    }

    let original_url = new URL(request.url);
    let original_url_path = original_url.pathname

    if (original_url.origin === self.origin) {
        if (PREVIEW_PATH_REGEX.test(original_url_path)) {
            let resource_path = original_url_path.replace(PREVIEW_PATH_REGEX, '')
            let resource_request_path = '/resources/app/playground/' + resource_path
            let resource_req = new Request(self.origin + resource_request_path);
            let resource_match = await resources.match(resource_req)
            if (resource_match) { return resource_match }

            let resource_resp = await fetch(resource_req)
            await resources.put(resource_req, resource_resp)
        }
    }

    let upstream_req = request.clone()

    // Disable upstream cache
    // let upstream_match = await upstream.match(upstream_req)
    // if (upstream_match) {
    //     return upstream_match
    // }

    let upstream_response = await fetch(upstream_req)
    await upstream.put(upstream_req, upstream_response.clone())

    return upstream_response
}

async function cachePreviewObject(pathname, mimeType, content) {
    let cache = await caches.open(CACHE)
    let request = new Request(self.origin + PREVIEW_PATH)
    let response = new Response([content], { contentType: mimeType })
    response.headers.set('content-type', mimeType)
    await cache.put(request, response)
}

self.onmessage = async ({data, ports}) => {
    switch (data.topic) {
        case '/resourceWorker/compilationStarted':
            console.info('[resource worker] received compilationStarted, awaiting message from compilerWorker')
            let compilerWorkerPort = ports[0]
            let mainWindowPort = ports[1]

            compilerWorkerPort.onmessage = async ({data}) => {
                switch (data.topic) {
                    case '/resourceWorker/compilationComplete':
                        let { content, pathname, mimeType } = data
                        let fullPathname = PREVIEW_PATH + pathname
                        await cachePreviewObject(fullPathname, mimeType, content )
                        mainWindowPort.postMessage({ topic: '/mainWindow/refreshPreview', pathname })
                        break;
                }
            }

            break;

        // case '/serviceWorker/cachePreviewObject':
        //     let { content, mimeType } = data
        //     let mainWindowPort = ports[0]
        //     let compilerWorkerPort = ports[1]

        //     compilerWorkerPort

        //     await cachePreviewObject('/app/playground/preview-1bcx1/', 'text/html', data.content)
        //     ports[0].postMessage({ topic: '/mainWindow/cachePreviewObjectUpdated' })
        //     break;
    }
}

const resourceFetch = request => request.method != 'GET' ? fetch(request) : cached(request)

// const resourceFetch = request => request.method != 'GET' ?
//     fetch(request):
//     caches.open(CACHE)
//         .then(async cache => {
//             let resp = await cache.match(request)
//             if (!resp) {
//                 resp = await fetch(request)
//                 cache.put(request, resp.clone())

//                 return resp
//             }

//             console.log('> from cache', request.url)
//             return resp
//         })

self.addEventListener('fetch', event => event.respondWith(resourceFetch(event.request)))
