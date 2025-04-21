# VSC Language Support (VecScoreDrawer)

## 構成
- `vsc_language/` … VSCode拡張本体（LSPクライアント・シンタックスハイライト・バイナリ同梱）
- `vsc_lsp/` … Rust製LSPサーバー（開発・ビルド用）

## ビルド手順（Windows例）

1. Rust LSPサーバーのビルド
   ```
   cd ../vsc_lsp
   cargo build --release
   ```

2. バイナリのコピー
   ```
   copy target\release\vsc_lsp.exe ..\vsc_language\bin\vsc_lsp.exe
   ```

3. 拡張機能のTypeScriptビルド
   ```
   cd ..\vsc_language
   npm install
   npm run compile
   ```

4. VSIXパッケージ作成
   ```
   vsce package
   ```

5. VSCodeでVSIXをインストールし、.vscファイルで「コメント色分け」と「LSP機能」が両立することを確認

## 注意
- `vsc_language/package.json`の`files`に`bin/vsc_lsp.exe`、`syntaxes/**`、`language-configuration.json`が含まれていることを確認
- F5デバッグ時も同様に`bin/vsc_lsp.exe`が存在すること
