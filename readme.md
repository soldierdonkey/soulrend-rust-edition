# Soulrend Rust Edition
## What is Soulrend?
This project is a 2d remaster of the Minecraft Modpack "Soulrend - Dark Fantasy RPG Soulslike."
Soulrend, per its name, is a Dark Fantasy RPG Soulslike.
Dark fantasy refers to its aesthetics. It relies on medieval weapony and magic. No guns, no cars. RPG refers to its progression style. There is minimal negative or backwards progression, and there is a strong emphasis on skill trees. Soulslike refers to the combat style. Enemies are complex, and so are your attacks. Each move must be carefully calculated and frivolous attacks/keyboard mashing will be punished.
## What is Soulrend Rust Edition?
Soulrend Rust Edition (SRRE) is a two dimensional remake of Soulrend. It is written in rust and adapts the RPG style of Soulrend into a rougelite-style progression. This means while some progress is saved each time you die, you must redo large portions of the game. This integrates in the soulslike philosophy by making combat even more punishing. Gone are the days of "corpse runs," where players would easily be able to recover from their mistakes. Spiritually, this is still a "minecraft" modpack.
## Basic Features of Soulrend Rust Edition.
Soulrend Rust Edition is a side-view game that relies on Terraria-esque pixels on a non-destructible map. There is a main screen that contains many other buttons. This enters into a world management system, which in turn opens to the actual game. The actual game is pausable at any time. Aditionally, there is an inventory system. Players have 9 hotbar slots and 18 inventory slots. Multiplayer will likely never be supported.
## Detailed Features of Soulrend Rust Edition
### Graphics
I think Macroquad uses OpenGL. SRRE relies on a 1.0 = 1 block coordinate system. The scene is drawn on a logical width/height basis. Eventually, the graphics/coordinates stuff wil be data driven. The camera is centered on an arbitrary set of coordinates, not necesarily centered around the player.
### Physics
SRRE uses a Kenimatic AABB (I think this acronym means axis-aligned bounding box) physics. Entities do not interact with entities when it comes to physics, and solids do not interact with solids. Eventually, fluids will be implememnted, although they will be completely static.
### World Generation
The SRRE world system goes Installation > Instance > Dimension (also called world)  > Scene > Column > Row > Tile. Only one scene will be loaded at once. Currently, a noise generation method will is used, but a more advanced structure-based system will be implemented in the near future.
## To-be implemented features of Soulrend Rust Edition
### Items
Items will be nearly completely data-driven. Similar to the Origins Mod, items will be defined by data and nearly data alone. Scripting/special features will be implemented through conditionals embedded into JSON files.
### Enemies
Enemies will have custom move sets, similar to how they function in soulslike games. This will be implemented much later.
## Development disclaimer.
This project (for now) uses textures from better faithful 32x. These textures are for development purposes only. Their (amazing) work can be found [here.](https://www.curseforge.com/minecraft/texture-packs/faithful-32x)

This programs relies on Rust and Macroquad:
 - https://rust-lang.org
 - https://docs.rs/macroquad/latest/macroquad/