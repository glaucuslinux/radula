switch("d", "danger")
switch("d", "lto")
switch("d", "release")
switch("d", "useMalloc")
switch("d", "strip")
switch("threads", "on")
switch("opt", "speed")
switch("os", "linux")
switch("cpu", "amd64")
switch("t", "-pipe -Werror=format-security -Wformat=2 -g0 -O2 -fdevirtualize-at-ltrans -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -flto=auto -flto-compression-level=19 -fuse-linker-plugin -fstack-protector-strong -fstack-clash-protection -Wp,-D_FORTIFY_SOURCE=3 -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt")
switch("l", "-Wl,-O1 -Wl,-s -Wl,-z,defs,-z,noexecstack,-z,now,-z,relro -Wl,--as-needed -Wl,--gc-sections -Wl,-pie -Wl,--sort-common -Wl,--hash-style=gnu -Wl,-z,x86-64-v3 -pipe -Werror=format-security -Wformat=2 -g0 -O2 -fdevirtualize-at-ltrans -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -flto=auto -flto-compression-level=19 -fuse-linker-plugin -fstack-protector-strong -fstack-clash-protection -Wp,-D_FORTIFY_SOURCE=3 -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt")
switch("mm", "orc")
