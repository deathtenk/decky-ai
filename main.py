import os
import sys

# The decky plugin module is located at decky-loader/plugin
# For easy intellisense checkout the decky-loader code one directory up
# or add the `decky-loader/plugin` path to `python.analysis.extraPaths` in `.vscode/settings.json`
import decky_plugin
import subprocess
import pathlib

PLUGIN_DIR = str(pathlib.Path(__file__).parent.resolve())
PLUGIN_BIN_DIR = PLUGIN_DIR + "/bin"
PLUGIN_PY_MODULES_DIR = PLUGIN_DIR + "/py_deps"
GET_APP_URL = 'https://api.steampowered.com/ICommunityService/GetApps/v1/'
ACCESS_TOKEN = 'ed381a462f724f0434eced9ec21f5a9f'

sys.path.append(PLUGIN_PY_MODULES_DIR)
import requests

class Plugin:

    # Function that takes data from application and feeds it to chatGPT, returns map of arguments
    async def ask_gpt(self, gameTitle, question):
        args = [os.path.join(PLUGIN_BIN_DIR,"aichat"),
                os.path.join(decky_plugin.DECKY_PLUGIN_SETTINGS_DIR, "config.json"),
                "\"" + question + "\""]
        if (gameTitle != ""): {
            args.extend(["-g", gameTitle])
        }
        result = subprocess.run(args, cwd=PLUGIN_BIN_DIR, capture_output=True, text=True)
        return {"text" : result.stdout}

    # Function that returns running app ID, exits with status code 1 if nothing is found
    async def get_running_app_data(self):
        args = [os.path.join(PLUGIN_BIN_DIR,"runningapp.sh")]
        result = subprocess.run(args, cwd=PLUGIN_BIN_DIR, capture_output=True, text=True)
        id = result.stdout
        params = {
            'access_token': ACCESS_TOKEN,
            'appids[0]': id
        }
        response = requests.get(GET_APP_URL, params=params)
        if response.status_code == 200:
            data = response.json()
            decky_plugin.logger.info("success! name:")
            game_title = data['response']['apps'][0]['name']
            return {"gameTitle" : game_title, "steamid" : id}
        else:
            decky_plugin.logger.info('Request failed with status code', response.status_code)
        return id

    # Asyncio-compatible long-running code, executed in a task when the plugin is loaded
    async def _main(self):
        decky_plugin.logger.info("Hello World from Main!")

    # Function called first during the unload process, utilize this to handle your plugin being removed
    async def _unload(self):
        decky_plugin.logger.info("Goodbye World!")
        pass

    # Migrations that should be performed before entering `_main()`.
    async def _migration(self):
        decky_plugin.logger.info("Migrating")
        # Here's a migration example for logs:
        # - `~/.config/decky-template/template.log` will be migrated to `decky_plugin.DECKY_PLUGIN_LOG_DIR/template.log`
        decky_plugin.migrate_logs(os.path.join(decky_plugin.DECKY_USER_HOME,
                                               ".config", "decky-template", "template.log"))
        # Here's a migration example for settings:
        # - `~/homebrew/settings/template.json` is migrated to `decky_plugin.DECKY_PLUGIN_SETTINGS_DIR/template.json`
        # - `~/.config/decky-template/` all files and directories under this root are migrated to `decky_plugin.DECKY_PLUGIN_SETTINGS_DIR/`
        decky_plugin.migrate_settings(
            os.path.join(decky_plugin.DECKY_HOME, "settings", "template.json"),
            os.path.join(decky_plugin.DECKY_USER_HOME, ".config", "decky-template"))
        # Here's a migration example for runtime data:
        # - `~/homebrew/template/` all files and directories under this root are migrated to `decky_plugin.DECKY_PLUGIN_RUNTIME_DIR/`
        # - `~/.local/share/decky-template/` all files and directories under this root are migrated to `decky_plugin.DECKY_PLUGIN_RUNTIME_DIR/`
        decky_plugin.migrate_runtime(
            os.path.join(decky_plugin.DECKY_HOME, "template"),
            os.path.join(decky_plugin.DECKY_USER_HOME, ".local", "share", "decky-template"))
