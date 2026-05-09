#ifndef BLOCK_H
#define BLOCK_H

#include <stddef.h>
#include <stdint.h>

struct block_dev{
    uint32_t block_size;
    uint64_t total_blocks;
    int (*read_block)(struct block_dev *dev, uint64_t block_num, void *buffer);
    int (*write_block)(struct block_dev *dev, uint64_t block_num, const void *buffer);
    int (*flush)(struct block_dev *dev);

    void *private_data; 
};

// crée un RAM disk 
struct block_dev *ramdisk_create(uint64_t num_blocks, uint32_t block_size);
void ramdisk_destroy(struct block_dev *dev);

#endif // BLOCK_H