#include <argp.h>
#include <stdlib.h>

const char *argp_program_version = "Musicup 0.1.0";
const char *argp_program_bug_address =
    "https://github.com/haruki7049/musicup/issues";

static char doc[] =
    "Musicup -- A server which hosts your music files, compresses "
    "and publishes your music files";

static struct argp argp = {0, 0, 0, doc};

int main(int argc, char **argv) {
  argp_parse(&argp, argc, argv, 0, 0, 0);
  exit(0);
}
