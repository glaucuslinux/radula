#include <getopt.h>
#include <stdio.h>
#include <string.h>

#include "help.h"
#include "radula.h"

int main(int argc, char** argv) {

  int opt;

  char* behave = "ceras";
  char* ceras = "";

  while((opt = getopt(argc, argv, ":b:c:ghsv")) != -1) {
    switch (opt) {
      case 'b':
        if (strcmp(optarg, "binary") == 0) {
          behave = "binary";
          rad_behave_binary_help();
        } else if (strcmp(optarg, "bootstrap") == 0 || strcmp(optarg, "reproduce") == 0) {
          behave = "bootstrap";
        } else if (strcmp(optarg, "envenomate") == 0) {
          behave = "envenomate";
          rad_behave_envenomate_help();
        } else if (strcmp(optarg, "-h") == 0 || strcmp(optarg, "help") == 0) {
          rad_behave_help();
          return 0;
        }
        return 1;
      case 'c':
        rad_ceras_help();
        break;
      case 'g':
        rad_open("genome isn't available now\n");
        break;
      case 'h':
        rad_help();
        return 0;
      case 's':
        rad_open("species isn't available now\n");
        break;
      case 'v':
        rad_version();
        return 0;
      case '?':
        printf("unknown option %c\n", optopt);
        break;
      case ':':
        switch (optopt) {
          case 'b':
            rad_behave_help();
            break;
          case 'c':
            rad_ceras_help();
            break;
        }
        return 1;
      default:
        rad_help();
        return 1;
      }
  }
  rad_help();
  return 1;
}
