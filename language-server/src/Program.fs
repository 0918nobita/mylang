module MyLang

open Fable.Import.VSCode.Vscode

let activate (_context: ExtensionContext) =
    let outChannel = window.createOutputChannel "MyLang"
    outChannel.appendLine "Hello from Fable"
