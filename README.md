# Rust NES emulator

See [justinmichaud.com](http://justinmichaud.com/smb_challenge/index.html) for a playable demo.

Games that don't use any fancy ppu trickery work, including Donkey Kong and Super Mario Bros. Only mapper 0 is supported. Sound is not supported.

![Super Mario Bros](/smb.gif?raw=true "Super Mario Bros")

# Super Mario Bros Hacks

If USE_HACKS in settings.rs is set to true, the title screen and prelevel screens will be automatically skipped, and you will have infinite lives.
If USE_HACKS and SPECIAL are set, the game is tweaked for a one-button challenge. You can jump, and you cannot stop. The game screen is warped for an extra challenge, but deaths are instant and you have infinite lives:

![Super Mario Bros - SPECIAL and USE_HACKS](/smb-special-usehacks.png?raw=true "Super Mario Bros SPECIAL and USE HACKS")

# Building for web
Download the emscripten portable sdk, and source the emsdk_env.sh script. Run `make` and `make serve`, and you should be good to go!