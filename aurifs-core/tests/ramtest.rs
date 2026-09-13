// test wasn't made by me 

use aurifs_core::{Ramdisk, SECTOR_SIZE};

/// Générateur LCG basique pour produire des octets pseudo-aléatoires en no_std.
struct SimpleLcg {
    state: u64,
}

impl SimpleLcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u8(&mut self) -> u8 {
        // Formule LCG (Knuth) : X_{n+1} = (a * X_n + c) mod m
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.state >> 56) as u8
    }
}

#[test]
fn test_ram_disk_integrity() {
    const SECTOR_COUNT: usize = 16;
    let mut disk = Ramdisk::new(SECTOR_COUNT, SECTOR_SIZE);

    // 1. Remplissage du disque avec un motif pseudo-aléatoire
    let mut rng_write = SimpleLcg::new(0xDEADBEEF);
    let mut write_buf = [0u8; SECTOR_SIZE];

    for sector in 0..SECTOR_COUNT {
        for byte in write_buf.iter_mut() {
            *byte = rng_write.next_u8();
        }
        disk.write_sector(sector, &write_buf).expect("Écriture échouée");
    }

    // 2. Relecture et vérification octet par octet avec la même graine (seed)
    let mut rng_read = SimpleLcg::new(0xDEADBEEF);
    let mut read_buf = [0u8; SECTOR_SIZE];

    for sector in 0..SECTOR_COUNT {
        disk.read_sector(sector, &mut read_buf).expect("Lecture échouée");

        for (i, &read_byte) in read_buf.iter().enumerate() {
            let expected_byte = rng_read.next_u8();
            assert_eq!(
                read_byte, expected_byte,
                "Incohérence au secteur {}, octet {}",
                sector, i
            );
        }
    }
}