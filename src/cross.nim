# Copyright (c) 2018-2023, Firas Khalil Khana
# Distributed under the terms of the ISC License

import std/os

import constants

#
# Cross Functions
#

proc radula_behave_bootstrap_cross_ccache*() =
    putEnv(RADULA_ENVIRONMENT_CCACHE_CONFIGURATION,
        RADULA_PATH_RADULA_CLUSTERS / RADULA_DIRECTORY_GLAUCUS /
        RADULA_CERAS_CCACHE / RADULA_FILE_CCACHE_CONFIGURATION)
    putEnv(RADULA_ENVIRONMENT_CCACHE_DIRECTORY, getEnv(
        RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_CROSS) / RADULA_CERAS_CCACHE)
    putEnv(RADULA_ENVIRONMENT_PATH, getEnv(
        RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN) / RADULA_PATH_USR /
        RADULA_PATH_LIB / RADULA_CERAS_CCACHE & ':' & getEnv(RADULA_ENVIRONMENT_PATH))
    createDir(getEnv(RADULA_ENVIRONMENT_CCACHE_DIRECTORY))

proc radula_behave_bootstrap_cross_envenomate*() =
    echo "test"

proc radula_behave_bootstrap_cross_environment_directories*() =
    let path = getEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY) / RADULA_DIRECTORY_CROSS

    putEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_CROSS, path)

    putEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_CROSS_BUILDS, path / RADULA_DIRECTORY_BUILDS)
    putEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_CROSS_SOURCES, path / RADULA_DIRECTORY_SOURCES)

    # toolchain log file
    putEnv(RADULA_ENVIRONMENT_FILE_CROSS_LOG, getEnv(
        RADULA_ENVIRONMENT_DIRECTORY_LOGS) / RADULA_DIRECTORY_CROSS &
        CurDir & RADULA_DIRECTORY_LOGS)

proc radula_behave_bootstrap_cross_environment_pkg_config*() =
    putEnv(RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR, getEnv(
        RADULA_ENVIRONMENT_DIRECTORY_CROSS) / RADULA_PATH_PKG_CONFIG_LIBDIR_PATH)
    putEnv(RADULA_ENVIRONMENT_PKG_CONFIG_PATH, getEnv(RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR))
    putEnv(RADULA_ENVIRONMENT_PKG_CONFIG_SYSROOT_DIR, getEnv(
        RADULA_ENVIRONMENT_DIRECTORY_CROSS) / RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)

    # These environment variables are only `pkgconf` specific, but setting them
    # won't do any harm...
    putEnv(RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_INCLUDE_PATH, getEnv(
        RADULA_ENVIRONMENT_DIRECTORY_CROSS) / RADULA_PATH_PKG_CONFIG_SYSTEM_INCLUDE_PATH)
    putEnv(RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_LIBRARY_PATH, getEnv(
        RADULA_ENVIRONMENT_DIRECTORY_CROSS) / RADULA_PATH_PKG_CONFIG_SYSTEM_LIBRARY_PATH)
