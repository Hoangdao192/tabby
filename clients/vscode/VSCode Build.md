## Build

To get started, use the following commands:

```powershell

# Install dependencies
pnpm install

# Build project
pnpm build

# Change directory to VSCode extension
cd ./clients/vscode

# Install VSCE to create extension file
npm install -g @vscode/vsce

# Build VSIX file from the extension
vsce package

# Install the package file from the VSIX file by pressing Ctrl + Shift + P inside VSCode and choose Extensions: Install from VSIX...
```

## Architecture

On a high level the extension is divided into the following components:

                            +---------------------+
                            | MSB CodeGen Server  |
    +------------------+    | +-----------------+ |
    | VSCode Extension | ---->|                 | |
    +------------------+    | |     Chat UI     | |
             |              | |                 | |
             |              | +-----------------+ |
             |              |                     |
             v              | +-----------------+ |
    +------------------+    | |                 | |
    | MSB CodeGen Agent| ---->|       API       | |
    +------------------+    | |                 | |
                            | +-----------------+ |
                            +---------------------+

- **MSB CodeGen Server**: The server component of MSB CodeGen, responsible for managing user accounts, code completions, and chat functionality.
- **Chat UI**: The web-based UI for MSB CodeGen Chat, which is embedded as a webview in the VSCode extension. It is distributed together with the MSB CodeGen Server.
- **MSB CodeGen Agent**: The LSP server of MSB CodeGen, responsible for providing code completions and other language services to the VSCode extension. It communicates with the MSB CodeGen Server via the API. For VSCode, the MSB CodeGen Agent is a library, thus it is embedded in the extension.
