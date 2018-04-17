
const { CompilerService } = _exports

class WasmCompilerService extends CompilerService
{
    prepareService() {
        return getOrRegisterCompilerWorker()
    }

    async startCompilation(opts) {
        const compilerWorker = await getOrRegisterCompilerWorker()

        return new Promise((resolve, reject) => {
            let completion = new MessageChannel()
            let compileReq = {
                topic: '/compilerWorker/compile',
                source,
                pathname: '',
                mimeType: 'text/html',
                app_name,
                baseUrl,
                template_path,
                path
            }

            completion.onmessage = data => {
                if (data.error) {
                    resolve(data.content)
                } else {
                    reject(data.error)
                }
            }
        })
    }
}

let compiler
async function getOrRegisterCompilerWorker() {
    if (!compiler) {
        compiler = new Worker('/app/playground/worker.js')
    }
    return compiler
}
