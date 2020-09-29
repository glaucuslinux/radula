#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h> 
#include <unistd.h>

#include "behave.h"
#include "ceras.h"
#include "help.h"
#include "radula.h"

unsigned int ccache = 0;
unsigned int parallel = 1;
unsigned int quiet = 0;

int main(int argc, char** argv) {
  int opt;
  struct stat st;

  char GLAD[PATH_MAX];

  if(stat("/usr/cerata", &st) == 0 && S_ISDIR(st.st_mode)) {
    strcpy(GLAD, "/usr");
  } else {
    getcwd(GLAD, sizeof(GLAD));
  }

  char CERD[PATH_MAX];
  strcpy(CERD, GLAD);
  strcat(CERD, "/cerata");

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
        quiet = 1;
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
