./## Build

To get started, use the following commands:

```powershell

# Move to the plugin source folder
cd ./clients/intellij

#Build all artifact
gradlew.bat build

# Open Intellij Idea and install the plugin
# ⚙️ -> Plugins -> ⚙️ -> Install plugin from disk...
# Choose the file at ./build/distributions/intellij-msb-codegen.zip
```

## Architecture

On a high level the extension is divided into the following components:

                            +---------------------+
                            |     Tabby Server    |
    +------------------+    | +-----------------+ |
    | VSCode Extension | ---->|                 | |
    +------------------+    | |     Chat UI     | |
             |              | |                 | |
             |              | +-----------------+ |
             |              |                     |
             v              | +-----------------+ |
    +------------------+    | |                 | |
    |   Tabby Agent    | ---->|       API       | |
    +------------------+    | |                 | |
                            | +-----------------+ |
                            +---------------------+

- **Tabby Server**: The server component of Tabby, responsible for managing user accounts, code completions, and chat functionality.
- **Chat UI**: The web-based UI for Tabby Chat, which is embedded as a webview in the VSCode extension. It is distributed together with the Tabby Server.
- **Tabby Agent**: The LSP server of Tabby, responsible for providing code completions and other language services to the VSCode extension. It communicates with the Tabby Server via the API. For VSCode, the Tabby Agent is a library, thus it is embedded in the extension.
