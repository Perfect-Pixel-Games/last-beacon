# Last Beacon

> **Build. Venture. Recover. Return.**

*Last Beacon* is a single-player extraction roguelite where you command an underground colony on the brink of extinction. Design modular utility robots, send them to the hostile surface in search of vital resources, and return safely before everything is lost.

Every expedition is a calculated risk. Every destroyed robot leaves a story behind. Every successful return brings humanity one step closer to rebuilding.

---

## The Premise

Centuries after the collapse of civilization, humanity survives deep beneath the surface inside a vast underground silo known as **The Beacon**.

The surface is toxic, unstable, and contested by rival scavengers. Human life cannot survive there for long, so autonomous robots are built to venture into the wasteland, recover resources, and return home.

You are not the robot.

You are the caretaker of the Beacon.

Your responsibility is simple:

> **Keep the Beacon alive.**

---

## Core Gameplay Loop

```text
Build Robot
      │
      ▼
Deploy to Surface
      │
      ▼
Explore & Scavenge
      │
      ▼
Push Your Luck?
      │
      ▼
Return to Teleporter
      │
      ▼
Extract Resources
      │
      ▼
Upgrade Beacon
      │
      ▼
Build Better Robots
      │
      └───────────────┐
                      ▼
                 Repeat
```

If a robot is destroyed, it remains where it fell. Deploy another robot to recover its remains, salvage valuable components, and complete unfinished expeditions.

---

## Key Features

### 🤖 Modular Robot Construction

Design robots from modular components inspired by construction games and engineering toys.

Every module has a purpose:

- Batteries
- Cargo Bays
- Mining Lasers
- Scanners
- Sensors
- Armour
- Hover Drives
- Wheels
- Tracks
- Thrusters

Robot design is a puzzle of balancing weight, power consumption, storage, durability, mobility, and cost. No two robots need to be alike.

### 📦 Tetris-Style Inventory

Resources occupy physical space. Every item has a shape. Players must rotate and organise loot within the robot's cargo grid to maximise every expedition.

Large, valuable items become meaningful decisions: do you leave behind supplies to fit a rare reactor core, or return home with a safer load?

### 🛰 Extraction Gameplay

Each expedition begins at a teleporter. Explore abandoned structures, industrial ruins, and forgotten facilities.

The further you travel:

- the greater the rewards
- the greater the danger
- the harder it becomes to make it home

Extraction is always a choice. Greed is usually punished.

### ♻ Robot Recovery

Failure creates the next mission. Destroyed robots remain on the surface.

Recover them to reclaim:

- modules
- cargo
- rare upgrades
- valuable resources

A failed expedition becomes tomorrow's objective.

### 🏗 Grow the Beacon

The Beacon is the heart of the game. Permanent progression comes from expanding and upgrading your underground silo.

Possible upgrades include:

- Robot Assembly Bays
- Research Labs
- Power Generation
- Storage Warehouses
- Manufacturing Facilities
- Radar Arrays
- Drone Hangars
- Teleporter Improvements

The base is fully explorable and visually evolves as the colony grows.

### 🚇 A Living Underground Silo

Inspired by massive vertical industrial structures, the Beacon is built around a giant central shaft.

The player explores the base from a camera that travels along the central axis, looking outward toward the surrounding levels.

As the colony expands:

- new floors appear
- machinery fills empty spaces
- lights illuminate new districts
- robots move resources between facilities

The Beacon should feel alive.

### 🌍 The Surface

The world above is quiet, lonely, and dangerous.

Expect environments such as:

- Rusting factories
- Collapsed highways
- Solar farms
- Communication towers
- Mining facilities
- Abandoned laboratories
- Industrial scrapyards

Exploration rewards curiosity rather than constant combat.

---

## Design Pillars

### Every Expedition Matters

Each trip should feel meaningful. Every decision carries risk.

### Robots Are Disposable

The Beacon is permanent. Robots are tools. Some become legends. Others never return.

### Build, Don't Grind

Progress comes from intelligent planning, clever engineering, and successful expeditions rather than repetitive farming.

### Physical Systems

Everything exists physically.

- Robot modules occupy space.
- Inventory occupies space.
- Buildings occupy space.
- The Beacon grows physically over time.

Players should be able to see their progress.

### Hope, Not Despair

*Last Beacon* is not a story about surviving the apocalypse. It's about rebuilding after it.

Every successful mission makes the future a little brighter.

---

## Visual Direction

The visual style combines:

- Stylised industrial science fiction
- Weathered machinery
- Functional engineering
- Warm lighting inside the Beacon
- Cold, desolate landscapes above ground

Inspirations include:

- The hopeful industrial charm of *WALL·E*
- The vertical architecture of *Silo*
- The extraction tension of *Pacific Drive*
- Modular engineering and creative problem-solving games

The tone should be optimistic despite the harsh world.

---

## Current Vision

The goal is to create a game where players become attached not to a single character, but to the place they are protecting.

The Beacon becomes home. Every robot is built for a purpose. Every expedition is a gamble. Every return is a victory.

> **Protect the Beacon. Rebuild the future.**

---

## Project Layout

```text
last-beacon/
  engine/   # Foundation engine submodule, checkout, junction, or symlink
  game/     # Last Beacon Cargo package, source, manifest, and game assets
  scripts/  # Game-facing build, run, package, and validation commands
```

Game-owned assets live under `game/assets`. Foundation-owned assets may live under `engine/assets` and are packaged separately under `assets/engine` when present.

## Setup

Clone with submodules to get the default Foundation engine association:

```cmd
git clone --recurse-submodules https://github.com/Perfect-Pixel-Games/last-beacon.git
```

If the repository was cloned without submodules, initialize the engine later:

```cmd
git submodule update --init --recursive
```

### Alternate Engine Path

By default, scripts use `engine/`. To use another Foundation checkout, set `FOUNDATION_ENGINE_PATH` before running scripts:

```cmd
set FOUNDATION_ENGINE_PATH=E:\GameDev\Foundation
scripts\build.cmd --configuration test --target game
```

The alternate engine path must contain `scripts\foundation-build.cmd`.

## Build, Run, Package, and Validate

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

## Branch Workflow

This repository uses long-lived `dev` and `main` branches.

- Work happens on `feature/*` or `hotfix/*` branches.
- Pull requests into `dev` are normal feature integration.
- Pull requests into `main` should come only from `dev` or `hotfix/*`.
- GitHub branch protection should require the Last Beacon build checks before merging.
