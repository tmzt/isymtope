const UPSTREAM_CACHE = 'playground-upstream-cache'
const CACHE = 'playground-preview-cache'

const PREVIEW_PATH = '/app/playground/preview-1bcx1/'
const PREVIEW_PART = 'preview-1bcx1/'

const UPSTREAM_PATH = '/app/playground/'

self.addEventListener('install', event => {
    console.log('% install')
})

self.addEventListener('activate', event => {
    console.log('% activate')
})

const cached = async request => {
    let upstream = await caches.open(UPSTREAM_CACHE)
    let cache = await caches.open(CACHE)

    let preview_match = await cache.match(request)
    if (preview_match) {
        return preview_match
    }

    let upstream_req

    // Strip preview prefix before making upstream request
    // const preview_regex = new RegExp('^' + PREVIEW_PATH)
    // let url = new URL(request.url)
    // if (preview_regex.test(url.pathname)) {
    //     let upstream_url = new URL(request.url)
    //     upstream_url.pathname = url.pathname.replace(preview_regex, UPSTREAM_PATH)
    //     upstream_req = new Request(upstream_url.href)
    // } else {
    //     upstream_req = request.clone()
    // }
    upstream_req = request.clone()

    // Disable upstream cache
    // let upstream_match = await upstream.match(upstream_req)
    // if (upstream_match) {
    //     return upstream_match
    // }

    let upstream_response = await fetch(upstream_req)
    upstream.put(upstream_req, upstream_response.clone())

    return upstream_response
}

async function cachePreviewObject(pathname, mimeType, content) {
    let cache = await caches.open(CACHE)
    let request = new Request(self.origin + PREVIEW_PATH)
    let response = new Response([content], { contentType: mimeType })
    cache.put(request, response)
}

self.onmessage = async ({data}) => {
    switch (data.topic) {
        case '/serviceWorker/cachePreviewObject':
            await cachePreviewObject('/app/playground/preview-1bcx1/', 'text/html', data.content)
            self.postMessage({ topic: '/mainWindow/cachePreviewObjectUpdated' })
            break;
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
