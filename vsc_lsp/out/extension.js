"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const path = require("path");
const vscode_1 = require("vscode");
const node_1 = require("vscode-languageclient/node");
let client;
function activate(context) {
    const serverPath = context.asAbsolutePath(path.join('..', 'target', 'debug', 'vsc_lsp.exe'));
    const serverOptions = {
        run: { command: serverPath, transport: node_1.TransportKind.stdio },
        debug: { command: serverPath, transport: node_1.TransportKind.stdio }
    };
    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'vecscore' }],
        synchronize: {
            fileEvents: vscode_1.workspace.createFileSystemWatcher('**/*.vsc')
        }
    };
    client = new node_1.LanguageClient('vecscoreLsp', 'VecScore LSP', serverOptions, clientOptions);
    // client が Disposable を持つのでこちらを登録
    context.subscriptions.push(client);
    // サーバー起動は別呼び出し
    client.start();
}
function deactivate() {
    return client ? client.stop() : undefined;
}
//# sourceMappingURL=extension.js.map