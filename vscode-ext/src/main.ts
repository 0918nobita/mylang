import path from 'path';
import vscode, { ExtensionContext } from 'vscode';
import {
    LanguageClient,
    ServerOptions,
    LanguageClientOptions,
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

export const activate = (context: ExtensionContext) => {
    const serverOptions: ServerOptions = {
        command: path.join(
            context.extensionPath,
            '../target/debug/mylang_lsp_server'
        ),
        options: {
            env: { RUST_LOG: 'info' },
        },
    };
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'mylang' }],
    };
    try {
        client = new LanguageClient(
            'mylang-mode',
            serverOptions,
            clientOptions
        );
        context.subscriptions.push(client.start());
    } catch (e) {
        void vscode.window.showErrorMessage('Failed to start mylang-mode');
    }
};

export const deactive = () => client?.stop();
