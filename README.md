# Momo

Momo is a open source, desktop, music player.

<!-- TODO: Add screenshots and showcase features once we have them.  -->

## Quickstart

### Install the Coding Profile

1. Press `Ctrl + Shift + P`
2. Type `Import Profile`
3. Select `Profiles: Import Profile...`
4. Select `Select File...`
5. Navigate to the project and select `momo.code-profile` that's in the `.vscode` folder.
6. Select `Create Profile`

The code profile will then install with all the proper extensions and configurations set. We use code profiles so that everyone has a relatively similar coding experience. You're free to install any other extensions after installing the code profile.

### Prerequisites

Go to [tauri.app](https://tauri.app/v1/guides/getting-started/prerequisites) and go through the prerequisites process based on your operating system.

### Installing & Running

Now, open a VSCode terminal and run:

```sh
npm install
```

and then if you're trying to do fast development then run:

```sh
npm run dev
```

This will allow you to go to [localhost:1420](http://localhost:1420) and develope via the browser.

If you want to get a full desktop dev environment then run:

```sh
npm run tauri dev
```

This will open the application in it's own window.

_Also, if you right click in this window you can inspect it as if you were in a normal web browser._

If you want to build the application, follow the build instructions for your operating system on [tauri.app](https://tauri.app/v1/guides/building/).
