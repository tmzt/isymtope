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