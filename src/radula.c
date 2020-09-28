#include <getopt.h>
#include <stdio.h>
#include <string.h>

#include "behave.h"
#include "ceras.h"
#include "help.h"

unsigned int ccache = 0;
unsigned int parallel = 0;
unsigned int sleep = 0;

int main(int argc, char** argv) {
  int opt;

  while((opt = getopt(argc, argv, ":b:c:ghqsv")) != -1) {
    switch (opt) {
      case 'b':
        return rad_behave();
      case 'c':
        return rad_ceras();
      case 'g':
        rad_open("genome isn't available now\n");
        break;
      case 'h':
        rad_help();
        return 0;
      case 'q':
        sleep = 1;
        break;
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
