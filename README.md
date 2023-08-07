# chip_8

This is a simple CHIP-8 interpreter / emulator project that I'm making to try to help me learn the Rust language, so don't bully me **too** hard for the code.

More or less feature complete as an emulator, might add debug features in the future and / or clean up the codebase a bit.

Passed every test I could throw at it except the "quirks" test. I haven't implemented v-blank and I can't get it to pass the sprite wrapping / clipping test for the life of me, even though in game it looks fine. It's not really a big deal though.

Accepts ROMs via command line arguments. Ex. "chip_8 roms/foo.ch8". Haven't tested but I think you can just drag them onto the executable on Windows, as well.
