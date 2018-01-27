
function fetchContent(path) {
    let editor = window._editor
    if (editor) {
        window.fetch('/' + path).then(resp => {
            resp.text().then(body => {
                editor.setValue(body)
            })
        })
    }
}

function editorContentReducer(state, action) {
    switch(action.type) {
        case 'EDITORCONTENT.LOAD':
            fetchContent(action.name); return true;
        default: return null
    }
}

document.addEventListener('DOMContentLoaded', () => {
    require.config({ paths: { 'vs': 'https://unpkg.com/monaco-editor@0.8.3/min/vs' }});
    window.MonacoEnvironment = { getWorkerUrl: () => proxy };
    
    let proxy = URL.createObjectURL(new Blob([`
        self.MonacoEnvironment = {
            baseUrl: 'https://unpkg.com/monaco-editor@0.8.3/min/'
        };
        importScripts('https://unpkg.com/monaco-editor@0.8.3/min/vs/base/worker/workerMain.js');
    `], { type: 'text/javascript' }));
    
    let editorComponentDiv = document.getElementById('editorComponent')
    let editorDiv = document.createElement('div')
    editorDiv.setAttribute('id', 'editor')
    editorComponentDiv.appendChild(editorDiv)

    require(["vs/editor/editor.main"], function () {
        editor = monaco.editor.create(editorDiv, {
        	value: '',
        	theme: 'vs-dark'
        });
        
        editor.addListener('didType', () => {
        	console.log(editor.getValue());
        });

        window.fetch('/playground.ism').then(resp => {
            resp.text().then(body => {
                editor.setValue(body)
            })
        })

        window._editor = editor
    });
})