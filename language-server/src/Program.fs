module MyLang

open Fable.Import.VSCode.Vscode

let activate (context: ExtensionContext) =
    let outChannel = window.createOutputChannel "MyLang"
    outChannel.appendLine "Hello from Fable"
    outChannel.appendLine <| sprintf "Extension URI: %s" (context.extensionUri.toString())
