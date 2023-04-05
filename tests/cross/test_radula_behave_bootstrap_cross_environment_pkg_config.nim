# Copyright (c) 2018-2023, Firas Khalil Khana
# Distributed under the terms of the ISC License

import os
import strutils

import ../../src/bootstrap
import ../../src/constants
import ../../src/cross

radula_behave_bootstrap_environment()

radula_behave_bootstrap_cross_environment_pkg_config()

echo "PKG_CONFIG_LIBDIR               :: ", getEnv(RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR)
echo "PKG_CONFIG_PATH                 :: ", getEnv(RADULA_ENVIRONMENT_PKG_CONFIG_PATH)
echo "PKG_CONFIG_SYSROOT_DIR          :: ", getEnv(RADULA_ENVIRONMENT_PKG_CONFIG_SYSROOT_DIR)
echo "PKG_CONFIG_SYSTEM_INCLUDE_PATH  :: ", getEnv(RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_INCLUDE_PATH)
echo "PKG_CONFIG_SYSTEM_LIBRARY_PATH  :: ", getEnv(RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_LIBRARY_PATH)

doAssert getEnv(RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR).endsWith("cross/usr/lib/pkgconfig")
doAssert getEnv(RADULA_ENVIRONMENT_PKG_CONFIG_PATH).endsWith("cross/usr/lib/pkgconfig")
doAssert getEnv(RADULA_ENVIRONMENT_PKG_CONFIG_SYSROOT_DIR).endsWith("cross/")
doAssert getEnv(RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_INCLUDE_PATH).endsWith("cross/usr/include")
doAssert getEnv(RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_LIBRARY_PATH).endsWith("cross/usr/lib")