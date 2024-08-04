pub fn chunk_data(data: &[u8], chunk_size: usize) -> Vec<Vec<u8>> {
    data.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect()
}

