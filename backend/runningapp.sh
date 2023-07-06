#!/bin/bash

ps aux | grep -o 'SteamLaunch AppId=\([0-9]\+\)' | grep -o '\([0-9]\+\)'