## Upload

### Build
```powershell

# Install dependencies
pnpm install

# Build the project
pnpm build

# Move to extension directory
cd clients/vscode

# Install vsce to build extension package
npm install -g @vscode/vsce

# Package the extension
vsce package
```

### Upload
- Open [Visual Studio Marketplace](https://marketplace.visualstudio.com/).
- Login with your Azure DevOps account.
- Press `Publish Extensions`
- Create your publisher.
- Choose `New Extension` and upload your `.vsix` file.