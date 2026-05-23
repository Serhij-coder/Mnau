# Project Overview — Mnau

**Mnau** is a 2D arcade game in Rust (Macroquad). Control a cat, collect fish for points, avoid cars.

## Gameplay

- **Objective**: Collect fish to earn points, survive as long as possible
- **Challenge**: Cars spawn from all sides and move across the screen; hitting one ends the game
- **Difficulty**: Spawn intervals and car speeds increase exponentially over time
- **Score**: Green fish +300, White +100, Skeleton -200
- **Retry**: Game over screen lets you restart or quit

## Tech

- **Language**: Rust (Edition 2024)
- **Framework**: Macroquad 0.4.14
- **Architecture**: Modular entity design (Cat, Fish, Car) with AABB collision
