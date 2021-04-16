// Copyright (c) 2018-2021, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command, Stdio};
use std::string::String;

use super::constants;

//
// Behave Functions
//

fn radula_behave_bootstrap_architecture_environment(x: &'static str) {
    env::set_var(
        constants::RADULA_ENVIRONMENT_TUPLE_BUILD,
        String::from_utf8_lossy(
            &Command::new(
                Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CERATA).unwrap())
                    .join(constants::RADULA_PATH_CONFIG_GUESS),
            )
            .output()
            .unwrap()
            .stdout,
        )
        .trim(),
    );

    env::set_var(constants::RADULA_ENVIRONMENT_ARCHITECTURE, x);
    env::set_var(
        constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA,
        [constants::RADULA_ARCHITECTURE_CERATA, x].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TUPLE_TARGET,
        [x, "-", constants::RADULA_ARCHITECTURE_TUPLE_TARGET].concat(),
    );

    match x {
        constants::RADULA_ARCHITECTURE_AARCH64 => {
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA,
                [
                    constants::RADULA_ARCHITECTURE_CERATA,
                    constants::RADULA_ARCHITECTURE_AARCH64_CERATA,
                ]
                .concat(),
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS,
                constants::RADULA_ARCHITECTURE_AARCH64_FLAGS,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_AARCH64_GCC_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX,
                constants::RADULA_ARCHITECTURE_AARCH64_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION,
                "",
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE,
                constants::RADULA_ARCHITECTURE_AARCH64_LINUX_IMAGE,
            );
            env::set_var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL, x);
        }
        constants::RADULA_ARCHITECTURE_ARMV6ZK => {
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_FLAGS,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_GCC_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_LINUX_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_LINUX_IMAGE,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_TUPLE_TARGET,
                [
                    x,
                    "-",
                    constants::RADULA_ARCHITECTURE_TUPLE_TARGET,
                    constants::RADULA_ARCHITECTURE_ARMV6ZK_TUPLE_TARGET,
                ]
                .concat(),
            );
        }
        constants::RADULA_ARCHITECTURE_I686 => {
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS,
                constants::RADULA_ARCHITECTURE_I686_FLAGS,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_I686_GCC_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX,
                constants::RADULA_ARCHITECTURE_I686_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_I686_LINUX_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE,
                constants::RADULA_ARCHITECTURE_I686_LINUX_IMAGE,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL,
                constants::RADULA_ARCHITECTURE_I686_LINUX,
            );
        }
        constants::RADULA_ARCHITECTURE_X86_64 => {
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS,
                constants::RADULA_ARCHITECTURE_X86_64_FLAGS,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_X86_64_GCC_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX,
                constants::RADULA_ARCHITECTURE_X86_64_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_X86_64_LINUX_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE,
                constants::RADULA_ARCHITECTURE_I686_LINUX_IMAGE,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL,
                constants::RADULA_ARCHITECTURE_X86_64_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_TUPLE_TARGET,
                [
                    constants::RADULA_ARCHITECTURE_X86_64_LINUX,
                    "-",
                    constants::RADULA_ARCHITECTURE_TUPLE_TARGET,
                ]
                .concat(),
            );
        }
        _ => {}
    }
}

fn radula_behave_bootstrap_clean() {
    radula_behave_remove_dir_all_force(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_BUILDS).unwrap(),
    );
}

fn radula_behave_bootstrap_cross_construct() {
    let radula_behave_construct_cross = |x: &'static str| {
        radula_behave_construct(x, constants::RADULA_DIRECTORY_CROSS);
    };

    // Filesystem & Package Management
    radula_behave_construct_cross("iana-etc");
    radula_behave_construct_cross("hydroskeleton");
    radula_behave_construct_cross("cerata");
    radula_behave_construct_cross("radula");

    // Headers
    radula_behave_construct_cross("musl-utils");
    radula_behave_construct_cross("linux-headers");

    // Init
    radula_behave_construct_cross("skalibs");
    radula_behave_construct_cross("execline");
    radula_behave_construct_cross("s6");
    radula_behave_construct_cross("utmps");

    // Permissions
    radula_behave_construct_cross("attr");
    radula_behave_construct_cross("acl");
    radula_behave_construct_cross("shadow");
    radula_behave_construct_cross("libressl");

    // Userland
    radula_behave_construct_cross("toybox");
    radula_behave_construct_cross("bc");
    radula_behave_construct_cross("diffutils");
    radula_behave_construct_cross("file");
    radula_behave_construct_cross("findutils");
    radula_behave_construct_cross("grep");
    radula_behave_construct_cross("hostname");
    radula_behave_construct_cross("mlocate");
    radula_behave_construct_cross("sed");
    radula_behave_construct_cross("which");

    // Compression
    radula_behave_construct_cross("bzip2");
    radula_behave_construct_cross("lbzip2");
    radula_behave_construct_cross("lbzip2-utils");
    radula_behave_construct_cross("lz4");
    radula_behave_construct_cross("lzlib");
    radula_behave_construct_cross("plzip");
    radula_behave_construct_cross("xz");
    radula_behave_construct_cross("zlib-ng");
    radula_behave_construct_cross("pigz");
    radula_behave_construct_cross("zstd");
    radula_behave_construct_cross("libarchive");

    // Synchronization
    radula_behave_construct_cross("rsync");

    // Shell
    radula_behave_construct_cross("netbsd-curses");
    radula_behave_construct_cross("oksh");
    radula_behave_construct_cross("dash");

    // Editors & Pagers
    radula_behave_construct_cross("libedit");
    radula_behave_construct_cross("pcre2");
    radula_behave_construct_cross("less");
    radula_behave_construct_cross("vim");
    radula_behave_construct_cross("mandoc");

    // Networking
    radula_behave_construct_cross("libcap");
    radula_behave_construct_cross("iproute2");
    radula_behave_construct_cross("iputils");
    radula_behave_construct_cross("dhcp");

    // Utilities
    radula_behave_construct_cross("psmisc");
    radula_behave_construct_cross("procps-ng");
    radula_behave_construct_cross("util-linux");
    radula_behave_construct_cross("e2fsprogs");
    radula_behave_construct_cross("kmod");
    radula_behave_construct_cross("pciutils");
    radula_behave_construct_cross("hwids");
    radula_behave_construct_cross("eudev");

    // Services
    radula_behave_construct_cross("s6-linux-init");
    radula_behave_construct_cross("s6-rc");
    radula_behave_construct_cross("s6-boot-scripts");

    // Kernel
    radula_behave_construct_cross("linux");
}

fn radula_behave_bootstrap_cross_environment() {
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY,
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY).unwrap())
            .join(constants::RADULA_DIRECTORY_CROSS),
    );

    let x = env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY).unwrap();

    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_BUILDS,
        Path::new(&x).join(constants::RADULA_DIRECTORY_BUILDS),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_SOURCES,
        Path::new(&x).join(constants::RADULA_DIRECTORY_SOURCES),
    );

    let y = env::var(constants::RADULA_ENVIRONMENT_TUPLE_TARGET).unwrap();

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_ARCHIVER,
        [&y, "-", constants::RADULA_CROSS_ARCHIVER].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_ASSEMBLER,
        [&y, "-", constants::RADULA_CROSS_ASSEMBLER].concat(),
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_BUILD_C_COMPILER,
        constants::RADULA_CROSS_C_COMPILER,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_C_COMPILER,
        [&y, "-", constants::RADULA_CROSS_C_COMPILER].concat(),
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_C_COMPILER_LINKER,
        constants::RADULA_CROSS_C_CXX_COMPILER_LINKER,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_C_PREPROCESSOR,
        [
            &y,
            "-",
            constants::RADULA_CROSS_C_COMPILER,
            " ",
            constants::RADULA_CROSS_C_PREPROCESSOR,
        ]
        .concat(),
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_COMPILE,
        [&y, "-"].concat(),
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_CXX_COMPILER,
        [&y, "-", constants::RADULA_CROSS_CXX_COMPILER].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_CXX_COMPILER_LINKER,
        constants::RADULA_CROSS_C_CXX_COMPILER_LINKER,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_HOST_C_COMPILER,
        constants::RADULA_CROSS_C_COMPILER,
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_HOST_CXX_COMPILER,
        constants::RADULA_CROSS_CXX_COMPILER,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_LINKER,
        [&y, "-", constants::RADULA_CROSS_LINKER].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_NAMES,
        [&y, "-", constants::RADULA_CROSS_NAMES].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_OBJECT_COPY,
        [&y, "-", constants::RADULA_CROSS_OBJECT_COPY].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_OBJECT_DUMP,
        [&y, "-", constants::RADULA_CROSS_OBJECT_DUMP].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_RANDOM_ACCESS_LIBRARY,
        [&y, "-", constants::RADULA_CROSS_RANDOM_ACCESS_LIBRARY].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_READ_ELF,
        [&y, "-", constants::RADULA_CROSS_READ_ELF].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_SIZE,
        [&y, "-", constants::RADULA_CROSS_SIZE].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_STRINGS,
        [&y, "-", constants::RADULA_CROSS_STRINGS].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_STRIP,
        [&y, "-", constants::RADULA_CROSS_STRIP].concat(),
    );
}

fn radula_behave_bootstrap_cross_prepare() {
    let radula_behave_bootstrap_restore = |x: &'static str| {
        Command::new(constants::RADULA_TOOTH_RSYNC)
            .args(&[
                constants::RADULA_TOOTH_RSYNC_FLAGS,
                Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS).unwrap())
                    .join(x)
                    .to_str()
                    .unwrap(),
                &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS).unwrap(),
                "--delete",
            ])
            .stdout(Stdio::null())
            .spawn();
    };

    radula_behave_bootstrap_restore(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS);
    radula_behave_bootstrap_restore(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN);

    radula_behave_remove_dir_all_force(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_BUILDS).unwrap(),
    );
    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_BUILDS).unwrap());

    // remove `logs/cross.log`
    //fs::remove_file(
    //Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS).unwrap())
    //.join(constants::SOME_PATH_TO_LOG_FILE),
    //);

    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_SOURCES).unwrap());
}

fn radula_behave_bootstrap_cross_strip() {
    Command::new(constants::RADULA_TOOTH_FIND)
        .args(&[
            Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap())
                .join(constants::RADULA_PATH_ETC)
                .to_str()
                .unwrap(),
            "-type d -empty -delete",
        ])
        .spawn();

    let x = String::from(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap())
            .join(constants::RADULA_PATH_USR)
            .to_str()
            .unwrap(),
    );

    Command::new(constants::RADULA_TOOTH_FIND)
        .args(&[
            &x,
            "-name *.a -type f -exec",
            constants::RADULA_CROSS_STRIP,
            "-gv {} \\;",
        ])
        .spawn();
    Command::new(constants::RADULA_TOOTH_FIND)
        .args(&[
            &x,
            "\\( -name *.so* -a ! -name *dbg \\) -type f -exec",
            constants::RADULA_CROSS_STRIP,
            "--strip-unneeded -v {} \\;",
        ])
        .spawn();
    Command::new(constants::RADULA_TOOTH_FIND)
        .args(&[
            &x,
            "-type f -exec",
            constants::RADULA_CROSS_STRIP,
            "-sv {} \\;",
        ])
        .spawn();
    Command::new(constants::RADULA_TOOTH_FIND)
        .args(&[&x, "-name *.la -delete"])
        .spawn();

    // This could be removed in the future if nothing generates `charset.alias`
    fs::remove_file(
        Path::new(&x)
            .join(constants::RADULA_PATH_LIB)
            .join(constants::RADULA_PATH_CHARSET_ALIAS),
    );
}

fn radula_behave_bootstrap_distclean() {
    radula_behave_remove_dir_all_force(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS).unwrap(),
    );

    fs::remove_file(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS).unwrap())
            .join(constants::RADULA_PATH_GLAUCUS_IMAGE),
    );

    radula_behave_remove_dir_all_force(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES).unwrap(),
    );

    radula_behave_bootstrap_clean();

    // Only remove `RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY` completely after
    // `RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_BUILDS` and
    // `RADULA_ENVIRONMENT_DIRECTORY_CROSS_BUILDS` are removed
    radula_behave_remove_dir_all_force(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY).unwrap(),
    );
}

fn radula_behave_bootstrap_environment() {
    let x = fs::canonicalize("..").unwrap();

    env::set_var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS, &x);

    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS,
        x.join(constants::RADULA_DIRECTORY_BACKUPS),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CERATA,
        x.join(constants::RADULA_DIRECTORY_CERATA),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS,
        x.join(constants::RADULA_DIRECTORY_CROSS),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS,
        x.join(constants::RADULA_DIRECTORY_LOGS),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES,
        x.join(constants::RADULA_DIRECTORY_SOURCES),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY,
        x.join(constants::RADULA_DIRECTORY_TEMPORARY),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN,
        x.join(constants::RADULA_DIRECTORY_TOOLCHAIN),
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_PATH,
        Path::new(
            &[
                Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN).unwrap())
                    .join("bin")
                    .to_str()
                    .unwrap(),
                ":",
            ]
            .concat(),
        )
        .join(
            env::var(constants::RADULA_ENVIRONMENT_PATH)
                .unwrap()
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );

    radula_behave_teeth_environment();
}

fn radula_behave_bootstrap_initialize() {
    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS).unwrap());
    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES).unwrap());
}

fn radula_behave_bootstrap_toolchain_backup() {
    let radula_behave_bootstrap_backup = |x: &'static str| {
        Command::new(constants::RADULA_TOOTH_RSYNC)
            .args(&[
                constants::RADULA_TOOTH_RSYNC_FLAGS,
                &env::var(x).unwrap(),
                &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS).unwrap(),
                "--delete",
            ])
            .stdout(Stdio::null())
            .spawn();
    };

    radula_behave_bootstrap_backup(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS);
    radula_behave_bootstrap_backup(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN);
}

fn radula_behave_bootstrap_toolchain_construct() {
    let radula_behave_construct_toolchain = |x: &'static str| {
        radula_behave_construct(x, constants::RADULA_DIRECTORY_TOOLCHAIN);
    };

    radula_behave_construct_toolchain("musl-headers");
    radula_behave_construct_toolchain("binutils");
    radula_behave_construct_toolchain("gcc");
    radula_behave_construct_toolchain("musl");
    radula_behave_construct_toolchain("libgcc");
    radula_behave_construct_toolchain("libstdc++-v3");
    radula_behave_construct_toolchain("libgomp");
}

fn radula_behave_bootstrap_toolchain_environment() {
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY,
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY).unwrap())
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN),
    );

    let x = env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY).unwrap();

    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_BUILDS,
        Path::new(&x).join(constants::RADULA_DIRECTORY_BUILDS),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_SOURCES,
        Path::new(&x).join(constants::RADULA_DIRECTORY_SOURCES),
    );
}

fn radula_behave_bootstrap_toolchain_prepare() {
    radula_behave_remove_dir_all_force(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap(),
    );
    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap());

    radula_behave_remove_dir_all_force(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN).unwrap(),
    );
    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN).unwrap());

    radula_behave_remove_dir_all_force(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_BUILDS).unwrap(),
    );
    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_BUILDS).unwrap());

    fs::create_dir_all(
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_SOURCES).unwrap(),
    );
}

fn radula_behave_ccache_environment() {
    env::set_var(
        constants::RADULA_ENVIRONMENT_PATH,
        Path::new(&[constants::RADULA_PATH_CCACHE, ":"].concat()).join(
            env::var(constants::RADULA_ENVIRONMENT_PATH)
                .unwrap()
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );
}

fn radula_behave_construct(x: &'static str, y: &'static str) {
    // We only require `nom` and `ver` from the `ceras` file
    let z: [String; 8] = radula_behave_source(x);

    // Perform swallow within construct since it makes more sense that way
    match x {
        // `gcc` will be removed from the list below when dependency resolution is working
        "gcc" => {
            radula_behave_swallow("gmp");
            radula_behave_swallow("mpfr");
            radula_behave_swallow("mpc");
            radula_behave_swallow("isl");
            radula_behave_swallow(x);
        }
        "hydroskeleton" => (),
        "libgcc" | "libgomp" | "libstdc++-v3" => radula_behave_swallow("gcc"),
        "linux-headers" => radula_behave_swallow("linux"),
        "musl-headers" | "musl-utils" => radula_behave_swallow("musl"),
        _ => radula_behave_swallow(x),
    }

    // The `.wait()` is needed to allow `Ctrl + C` to work...
    Command::new(constants::RADULA_TOOTH_SHELL)
        .args(&[
            constants::RADULA_TOOTH_SHELL_FLAGS,
            &format!(
                // `ceras` and `*.ceras` files are only using `nom` and `ver`.
                //
                // All basic functions need to be called together to prevent the loss of the
                // current working directory, otherwise we'd have to store it and pass it or `cd`
                // into it whenever a basic function is called.
                //
                // The basic function `check` won't be used for now...
                "nom={} ver={} . {} && prepare && configure && build && install",
                z[0],
                z[1],
                Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CERATA).unwrap())
                    .join(x)
                    .join([y, ".ceras"].concat())
                    .to_str()
                    .unwrap()
            ),
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

// This will be used whether we're bootstrapping or not so don't add _bootstrap_ to its name
fn radula_behave_flags_environment() {}

fn radula_behave_pkg_config_environment() {
    env::set_var(
        constants::RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR,
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap()).join(
            constants::RADULA_PATH_PKG_CONFIG_LIBDIR_PATH
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_PKG_CONFIG_PATH,
        env::var(constants::RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR).unwrap(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_PKG_CONFIG_SYSROOT_DIR,
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap()).join(
            constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );

    // These environment variables are only `pkgconf` specific, but setting them
    // won't do any harm...
    env::set_var(
        constants::RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_INCLUDE_PATH,
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap()).join(
            constants::RADULA_PATH_PKG_CONFIG_SYSTEM_INCLUDE_PATH
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_LIBRARY_PATH,
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap()).join(
            constants::RADULA_PATH_PKG_CONFIG_SYSTEM_LIBRARY_PATH
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );
}

// Use our custom version of `remove_dir_all`
fn radula_behave_remove_dir_all_force(x: &str) {
    if Path::new(x).exists() {
        fs::remove_dir_all(x);
    }
}

// Sources the `ceras` file and returns an array of strings representing the
// variables inside of it
fn radula_behave_source(x: &'static str) -> [String; 8] {
    // We can't have a `"".to_string()` copied 8 times, which is why we're using
    // a constant beforehand
    const S: String = String::new();
    let mut y: [String; 8] = [S; 8];

    // Magic
    for (i, j) in String::from_utf8_lossy(
        &Command::new(constants::RADULA_TOOTH_SHELL)
            .args(&[
                constants::RADULA_TOOTH_SHELL_FLAGS,
                &format!(
                    ". {} && echo $nom~~$ver~~$cmt~~$url~~$sum~~$cys~~$cnt",
                    Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CERATA).unwrap())
                        .join(x)
                        .join("ceras")
                        .to_str()
                        .unwrap(),
                ),
            ])
            .output()
            .unwrap()
            .stdout,
    )
    .trim()
    .split("~~")
    .enumerate()
    {
        y[i] = j.to_owned();
    }

    return y;
}

// Fetches and verifies ceras's source
fn radula_behave_swallow(x: &'static str) {
    // Receive the variables from the `ceras` file
    let y: [String; 8] = radula_behave_source(x);

    // Get the path of the source directory
    let z = String::from(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES).unwrap())
            .join(&y[0])
            .to_str()
            .unwrap(),
    );

    let w = String::from(
        Path::new(&z)
            .join(Path::new(&y[3]).file_name().unwrap())
            .to_str()
            .unwrap(),
    );

    // verify
    let radula_behave_verify = || {
        write!(
            Command::new(constants::RADULA_TOOTH_CHECKSUM)
                .arg(constants::RADULA_TOOTH_CHECKSUM_FLAGS)
                .stdin(Stdio::piped())
                .spawn()
                .unwrap()
                .stdin
                .unwrap(),
            "{}",
            [&y[4], " ", &w].concat()
        )
        .unwrap();
    };

    if !Path::is_dir(Path::new(&z)) {
        if y[1] == constants::RADULA_TOOTH_GIT {
            // Clone the `git` repo
            Command::new(constants::RADULA_TOOTH_GIT)
                .args(&["clone", &y[3], &z])
                .spawn();
            // Checkout the freshly cloned `git` repo at the specified commit number
            Command::new(constants::RADULA_TOOTH_GIT)
                .args(&["-C", &z, "checkout", &y[2]])
                .spawn();
        } else {
            fs::create_dir_all(&z);

            Command::new(constants::RADULA_TOOTH_CURL)
                .args(&[constants::RADULA_TOOTH_CURL_FLAGS, &w, &y[3]])
                .spawn();

            // verify
            radula_behave_verify();

            Command::new(constants::RADULA_TOOTH_TAR)
                .args(&["xvf", &w, "-C", &z])
                .stdout(Stdio::null())
                .spawn();
        }
    } else if y[1] != constants::RADULA_TOOTH_GIT {
        radula_behave_verify();
    }
}

fn radula_behave_teeth_environment() {
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_AUTORECONF,
        [
            constants::RADULA_TOOTH_AUTORECONF,
            " ",
            constants::RADULA_TOOTH_AUTORECONF_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_CHMOD,
        [
            constants::RADULA_TOOTH_CHMOD,
            " ",
            constants::RADULA_TOOTH_CHMOD_CHOWN_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_CHOWN,
        [
            constants::RADULA_TOOTH_CHOWN,
            " ",
            constants::RADULA_TOOTH_CHMOD_CHOWN_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_LN,
        [
            constants::RADULA_TOOTH_LN,
            " ",
            constants::RADULA_TOOTH_LN_FLAGS,
        ]
        .concat(),
    );

    // `make` and its flags
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_MAKE,
        constants::RADULA_TOOTH_MAKE,
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_MAKE_FLAGS,
        [
            "-j",
            // We need to trim the output or parse won't work...
            &(String::from_utf8_lossy(
                &Command::new(constants::RADULA_TOOTH_NPROC)
                    .output()
                    .unwrap()
                    .stdout,
            )
            .trim()
            .parse::<f32>()
            .unwrap()
                * 1.5)
                .to_string(),
            " ",
            constants::RADULA_TOOTH_MAKE_FLAGS,
        ]
        .concat(),
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_MKDIR,
        [
            constants::RADULA_TOOTH_MKDIR,
            " ",
            constants::RADULA_TOOTH_MKDIR_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_MV,
        [
            constants::RADULA_TOOTH_MV,
            " ",
            constants::RADULA_TOOTH_MV_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_PATCH,
        [
            constants::RADULA_TOOTH_PATCH,
            " ",
            constants::RADULA_TOOTH_PATCH_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_RM,
        [
            constants::RADULA_TOOTH_RM,
            " ",
            constants::RADULA_TOOTH_RM_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_RSYNC,
        [
            constants::RADULA_TOOTH_RSYNC,
            " ",
            constants::RADULA_TOOTH_RSYNC_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_UMOUNT,
        [
            constants::RADULA_TOOTH_UMOUNT,
            " ",
            constants::RADULA_TOOTH_UMOUNT_FLAGS,
        ]
        .concat(),
    );
}

//
// Help Functions
//

fn radula_help(x: &'static str) {
    println!("{}\n\n{}", constants::RADULA_HELP_VERSION, x);
}

//
// Options Functions
//

pub fn radula_options() {
    let mut x = env::args().skip(1);

    if x.len() < 1 {
        radula_help(constants::RADULA_HELP);
        exit(1);
    }

    while let Some(y) = x.next().as_deref() {
        match y {
            // `.unwrap_or_default()` actually returns an empty string literal which gets matched
            // to `_` in the below switch (if we had `.unwrap_or("h")` then that'll return the help
            // message without exiting with an error status of `1`).
            "-b" | "--behave" => match x.next().as_deref().unwrap_or_default() {
                "b" | "bootstrap" => {
                    match x.next().as_deref().unwrap_or_default() {
                        "c" | "clean" => {
                            radula_behave_bootstrap_environment();

                            radula_behave_bootstrap_toolchain_environment();

                            // Only included to work for cross which in turn is only included to make
                            // clean work...
                            radula_behave_bootstrap_architecture_environment(
                                constants::RADULA_ARCHITECTURE_X86_64,
                            );

                            radula_behave_bootstrap_cross_environment();

                            radula_behave_bootstrap_clean();

                            println!("clean complete");
                        }
                        "d" | "distclean" => {
                            radula_behave_bootstrap_environment();

                            radula_behave_bootstrap_toolchain_environment();

                            // Only included to work for cross which in turn is only included to make
                            // clean work...
                            radula_behave_bootstrap_architecture_environment(
                                constants::RADULA_ARCHITECTURE_X86_64,
                            );

                            radula_behave_bootstrap_cross_environment();

                            radula_behave_bootstrap_distclean();

                            println!("distclean complete");
                        }
                        "h" | "help" => radula_help(constants::RADULA_HELP_BEHAVE_BOOTSTRAP),
                        "i" | "image" => println!("Do nothing"),
                        "l" | "list" => radula_help(constants::RADULA_HELP_BEHAVE_BOOTSTRAP_LIST),
                        "r" | "require" => {
                            println!("Checking if host has all required packages...")
                        }
                        "s" | "release" => println!("release complete"),
                        "t" | "toolchain" => {
                            radula_behave_bootstrap_environment();

                            radula_behave_ccache_environment();

                            radula_behave_bootstrap_initialize();

                            radula_behave_bootstrap_architecture_environment(
                                constants::RADULA_ARCHITECTURE_X86_64,
                            );

                            radula_behave_bootstrap_toolchain_environment();

                            radula_behave_bootstrap_clean();

                            radula_behave_bootstrap_toolchain_prepare();
                            radula_behave_bootstrap_toolchain_construct();
                            radula_behave_bootstrap_toolchain_backup();
                        }
                        "x" | "cross" => {
                            radula_behave_bootstrap_environment();

                            radula_behave_pkg_config_environment();

                            radula_behave_bootstrap_architecture_environment(
                                constants::RADULA_ARCHITECTURE_X86_64,
                            );

                            radula_behave_flags_environment();

                            radula_behave_bootstrap_cross_environment();
                            radula_behave_bootstrap_cross_prepare();

                            radula_behave_bootstrap_cross_construct();

                            radula_behave_bootstrap_cross_strip();
                        }
                        _ => {
                            radula_help(constants::RADULA_HELP_BEHAVE_BOOTSTRAP);
                            exit(1);
                        }
                    }
                    return;
                }
                // `return` should be removed to allow dealing with multiple cerata simultaneously
                "e" | "envenomate" => {
                    match x.next().as_deref().unwrap_or_default() {
                        "h" | "help" => radula_help(constants::RADULA_HELP_BEHAVE_ENVENOMATE),
                        _ => {
                            radula_help(constants::RADULA_HELP_BEHAVE_ENVENOMATE);
                            exit(1);
                        }
                    }
                    return;
                }
                // `return` should be removed to allow dealing with multiple cerata simultaneously
                "i" | "binary" => {
                    match x.next().as_deref().unwrap_or_default() {
                        "h" | "help" => radula_help(constants::RADULA_HELP_BEHAVE_BINARY),
                        _ => {
                            radula_help(constants::RADULA_HELP_BEHAVE_BINARY);
                            exit(1);
                        }
                    }
                    return;
                }
                "h" | "help" => {
                    radula_help(constants::RADULA_HELP_BEHAVE);
                    return;
                }
                _ => {
                    radula_help(constants::RADULA_HELP_BEHAVE);
                    exit(1);
                }
            },
            "-c" | "--ceras" => {
                match x.next().as_deref().unwrap_or_default() {
                    "c" | "cnt" => println!("Do nothing"),
                    "h" | "help" => radula_help(constants::RADULA_HELP_CERAS),
                    "n" | "nom" => println!("Do nothing"),
                    "s" | "sum" => println!("Do nothing"),
                    "u" | "url" => println!("Do nothing"),
                    "v" | "ver" => println!("Do nothing"),
                    "y" | "cys" => println!("Do nothing"),
                    _ => {
                        radula_help(constants::RADULA_HELP_CERAS);
                        exit(1);
                    }
                }
                return;
            }
            "-h" | "--help" => {
                radula_help(constants::RADULA_HELP);
                return;
            }
            "-v" | "--version" => {
                println!("{}", constants::RADULA_HELP_VERSION);
                return;
            }
            _ => {
                radula_help(constants::RADULA_HELP);
                exit(1);
            }
        }
    }
}