# Copyright (c) 2018-2024, Firas Khalil Khana
# Distributed under the terms of the ISC License

import
  std/[algorithm, os, osproc, sequtils, strformat, strutils, tables, terminal, times],
  constants, tools,
  hashlib/misc/blake3, toml_serialization, toposort

type
  Ceras = object
    nom, ver, cmt, url, sum, bld, run = RAD_PRINT_NIL

# Check if the `ceras` src is extracted
proc rad_ceras_check_extract_src(file: string): bool =
  walkDir(parentDir(file)).toSeq().len() > 1

proc rad_ceras_clean*() =
  removeDir(RAD_PATH_RAD_LOG)
  removeDir(RAD_PATH_RAD_TMP)

proc rad_ceras_distclean*() =
  removeDir(RAD_PATH_RAD_CACHE_BIN)
  removeDir(RAD_PATH_RAD_CACHE_SRC)
  removeDir(RAD_PATH_RAD_CACHE_VENOM)

  rad_ceras_clean()

# Return the full path to the `ceras` file
func rad_ceras_path(nom: string): string =
  RAD_PATH_RAD_LIB_CLUSTERS_GLAUCUS / nom / RAD_FILE_CERAS

# Check if the `ceras` file exists
proc rad_ceras_check_exist(nom: string) =
  if not fileExists(rad_ceras_path(nom)):
    rad_abort(&"""{"nom":8}{nom:48}""")

# Parse the `ceras` file
proc rad_ceras_parse(nom: string): Ceras =
  Toml.loadFile(rad_ceras_path(nom), Ceras)

proc rad_ceras_print*(cerata: openArray[string]) =
  for nom in cerata.deduplicate():
    rad_ceras_check_exist(nom)

    let ceras = rad_ceras_parse(nom)

    echo "nom  :: ", ceras.nom
    echo "ver  :: ", ceras.ver
    echo "cmt  :: ", ceras.cmt
    echo "url  :: ", ceras.url
    echo "sum  :: ", ceras.sum
    echo "bld  :: ", ceras.bld
    echo "run  :: ", ceras.run

    echo ""

proc rad_ceras_print_content(idx: int, nom, ver, status: string) =
  styledEcho fgMagenta, styleBright, &"""{idx + 1:<8}{nom:24}{ver:24}{status:8}{now().format("hh:mm tt")}""", resetStyle

proc rad_ceras_print_footer(idx: int, nom, ver, status: string) =
  styledEcho fgGreen, &"{idx + 1:<8}", resetStyle, &"{nom:24}{ver:24}", fgGreen, &"{status:8}", fgYellow, now().format("hh:mm tt"), fgDefault

proc rad_ceras_print_header() =
  styledEcho styleBright, &"""{"idx":8}{"nom":24}{"ver":24}{"cmd":8}fin""", resetStyle

# Resolve deps using topological sorting
proc rad_ceras_resolve_deps(nom: string, deps: var Table[string, seq[string]], run = true) =
  let ceras = rad_ceras_parse(nom)

  deps[ceras.nom] =
    if run:
      case ceras.run
      of RAD_PRINT_NIL:
        @[]
      else:
        ceras.run.split()
    else:
      case ceras.bld
      of RAD_PRINT_NIL:
        @[]
      else:
        ceras.bld.split()

  if deps[nom].len() > 0:
    for dep in deps[nom]:
      rad_ceras_resolve_deps(dep, deps, if run: true else: false)

proc rad_ceras_check*(cerata: openArray[string], run = true): seq[string] =
  var deps: Table[string, seq[string]]

  for nom in cerata.deduplicate():
    rad_ceras_check_exist(nom)

    rad_ceras_resolve_deps(nom, deps, if run: true else: false)

  topoSort(deps)

proc rad_ceras_fetch(cerata: openArray[string]) =
  rad_ceras_print_header()

  for idx, nom in cerata:
    let ceras = rad_ceras_parse(nom)

    # Check for virtual cerata
    case ceras.url
    of RAD_PRINT_NIL:
      rad_ceras_print_footer(idx, ceras.nom, ceras.ver, RAD_PRINT_FETCH)
    else:
      let
        path = getEnv(RAD_ENV_DIR_SRCD) / ceras.nom
        archive = path / lastPathPart(ceras.url)

      if dirExists(path):
        case ceras.ver
        of RAD_TOOL_GIT:
          rad_ceras_print_footer(idx, ceras.nom, ceras.cmt, RAD_PRINT_FETCH)
        else:
          if rad_verify_file(archive, ceras.sum):
            if not rad_ceras_check_extract_src(archive):
              rad_ceras_print_content(idx, ceras.nom, ceras.ver, RAD_PRINT_FETCH)

              discard rad_extract_tar(archive, path)

              cursorUp 1
              eraseLine()

            rad_ceras_print_footer(idx, ceras.nom, ceras.ver, RAD_PRINT_FETCH)
          else:
            rad_ceras_print_content(idx, ceras.nom, ceras.ver, RAD_PRINT_FETCH)

            removeDir(path)
            createDir(path)

            discard rad_download_file(archive, ceras.url)

            if rad_verify_file(archive, ceras.sum):
              discard rad_extract_tar(archive, path)
            else:
              cursorUp 1
              eraseLine()

              rad_abort(&"""{"sum":8}{ceras.nom:24}{ceras.ver:24}""")

            cursorUp 1
            eraseLine()

            rad_ceras_print_footer(idx, ceras.nom, ceras.ver, RAD_PRINT_FETCH)
      else:
        case ceras.ver
        of RAD_TOOL_GIT:
          rad_ceras_print_content(idx, ceras.nom, ceras.cmt, RAD_PRINT_CLONE)

          discard rad_clone_repo(path, ceras.url)
          discard rad_checkout_repo(ceras.cmt, path)

          cursorUp 1
          eraseLine()

          rad_ceras_print_footer(idx, ceras.nom, ceras.cmt, RAD_PRINT_CLONE)
        else:
          rad_ceras_print_content(idx, ceras.nom, ceras.ver, RAD_PRINT_FETCH)

          createDir(path)

          discard rad_download_file(archive, ceras.url)

          if rad_verify_file(archive, ceras.sum):
            discard rad_extract_tar(archive, path)
          else:
            cursorUp 1
            eraseLine()

            rad_abort(&"""{"sum":8}{ceras.nom:24}{ceras.ver:24}""")

          cursorUp 1
          eraseLine()

          rad_ceras_print_footer(idx, ceras.nom, ceras.ver, RAD_PRINT_FETCH)

proc rad_ceras_build*(cerata: openArray[string], stage = RAD_STAGE_NATIVE, resolve = true) =
  let cluster = rad_ceras_check(cerata, false)

  rad_ceras_fetch(cluster)

  echo ""

  rad_ceras_print_header()

  for idx, nom in (if resolve: cluster else: cerata.toSeq()):
    let
      ceras = rad_ceras_parse(nom)

      log = getEnv(RAD_ENV_DIR_LOGD) / &"{ceras.nom}{CurDir}{RAD_DIR_LOG}"

    rad_ceras_print_content(idx, ceras.nom,
      case ceras.ver
      of RAD_TOOL_GIT:
        ceras.cmt
      else:
        ceras.ver
    , RAD_PRINT_BUILD)

    case stage
    of RAD_STAGE_NATIVE:
      if fileExists(RAD_PATH_RAD_CACHE_VENOM / ceras.nom / &"""{ceras.nom}{(
        case ceras.url
        of RAD_PRINT_NIL:
          ""
        else:
          '-' & ceras.ver
      )}{(
        case ceras.ver
        of RAD_TOOL_GIT:
          '-' & ceras.cmt
        else:
          ""
      )}{RAD_FILE_TAR_ZST}"""):
        cursorUp 1
        eraseLine()

        rad_ceras_print_footer(idx, ceras.nom,
          case ceras.ver
          of RAD_TOOL_GIT:
            ceras.cmt
          else:
            ceras.ver
        , RAD_PRINT_BUILD)

        continue

      putEnv(RAD_ENV_DIR_SACD, RAD_PATH_RAD_CACHE_VENOM / ceras.nom / RAD_DIR_SAC)
      createDir(getEnv(RAD_ENV_DIR_SACD))

    # We only use `nom` and `ver` from `ceras`
    #
    # All phases need to be called sequentially to prevent the loss of the
    # current working dir...
    var status = execCmd(&"""
      {RAD_TOOL_SHELL} {RAD_FLAGS_TOOL_SHELL_COMMAND} 'nom={ceras.nom} ver={ceras.ver} {CurDir} {RAD_PATH_RAD_LIB_CLUSTERS_GLAUCUS}{DirSep}{ceras.nom}{DirSep}{RAD_PRINT_BUILD}{CurDir}{stage} &&
      ceras_prepare $1 &&
      ceras_configure $1 &&
      ceras_build $1 &&
      ceras_check $1 &&
      ceras_install $1'
    """ % &">> {log} 2>&1")

    cursorUp 1
    eraseLine()

    if status != 0:
      rad_abort(&"""{status:<8}{ceras.nom:24}{(
        case ceras.ver
        of RAD_TOOL_GIT:
          ceras.cmt
        else:
          ceras.ver
      ):24}""")

    case stage
    of RAD_STAGE_NATIVE:
      status = rad_create_tar_zst(RAD_PATH_RAD_CACHE_VENOM / ceras.nom / &"""{ceras.nom}{(
        case ceras.url
        of RAD_PRINT_NIL:
          ""
        else:
          '-' & ceras.ver
      )}{(
        case ceras.ver
        of RAD_TOOL_GIT:
          '-' & ceras.cmt
        else:
          ""
      )}{RAD_FILE_TAR_ZST}""", getEnv(RAD_ENV_DIR_SACD))

      if status == 0:
        rad_gen_sum(getEnv(RAD_ENV_DIR_SACD), RAD_PATH_RAD_CACHE_VENOM / nom / RAD_FILE_SUM)

        removeDir(getEnv(RAD_ENV_DIR_SACD))

    rad_ceras_print_footer(idx, ceras.nom,
      case ceras.ver
      of RAD_TOOL_GIT:
        ceras.cmt
      else:
        ceras.ver
    , RAD_PRINT_BUILD)

proc rad_ceras_install*(cerata: openArray[string]) =
  let cluster = rad_ceras_check(cerata)

  rad_ceras_print_header()

  for idx, nom in cluster:
    let ceras = rad_ceras_parse(nom)

    rad_ceras_print_content(idx, ceras.nom,
      case ceras.ver
      of RAD_TOOL_GIT:
        ceras.cmt
      else:
        ceras.ver
    , RAD_PRINT_INSTALL)

    let status = rad_extract_tar(RAD_PATH_RAD_CACHE_VENOM / ceras.nom / &"""{ceras.nom}{(
      case ceras.url
      of RAD_PRINT_NIL:
        ""
      else:
        '-' & ceras.ver
    )}{(
      case ceras.ver
      of RAD_TOOL_GIT:
        '-' & ceras.cmt
      else:
        ""
    )}{RAD_FILE_TAR_ZST}""", $DirSep)

    cursorUp 1
    eraseLine()

    if status != 0:
      rad_abort(&"""{status:<8}{ceras.nom:24}{(
        case ceras.ver
        of RAD_TOOL_GIT:
          ceras.cmt
        else:
          ceras.ver
      ):24}""")

    rad_ceras_print_footer(idx, ceras.nom,
      case ceras.ver
      of RAD_TOOL_GIT:
        ceras.cmt
      else:
        ceras.ver
    , RAD_PRINT_INSTALL)

proc rad_ceras_remove*(cerata: openArray[string]) =
  let cluster = rad_ceras_check(cerata)

  rad_ceras_print_header()

  for idx, nom in cluster:
    let ceras = rad_ceras_parse(nom)

    rad_ceras_print_content(idx, ceras.nom,
      case ceras.ver
      of RAD_TOOL_GIT:
        ceras.cmt
      else:
        ceras.ver
    , RAD_PRINT_REMOVE)

    let sum = RAD_PATH_RAD_CACHE_VENOM / ceras.nom / RAD_FILE_SUM

    for line in lines(sum):
      removeFile(DirSep & line.split()[2])

    cursorUp 1
    eraseLine()

    rad_ceras_print_footer(idx, ceras.nom,
      case ceras.ver
      of RAD_TOOL_GIT:
        ceras.cmt
      else:
        ceras.ver
    , RAD_PRINT_REMOVE)

proc rad_ceras_search*(pattern: openArray[string]) =
  var cerata: seq[string]

  for file in walkDir(RAD_PATH_RAD_LIB_CLUSTERS_GLAUCUS, relative = true, skipSpecial = true):
    for nom in pattern:
      if nom.toLowerAscii() in file[1]:
        cerata.add(file[1])

  sort(cerata)

  rad_ceras_print(cerata)
