use crate::token::Span;

/// A rendered source snippet around a byte span.
#[derive(Debug)]
pub struct Snippet {
    pub line: usize,
    pub column: usize,
    pub source_line: String,
    pub marker_line: String,
}

/// Builds a one-line source snippet and marker for the given byte span.
pub fn render_snippet(src: &str, span: &Span) -> Snippet {
    let mut line_start = 0usize;
    let mut line_no = 1usize;
    for (idx, ch) in src.char_indices() {
        if idx >= span.start {
            break;
        }
        if ch == '\n' {
            line_no += 1;
            line_start = idx + 1;
        }
    }

    let line_end = src[line_start..]
        .find('\n')
        .map(|rel| line_start + rel)
        .unwrap_or(src.len());
    let source_line = src[line_start..line_end].to_string();

    let column = src[line_start..span.start].chars().count() + 1;
    let width = {
        let end = span.end.max(span.start + 1).min(line_end);
        let w = src[span.start..end].chars().count();
        if w == 0 { 1 } else { w }
    };

    let mut marker_line = String::new();
    marker_line.push_str(&" ".repeat(column.saturating_sub(1)));
    marker_line.push_str(&"^".repeat(width));

    Snippet {
        line: line_no,
        column,
        source_line,
        marker_line,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_snippet_points_to_expected_column() {
        let src = "fn main() {\n  return 42;\n}";
        let start = src.find("42").expect("literal missing");
        let span = Span {
            start,
            end: start + 2,
        };
        let snippet = render_snippet(src, &span);
        assert_eq!(snippet.line, 2);
        assert_eq!(snippet.column, 10);
        assert_eq!(snippet.source_line, "  return 42;");
        assert_eq!(snippet.marker_line, "         ^^");
    }
}
