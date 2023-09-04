pub struct ByteStream {
    buffer: Vec<u8>,
    capacity: usize,
    write_pos: usize,
    read_pos: usize,
    eof: bool,
}

impl ByteStream {
    pub fn new(capacity: usize) -> Self {
        ByteStream {
            buffer: Vec::with_capacity(capacity),
            capacity,
            write_pos: 0,
            read_pos: 0,
            eof: false,
        }
    }

    pub fn write(&mut self, byte: u8) -> Result<(), &'static str> {
        if self.write_pos - self.read_pos >= self.capacity {
            Err("Buffer full")
        } else if self.eof {
            Err("Stream has ended. Cannot write more bytes.")
        } else {
            if self.write_pos >= self.buffer.len() {
                self.buffer.push(byte);
            } else {
                self.buffer[self.write_pos] = byte;
            }
            self.write_pos += 1;
            Ok(())
        }
    }

    pub fn read(&mut self) -> Option<u8> {
        if self.read_pos == self.write_pos {
            if self.eof {
                None
            } else {
                Some(self.buffer[self.read_pos])
            }
        } else {
            let byte = self.buffer[self.read_pos];
            self.read_pos += 1;
            Some(byte)
        }
    }

    pub fn end_input(&mut self) {
        self.eof = true;
    }

    pub fn is_eof(&self) -> bool {
        self.eof && (self.read_pos == self.write_pos)
    }
}

fn main() {
    let mut stream = ByteStream::new(3);

    stream.write(65).unwrap(); // 'A'
    stream.write(66).unwrap(); // 'B'
    stream.write(67).unwrap(); // 'C'

    assert_eq!(stream.read(), Some(65)); // 'A'
    assert_eq!(stream.read(), Some(66)); // 'B'
    assert_eq!(stream.read(), Some(67)); // 'C'
    
    stream.end_input();
    assert_eq!(stream.read(), None);
}
