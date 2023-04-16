# Copyright (c) 2018-2023, Firas Khalil Khana
# Distributed under the terms of the ISC License

import std/[
    os,
    parseopt
]

import
    architecture,
    bootstrap,
    ceras,
    clean,
    constants,
    cross,
    flags,
    teeth,
    toolchain

#
# Options Function
#

proc radula_options*() =
    # Catch Ctrl-C and exit gracefully
    setControlCHook(radula_behave_abort)

    if paramCount() < 1:
        echo RADULA_HELP
        quit(1)

    var
        p = initOptParser()
    for kind, key, value in getOpt():
        p.next()
        case kind
        of cmdArgument, cmdEnd:
            echo RADULA_HELP
            quit(1)
        of cmdLongOption, cmdShortOption:
            case p.key
            of "b", "behave":
                case value
                of "b", "bootstrap":
                    p.next()
                    case p.key
                    of "c", "clean":
                        radula_behave_bootstrap_environment()
                        radula_behave_bootstrap_toolchain_environment_directories()
                        radula_behave_bootstrap_cross_environment_directories()

                        radula_behave_bootstrap_clean()

                        echo "clean complete"
                    of "d", "distclean":
                        radula_behave_bootstrap_environment()
                        radula_behave_bootstrap_toolchain_environment_directories()
                        radula_behave_bootstrap_cross_environment_directories()

                        radula_behave_bootstrap_distclean()

                        echo "distclean complete"
                    of "h", "help":
                        echo RADULA_HELP_BEHAVE_BOOTSTRAP
                    of "i", "img":
                        echo "img complete"
                    of "l", "list":
                        echo RADULA_HELP_BEHAVE_BOOTSTRAP_LIST
                    of "r", "require":
                        echo "Checking if host has all required packages..."
                    of "s", "release":
                        radula_behave_bootstrap_environment()

                        radula_behave_bootstrap_toolchain_release()

                        echo "release complete"
                    of "t", "toolchain":
                        radula_behave_bootstrap_environment()

                        radula_behave_teeth_environment()

                        radula_behave_architecture_environment(RADULA_ARCHITECTURE_X86_64_V3)

                        radula_behave_bootstrap_toolchain_environment_directories()

                        # Needed for clean to work
                        radula_behave_bootstrap_cross_environment_directories()

                        radula_behave_bootstrap_clean()

                        radula_behave_bootstrap_initialize()

                        radula_behave_bootstrap_toolchain_ccache()

                        radula_behave_bootstrap_toolchain_prepare()
                        radula_behave_bootstrap_toolchain_envenomate()
                        radula_behave_bootstrap_toolchain_backup()

                        echo ""
                        echo "toolchain complete"
                    of "x", "cross":
                        radula_behave_bootstrap_environment()

                        radula_behave_teeth_environment()

                        radula_behave_architecture_environment(RADULA_ARCHITECTURE_X86_64_V3)

                        radula_behave_flags_environment()

                        radula_behave_bootstrap_cross_environment_directories()
                        radula_behave_bootstrap_cross_environment_teeth()

                        radula_behave_bootstrap_cross_ccache()

                        radula_behave_bootstrap_cross_environment_pkg_config()

                        radula_behave_bootstrap_cross_prepare()
                        radula_behave_bootstrap_cross_envenomate()

                        echo ""
                        echo "cross complete"
                    of "z", "iso":
                        echo "iso complete"
                    else:
                        echo RADULA_HELP_BEHAVE_BOOTSTRAP
                        quit(1)
                    quit(0)
                else:
                    echo RADULA_HELP_BEHAVE
                    quit(1)
            of "c", "ceras":
                radula_behave_ceras_print(remainingArgs(p))
                quit(0)
            of "h", "help":
                echo RADULA_HELP
                quit(0)
            of "v", "version":
                echo RADULA_HELP_VERSION
                quit(0)
            else:
                echo RADULA_HELP
                quit(1)
