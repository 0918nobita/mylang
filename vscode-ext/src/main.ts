import path from "path";
import vscode, { ExtensionContext } from "vscode";
import {
    LanguageClient,
    ServerOptions,
    LanguageClientOptions,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export const activate = (context: ExtensionContext) => {
    const serverOptions: ServerOptions = {
        command: path.join(
            context.extensionPath,
            "../target/debug/language_server"
        ),
    };
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: "file", language: "mylang" }],
    };
    try {
        client = new LanguageClient(
            "mylang-mode",
            serverOptions,
            clientOptions
        );
        context.subscriptions.push(client.start());
    } catch (e) {
        vscode.window.showErrorMessage("Failed to start mylang-mode");
    }
};

export const deactive = () => client?.stop();
