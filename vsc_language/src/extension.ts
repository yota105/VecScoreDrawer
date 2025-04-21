import * as vscode from 'vscode';
import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  console.log('VSC言語サポート拡張機能が有効化されました');

  // サーバーの実行ファイルパス
  const serverExe = path.join(context.extensionPath, 'bin', 'vsc_lsp.exe');
  const serverOptions: ServerOptions = {
    run:   { command: serverExe },
    debug: { command: serverExe }
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'vsc' }],
    outputChannel: vscode.window.createOutputChannel('VSC Language Server'),
  };

  client = new LanguageClient(
    'vscLanguageServer',
    'VSC Language Server',
    serverOptions,
    clientOptions
  );

  context.subscriptions.push(client);
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}