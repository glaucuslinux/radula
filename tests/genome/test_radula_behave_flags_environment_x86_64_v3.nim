# Copyright (c) 2018-2023, Firas Khalil Khana
# Distributed under the terms of the ISC License

import std/os

import
  ../../src/constants,
  ../../src/genome

radula_behave_genome_environment("x86-64")

radula_behave_flags_environment()

echo "CFLAGS    :: ", getEnv(RADULA_ENVIRONMENT_FLAGS_C_COMPILER)
echo "CXXFLAGS  :: ", getEnv(RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER)
echo "LDFLAGS   :: ", getEnv(RADULA_ENVIRONMENT_FLAGS_LINKER)

doAssert getEnv(RADULA_ENVIRONMENT_FLAGS_C_COMPILER) == "-pipe -Werror=format-security -Wformat=2 -g0 -O2 -fdevirtualize-at-ltrans -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -flto=auto -flto-compression-level=19 -fuse-linker-plugin -fstack-protector-strong -fstack-clash-protection -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mtune=generic -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2"
doAssert getEnv(RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER) == "-pipe -Werror=format-security -Wformat=2 -g0 -O2 -fdevirtualize-at-ltrans -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -flto=auto -flto-compression-level=19 -fuse-linker-plugin -fstack-protector-strong -fstack-clash-protection -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mtune=generic -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2 -fno-rtti -fvisibility-inlines-hidden -fvisibility=hidden"
doAssert getEnv(RADULA_ENVIRONMENT_FLAGS_LINKER) == "-Wl,-O1 -Wl,-s -Wl,-z,noexecstack,-z,now,-z,relro -Wl,--as-needed -Wl,--gc-sections -Wl,--sort-common -Wl,--hash-style=gnu -Wl,-z,x86-64-v3 -pipe -Werror=format-security -Wformat=2 -g0 -O2 -fdevirtualize-at-ltrans -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -flto=auto -flto-compression-level=19 -fuse-linker-plugin -fstack-protector-strong -fstack-clash-protection -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mtune=generic -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2"
