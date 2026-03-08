use crate::token::Span;

#[derive(Debug, Clone)]
pub struct SourceMap {
    line_starts: Vec<usize>,
    source_len: usize,
}

impl SourceMap {
    pub fn new(source: &str) -> Self {
        let mut line_starts = vec![0];
        for (idx, ch) in source.char_indices() {
            if ch == '\n' {
                line_starts.push(idx + 1);
            }
        }
        Self {
            line_starts,
            source_len: source.len(),
        }
    }

    pub fn offset_to_position(&self, offset: usize) -> (u32, u32) {
        let offset = offset.min(self.source_len);
        let line = self
            .line_starts
            .partition_point(|start| *start <= offset)
            .saturating_sub(1);
        let line_start = self.line_starts[line];
        ((line as u32), (offset - line_start) as u32)
    }

    pub fn position_to_offset(&self, line: u32, character: u32) -> usize {
        let line_idx = (line as usize).min(self.line_starts.len().saturating_sub(1));
        let line_start = self.line_starts[line_idx];
        let line_end = self
            .line_starts
            .get(line_idx + 1)
            .copied()
            .unwrap_or(self.source_len);
        (line_start + character as usize).min(line_end)
    }

    pub fn fallback_span(&self) -> Span {
        Span {
            start: 0,
            end: self.source_len.min(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SourceMap;

    #[test]
    fn maps_offsets_to_lines() {
        let map = SourceMap::new("a\nbc\n");
        assert_eq!(map.offset_to_position(0), (0, 0));
        assert_eq!(map.offset_to_position(2), (1, 0));
    }

    #[test]
    fn maps_positions_to_offsets() {
        let map = SourceMap::new("abc\ndef");
        assert_eq!(map.position_to_offset(1, 2), 6);
    }
}
