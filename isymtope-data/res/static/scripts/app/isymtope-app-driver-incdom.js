
const IsymtopeAppDriver = exports.IsymtopeAppDriver

///
/// For compatibility with IncDom's empty prototype objects.
///

function Blank() {}
Blank.prototype = Object.create(null);

function markExisting(nodes) {
    nodes = 'string' === typeof nodes ? document.querySelectorAll(nodes) : nodes
    Array.prototype.slice.call(nodes).forEach(function(node) {
        IncrementalDOM.importNode(node);
        var data = node['__incrementalDOMData'];
        data.staticsApplied = true;
        data.newAttrs = new Blank();
    });
}

class IsymtopeAppDriverIncDom extends IsymtopeAppDriver {
    constructor() {
        super()
        this._rootDiv = null
    }

    bindRootDiv(rootDiv) {
        this._rootDiv = rootDiv
        markExisting(rootDiv.querySelectorAll('*'))
    }

    update(render, store) {
        IncrementalDOM.patch(this._rootDiv, render);        
    }
}

exports.IsymtopeAppDriverIncDom = IsymtopeAppDriverIncDom
