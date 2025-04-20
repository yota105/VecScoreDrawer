import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  // 修正: 拡張機能のルートからの相対パスで target を指定
  const serverPath = context.asAbsolutePath(
    path.join('target', 'debug', 'vsc_lsp.exe')
  );

  const serverOptions: ServerOptions = {
    run:   { command: serverPath, transport: TransportKind.stdio },
    debug: { command: serverPath, transport: TransportKind.stdio }
  };
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'vecscore' }],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher('**/*.vsc')
    }
  };

  client = new LanguageClient(
    'vecscoreLsp',
    'VecScore LSP',
    serverOptions,
    clientOptions
  );

  context.subscriptions.push(client);
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  return client ? client.stop() : undefined;
}