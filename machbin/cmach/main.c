#include <stdio.h>
#include <stdint.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdlib.h>
#include <errno.h>

#include "parse_header.h"

int main(int argc, char **argv)
{
    struct stat stat_buf = {0};
    int fd = -1;
    int result = 0;
    uint8_t *file_data = NULL;
    ssize_t bytes_read = 0;

    if (argc < 2)
    {
        printf("Usage: %s <mach-o path>\n", argv[1]);
    }

    // Open targeted elf
    fd = open(argv[1], O_RDONLY);
    if (-1 == fd)
    {
        printf("[-] Error opening file. errno: %d\n", errno);
    }
    // Pull file stats to get size
    result = fstat(fd, &stat_buf);
    if (-1 == result)
    {
        printf("[-] Error file stat. errno: %d\n", errno);
    }
    // Allocate zero initialize memory to store binary
    file_data = calloc(stat_buf.st_size, sizeof(uint8_t));
    if (NULL == file_data)
    {
        printf("[-] Error allocating buffer. errno: %d\n", errno);
    }
    // Loop reading until we've read all bytes
    while (bytes_read != stat_buf.st_size)
    {
        bytes_read = read(fd, (uint8_t *)file_data + bytes_read, stat_buf.st_size - bytes_read);
        if (bytes_read != stat_buf.st_size)
        {
            printf("[-] Error reading file. errno: %d\n", errno);
        }
        printf("[*] Read: %zd bytes\n", bytes_read);
    }
    close(fd);
    fd = -1;

    return parse_all(file_data);
}