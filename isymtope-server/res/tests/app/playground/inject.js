(function () {
    let _onmessage = window.onmessage;
    window.onmessage = function (msg) {
        if (msg && 'object' == typeof msg && msg._previewIframeundefined == true) {
            if (msg._newLocation) { location.href = msg._newLocation }
            else if (msg._setInnerHTML) { body.innerHTML = msg._setInnerHTML }
        } else {
            if (_onmessage) {  _onmessage.apply(window, [].slice.call(arguments)) }
        }
    }
}())

// (function () { let _onmessage = window.onmessage; window.onmessage = function (msg) { if (msg && 'object' == typeof msg && msg._previewIframeundefined == true) { if (msg._newLocation) { location.href = msg._newLocation } else if (msg._setInnerHTML) { body.innerHTML = msg._setInnerHTML } } else { if (_onmessage) { _onmessage.apply(window, [].slice.call(arguments)) } } }}())