#[derive(Debug, Clone, Copy)]
pub struct PaletteSize(pub u8);

// Palette, her PalettedStorage'ın sahip olduğu bir değer paletidir.
pub struct Palette {
    last: u32,
    last_index: i16,
    size: PaletteSize,
    values: Vec<u32>,
}

impl Palette {
    // new_palette, belirli bir boyut ve eklenen değerlerin dilimini içeren yeni bir Palette döner.
    pub fn new(size: PaletteSize, values: Vec<u32>) -> Self {
        Self {
            last: u32::MAX,
            last_index: -1,
            size,
            values,
        }
    }

    // len, Palette'deki benzersiz değerlerin sayısını döner.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    // add, Palette'ye bir değer ekler.
    pub fn add(&mut self, v: u32) -> (i16, bool) {
        let index = self.values.len() as i16;
        self.values.push(v);

        if self.needs_resize() {
            self.increase_size();
            (index, true)
        } else {
            (index, false)
        }
    }

    // replace, her bir değeri günceller.
    pub fn replace<F>(&mut self, f: F)
    where
        F: Fn(u32) -> u32,
    {
        self.last = u32::MAX;
        for value in &mut self.values {
            *value = f(*value);
        }
    }

    // index, verilen değerin indeksini arar.
    pub fn index(&mut self, runtime_id: u32) -> i16 {
        if runtime_id == self.last {
            return self.last_index;
        }
        self.index_slow(runtime_id)
    }

    // index_slow, değerleri döngü ile arar.
    pub fn index_slow(&mut self, runtime_id: u32) -> i16 {
        for (i, &value) in self.values.iter().enumerate() {
            if value == runtime_id {
                self.last = runtime_id;
                self.last_index = i as i16;
                return self.last_index;
            }
        }
        -1
    }

    // value, belirli bir indeksteki değeri döner.
    pub fn value(&self, i: usize) -> u32 {
        self.values[i]
    }

    // needs_resize, boyut değişikliğine ihtiyaç olup olmadığını kontrol eder.
    pub fn needs_resize(&self) -> bool {
        self.values.len() > (1 << self.size.0)
    }

    // increase_size, boyutu artırır.
    pub fn increase_size(&mut self) {
        self.size.0 += 1; // Gerekirse boyutu artır
    }
}

// Palette boyutları
pub const SIZES: [PaletteSize; 9] = [
    PaletteSize(0), PaletteSize(1), PaletteSize(2),
    PaletteSize(3), PaletteSize(4), PaletteSize(5),
    PaletteSize(6), PaletteSize(8), PaletteSize(16),
];

// padded, paletin belirli boyutlarda olup olmadığını kontrol eder.
impl PaletteSize {
    fn padded(self) -> bool {
        self.0 == 3 || self.0 == 5 || self.0 == 6
    }
}

// palette_size_for, verilen n için uygun bir paletteSize bulur.
pub fn palette_size_for(n: usize) -> PaletteSize {
    for &size in &SIZES {
        if n <= (1 << size.0) as usize {
            return size;
        }
    }
    PaletteSize(0) // Asla olmaması gereken bir durum.
}

// uint32s, belirtilen palet boyutuyla temsil edilen bir depolama için gereken uint32 sayısını döner.
impl PaletteSize {
    pub fn uint32s(self) -> usize {
        if self.0 == 0 {
            return 0;
        }
        let indices_per_u32 = 32 / self.0 as usize;
        let mut uint32_count = 4096 / indices_per_u32;
        if self.padded() {
            uint32_count += 1;
        }
        uint32_count
    }
}
