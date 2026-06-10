# storm

> Cold rain over a pine forest, with periodic lightning and wandering wildlife.

A landscape scene: pine trees on a mountainside, rain falling across the whole frame, periodic lightning that briefly illuminates the scene, and occasional animals (a bird in a tree, deer, bear, or bigfoot) walking by.

## Visual elements

- **Rain**. Cold, dense, vertical streaks drawn with the system accent-cool palette.
- **Mountains + pines**. A static silhouette layer.
- **Lightning**. Periodic bright flashes that briefly light up the whole scene (and a low rumble sound on Windows).
- **Wildlife**. A bird sits in a tree. From time to time, a deer, bear, or bigfoot walks across the lower portion of the frame.
- **Live logo**. A dim version of your OS name + kernel, drawn behind the rain, lit by lightning.

## Dynamic / live behavior

- **Live logo**. Pulled from `get_system_info()`.
- **System load reactions**. Higher CPU/memory pressure increases rain density, lightning frequency, and the chance of a wildlife sighting. Storms get angrier under load.
- **Per-machine personality**. `host_bias` slightly shifts lightning timing per computer.
- **Accent colors**. Rain + lightning use your system accent and accent-hot.

## Configuration (registry)

Under `HKEY_CURRENT_USER\Software\local76\storm`:

- `RainDensity`: 0 = sparse, 1 = normal, 2 = heavy.
- `Wildlife`: 0 = off, 1 = on (default).
- `Lightning`: 0 = off, 1 = on (default).

Global options (`ColorTheme`, `GlobalScanlines`) apply.

## Notes

- One of the few scenes with sound (lightning rumble on Windows).
- The pine forest is hand-drawn once at the start; it does not move. Only the rain, lightning, and wildlife animate.
- The bigfoot sighting is intentional, not a bug.

Part of the [screensavers](https://github.com/local76/screensavers) collection. See the root README for installation.
