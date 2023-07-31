#ifndef PARSE_HEADER_H
#define PARSE_HEADER_H

#include <stdint.h>

#include <mach-o/loader.h>

int parse_all(uint8_t *bin_raw);

#endif