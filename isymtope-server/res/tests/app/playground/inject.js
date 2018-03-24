window.addEventListener('message', function(msg) {
    msg = JSON.parse(msg || '{}')
    switch(msg.type) {
        case 'replaceHtml':
            body.innerHTML = msg.body
    }
})