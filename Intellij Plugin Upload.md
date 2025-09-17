## Upload

### Build plugin

```powershell

# Build dependencies
pnpm install
pnpm build

# Move to plugin directory
cd clients/intellij

# Build plugin
gradlew.bat build
```

### Upload plugin
- Go to [Jetbrains Plugin Marketplace](https://plugins.jetbrain.com).
- Login with your JetBrains account.
- Press your account name and click "Upload Plugin".
- Fill out the form depends on your vendor and plugin details.
- Select the `.jar` file from the `clients/intellij/build/distributions` directory.
- Upload the plugin.

### Validation and Approval.
- The plugin will be reviewed in 2 days.
- In this time, check "Compatibility verification" for plugin validation.
- If the plugin status is "Problem", make nessary changes and upload an update.