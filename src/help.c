#include <stdio.h>

#include "help.h"
#include "radula.h"

void rad_open(const char* rad_string) {
  if (quiet == 0)
    printf("\033[0;34m~~ \033[0m");

  printf("%s\n", rad_string);
}

void rad_close(const char* rad_string) {
  if (quiet == 0)
    printf("\033[0;31m-- \033[0m");

  printf("%s\n", rad_string);
}

void rad_version() {
  rad_open("Copyright (c) 2020, Firas Khalil Khana");
  rad_open("Distributed under the terms of the ISC License");
  rad_open("");
  rad_open("radula version 2.0 (genome species)");
}

void rad_help() {
  rad_version();

  rad_open("");
  rad_open("Usage:");
  rad_open("\tradula [ Options ]");
  rad_open("");
  rad_open("Options:");
  rad_open("\t-b \tPerform any of the following behaviors:");
  rad_open("\t   \tbinary, bootstrap | reproduce, envenomate");
  rad_open("");
  rad_open("\t-c \tDisplay ceras information");
  rad_open("\t-g \tDisplay current genome");
  rad_open("\t-h \tDisplay this help message");
  rad_open("\t-q \tDecrease the verbosity level and don't prefix");
  rad_open("\t   \tthe output with ~~");
  rad_open("");
  rad_open("\t-s \tDisplay current species");
  rad_open("\t-v \tDisplay current version number");
}

void rad_behave_help() {
  rad_version();

  rad_open("");
  rad_open("Usage:");
  rad_open("\tradula -b [ Options ]");
  rad_open("");
  rad_open("Options:");
  rad_open("\tbootstrap \tPerform bootstrap behavior");
  rad_open("\tenvenomate\tPerform envenomate behavior");
  rad_open("");
  rad_open("\t-h, help  \tDisplay this help message");
  rad_open("");
  rad_open("\tbinary    \tPerform binary behavior");
  rad_open("\treproduce \tSame as bootstrap");
  rad_open("");
  rad_open("\t-q, quiet \tDecrease the verbosity level and don't prefix");
  rad_open("\t          \tthe output with ~~");
}

void rad_behave_binary_help() {
  rad_version();

  rad_open("");
  rad_open("Usage:");
  rad_open("\tradula -b binary [ Options ] [ Cerata ]");
  rad_open("");
  rad_open("Options:");
  rad_open("\tdecyst    \tRemove binary cerata without preserving their cysts");
  rad_open("");
  rad_open("\t-h, help  \tDisplay this help message");
  rad_open("");
  rad_open("\tinstall   \tInstall binary cerata (default)");
  rad_open("");
  rad_open("\t-q, quiet \tDecrease the verbosity level and don't prefix");
  rad_open("\t          \tthe output with ~~");
  rad_open("");
  rad_open("\tremove    \tRemove binary cerata while preserving their cyst(s)");
  rad_open("\tsearch    \tSearch for binary cerata within the remote repositories");
  rad_open("\tupgrade   \tUpgrade binary cerata");
}

void rad_behave_bootstrap_help() {
  rad_version();

  rad_open("");
  rad_open("Usage:");
  rad_open("\tradula -b [ bootstrap | reproduce ] [ Options ]");
  rad_open("");
  rad_open("Options:");
  rad_open("\tclean     \tClean up while preserving sources and backups");
  rad_open("\tdistclean \tClean up everything");
  rad_open("");
  rad_open("\t-h,   help\tDisplay this help message");
  rad_open("");
  rad_open("\timage     \tCreate a .img file of the glaucus system");
  rad_open("\tlist      \tList supported genomes and species");
  rad_open("");
  rad_open("\t-q, quiet \tDecrease the verbosity level and don't prefix");
  rad_open("\t          \tthe output with ~~");
  rad_open("");
  rad_open("\trequire   \tCheck if host has all required packages");
  rad_open("");
  rad_open("\trelease   \tRelease a compressed tarball of the toolchain");
  rad_open("");
  rad_open("\ttoolchain \tBootstrap a cross compiler toolchain");
  rad_open("\tcross     \tBootstrap a cross compiled glaucus system");
}

void rad_behave_envenomate_help() {
  rad_version();

  rad_open("");
  rad_open("Usage:");
  rad_open("\tradula -b envenomate [ Options ] [ Cerata ]");
  rad_open("");
  rad_open("Options:");
  rad_open("\tdecyst    \tRemove cerata without preserving their cyst(s)");
  rad_open("");
  rad_open("\t-h, help  \tDisplay this help message");
  rad_open("");
  rad_open("\tinstall   \tInstall cerata from source (default)");
  rad_open("");
  rad_open("\t-q, quiet \tDecrease the verbosity level and don't prefix");
  rad_open("\t          \tthe output with ~~");
  rad_open("");
  rad_open("\tremove    \tRemove cerata while preserving their cyst(s)");
  rad_open("\tsearch    \tSearch for cerata within the cerata directory");
  rad_open("\tupgrade   \tUpgrade cerata");
}

void rad_ceras_help() {
  rad_version();

  rad_open("");
  rad_open("Usage:");
  rad_open("\tradula -c [ Options ] [ Cerata ]");
  rad_open("");
  rad_open("Options:");
  rad_open("\tc, cnt, concentrate, concentrates\tDisplay cerata concentrate(s)");
  rad_open("");
  rad_open("\t-h help                          \tDisplay this help message");
  rad_open("");
  rad_open("\tl, lic, license, licenses        \tDisplay cerata license(s)");
  rad_open("\tn, nom, name                     \tDisplay cerata name(s)");
  rad_open("");
  rad_open("\t-q, quiet                        \tDecrease the verbosity level and don't prefix");
  rad_open("\t                                 \tthe output with ~~");
  rad_open("");
  rad_open("\ts, sum, checksum, sha512sum      \tDisplay cerata sha512sum(s)");
  rad_open("\tu, url, source                   \tDisplay cerata source(s)");
  rad_open("\tv, ver, version                  \tDisplay cerata version(s)");
  rad_open("\ty, cys, cyst, cysts              \tDisplay cerata cyst(s)");
}
