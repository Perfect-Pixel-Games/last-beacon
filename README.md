# LastBeacon

LastBeacon is the reference external game project for the Foundation engine.

## Layout

```text
last-beacon/
  engine/   # Foundation engine submodule, checkout, junction, or symlink
  game/     # LastBeacon Cargo package, source, manifest, and game assets
  scripts/  # Game-facing build, run, package, and validation commands
```

Game-owned assets live under `game/assets`. Foundation-owned assets may live under `engine/assets` and are packaged separately under `assets/engine` when present.

## Clone

Clone with submodules to get the default Foundation engine association:

```cmd
git clone --recurse-submodules https://github.com/Perfect-Pixel-Games/last-beacon.git
```

If the repository was cloned without submodules, initialize the engine later:

```cmd
git submodule update --init --recursive
```

## Alternate engine path

By default, scripts use `engine/`. To use another Foundation checkout, set `FOUNDATION_ENGINE_PATH` before running scripts:

```cmd
set FOUNDATION_ENGINE_PATH=E:\GameDev\Foundation
scripts\build.cmd --configuration test --target game
```

The alternate engine path must contain `scripts\foundation-build.cmd`.

## Commands

Build the game:

```cmd
scripts\build.cmd --platform windows-x64 --configuration test --target game
```

Run the game:

```cmd
scripts\run.cmd --platform windows-x64 --configuration test --target game
```

Package the game:

```cmd
scripts\package.cmd --platform windows-x64 --configuration shipping --target game
```

Validate the game crate:

```cmd
scripts\validate.cmd
```

## Branch workflow

This repository uses long-lived `dev` and `main` branches.

- Work happens on `feature/*` or `hotfix/*` branches.
- Pull requests into `dev` are normal feature integration.
- Pull requests into `main` should come only from `dev` or `hotfix/*`.
- GitHub branch protection should require the LastBeacon Build checks before merging.
