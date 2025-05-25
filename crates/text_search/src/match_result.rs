#[derive(Debug, Clone)]
pub struct MatchResult {
    pub content: String,
    pub range: TextRange,
    pub capture: CaptureInfo,
}

#[derive(Debug, Clone)]
pub struct CaptureInfo {
    pub content: String,
    pub range: TextRange,
}

#[derive(Debug, Clone)]
pub struct TextRange {
    pub start: usize,
    pub end: usize,
}

impl TextRange {
    pub fn get_start_line_col(&self, content: &str) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;
        let mut count = 0;

        for l in content.lines() {
            let len = l.len() + 1;
            if count + len > self.start {
                col = self.start - count + 1;
                break;
            }
            count += len;
            line += 1;
        }

        (line, col)
    }

    pub fn get_end_line_col(&self, content: &str) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;
        let mut count = 0;

        for l in content.lines() {
            let len = l.len() + 1;
            if count + len > self.end {
                col = self.end - count + 1;
                break;
            }
            count += len;
            line += 1;
        }

        (line, col)
    }
}
