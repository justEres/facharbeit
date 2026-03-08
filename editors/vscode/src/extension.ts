import * as path from "path";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;
let outputChannel: vscode.OutputChannel | undefined;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  outputChannel = vscode.window.createOutputChannel("eres Language Server");
  context.subscriptions.push(outputChannel);
  outputChannel.appendLine("Activating eres extension");

  const serverOptions = await resolveServerOptions();
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "eres" }],
    outputChannel
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
    outputChannel?.appendLine(`Starting language server from configured path: ${configuredPath}`);
    return {
      command: configuredPath,
      args: []
    };
  }

  const candidates = collectSearchRoots();
  outputChannel?.appendLine(`Search roots: ${candidates.join(", ") || "(none)"}`);

  const cwd = await findCargoWorkspaceRoot(candidates);
  if (!cwd) {
    throw new Error(
      "Could not find Cargo.toml for `cargo run --bin eres-lsp`. Open the facharbeit repo root or set `eres.languageServer.path`."
    );
  }

  outputChannel?.appendLine(`Starting language server via cargo in: ${cwd}`);

  return {
    command: "cargo",
    args: ["run", "--quiet", "--bin", "eres-lsp"],
    options: { cwd }
  };
}

function collectSearchRoots(): string[] {
  const roots = new Set<string>();

  for (const folder of vscode.workspace.workspaceFolders ?? []) {
    roots.add(folder.uri.fsPath);
  }

  const activePath = vscode.window.activeTextEditor?.document.uri.fsPath;
  if (activePath) {
    roots.add(path.dirname(activePath));
  }

  return [...roots];
}

async function findCargoWorkspaceRoot(candidates: string[]): Promise<string | undefined> {
  for (const candidate of candidates) {
    let current = candidate;
    while (true) {
      const cargoToml = path.join(current, "Cargo.toml");
      if (await pathExists(cargoToml)) {
        return current;
      }

      const parent = path.dirname(current);
      if (parent === current) {
        break;
      }
      current = parent;
    }
  }

  return undefined;
}

async function pathExists(filePath: string): Promise<boolean> {
  try {
    await vscode.workspace.fs.stat(vscode.Uri.file(filePath));
    return true;
  } catch {
    return false;
  }
}
