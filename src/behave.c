#include <getopt.h>
#include <string.h>

#include "behave.h"
#include "help.h"

//There is no REPRODUCE behavior as it's identical to BOOTSTRAP
enum {
  BINARY, BOOTSTRAP, CLEAN, DISTCLEAN, ENVENOMATE
} behaviors;

unsigned int rad_behave() {
  unsigned int behave = 0;

  if (strcmp(optarg, "binary") == 0) {
    behave = BINARY;
    rad_behave_binary_help();
    return 1;
  } else if (strcmp(optarg, "bootstrap") == 0 || strcmp(optarg, "reproduce") == 0) {
    behave = BOOTSTRAP;
    rad_behave_bootstrap_help();
    return 1;
  } else if (strcmp(optarg, "clean") == 0) {
    behave = CLEAN;
  } else if (strcmp(optarg, "distclean") == 0) {
    behave = DISTCLEAN;
  } else if (strcmp(optarg, "envenomate") == 0) {
    behave = ENVENOMATE;
    rad_behave_envenomate_help();
    return 1;
  } else if (strcmp(optarg, "-h") == 0 || strcmp(optarg, "help") == 0) {
    rad_behave_help();
    return 0;
  } else {
    rad_behave_help();
    return 1;
  }
}
