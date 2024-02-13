# fbink-sys
Bare access to [FBInk](https://github.com/NiLuJe/FBInk) automatically made by bindgen for rust, because Plato backend is too heavy sometimes. Statically linked.

`example` is a binary program which shows some bare fbink function to get some idea how to use it.

This is a really bare none modular way done, someone needs to improve it in the future. Reasons below.

To use it:
- As for the most important things, you need musl toolchain, the inkbox one at path: `/home/build/inkbox/kernel/toolchain/armv7l-linux-musleabihf-cross/` to automatically compile FBInk
- Use build.sh in the `example` folder to compile

To keep in mind:
- FBInk is compiled with these arguments: `make -j8 static MINIMAL=1 BITMAP=1 OPENTYPE=1 IMAGE=1 FONTS=1 CROSS_COMPILE=armv7l-linux-musleabihf-`
