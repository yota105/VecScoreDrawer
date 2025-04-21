"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.deactivate = exports.activate = void 0;
const vscode = require("vscode");
const path = require("path");
const node_1 = require("vscode-languageclient/node");
let client;
function activate(context) {
    console.log('VSC言語サポート拡張機能が有効化されました');
    // サーバーの実行ファイルパス
    const serverExe = path.join(context.extensionPath, 'bin', 'vsc_lsp.exe');
    const serverOptions = {
        run: { command: serverExe },
        debug: { command: serverExe }
    };
    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'vsc' }],
        outputChannel: vscode.window.createOutputChannel('VSC Language Server'),
    };
    client = new node_1.LanguageClient('vscLanguageServer', 'VSC Language Server', serverOptions, clientOptions);
    context.subscriptions.push(client);
    client.start();
}
exports.activate = activate;
function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
exports.deactivate = deactivate;
//# sourceMappingURL=extension.js.map