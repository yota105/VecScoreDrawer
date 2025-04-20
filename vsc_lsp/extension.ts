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
  const serverPath = context.asAbsolutePath(
    path.join('..', 'target', 'debug', 'vsc_lsp.exe')
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

  // client が Disposable を持つのでこちらを登録
  context.subscriptions.push(client);

  // サーバー起動は別呼び出し
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  return client ? client.stop() : undefined;
}