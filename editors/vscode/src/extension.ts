import * as path from "path";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  const serverOptions = await resolveServerOptions();
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "eres" }]
  };

  client = new LanguageClient(
    "eresLanguageServer",
    "eres Language Server",
    serverOptions,
    clientOptions
  );

  context.subscriptions.push(client);
  await client.start();
}

export async function deactivate(): Promise<void> {
  if (client) {
    await client.stop();
    client = undefined;
  }
}

async function resolveServerOptions(): Promise<ServerOptions> {
  const configuredPath = vscode.workspace
    .getConfiguration("eres")
    .get<string>("languageServer.path", "")
    .trim();

  if (configuredPath.length > 0) {
    return {
      command: configuredPath,
      args: []
    };
  }

  const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
  if (!workspaceFolder) {
    throw new Error("eres extension requires an open workspace folder");
  }

  const cwd = workspaceFolder.uri.fsPath;
  const cargoToml = path.join(cwd, "Cargo.toml");
  try {
    await vscode.workspace.fs.stat(vscode.Uri.file(cargoToml));
  } catch {
    throw new Error("Could not find Cargo.toml in the workspace root for `cargo run --bin eres-lsp`");
  }

  return {
    command: "cargo",
    args: ["run", "--quiet", "--bin", "eres-lsp"],
    options: { cwd }
  };
}
