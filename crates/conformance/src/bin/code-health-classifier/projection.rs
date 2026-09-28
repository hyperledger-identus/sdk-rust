//! Monotonic test-span merge and authored-line projection.

use std::ops::Range;

fn merge_ranges(mut spans: Vec<Range<usize>>) -> Vec<Range<usize>> {
    spans.sort_by_key(|span| (span.start, span.end));
    let mut merged: Vec<Range<usize>> = Vec::new();
    for span in spans {
        if let Some(last) = merged.last_mut()
            && span.start <= last.end
        {
            last.end = last.end.max(span.end);
        } else {
            merged.push(span);
        }
    }
    merged
}

#[derive(Debug, Default, Eq, PartialEq)]
pub(super) struct ProjectionWork {
    pub(super) authored_positions: usize,
    pub(super) span_advances: usize,
}

fn project_lines_with_work(source: &str, spans: Vec<Range<usize>>) -> (Vec<usize>, ProjectionWork) {
    let spans = merge_ranges(spans);
    let mut result = Vec::new();
    let mut work = ProjectionWork::default();
    let mut offset = 0;
    let mut span_index = 0;
    for (index, line) in source.split_inclusive('\n').enumerate() {
        let mut authored = false;
        let mut covered = true;
        for (line_offset, character) in line.char_indices() {
            if character.is_whitespace() {
                continue;
            }
            authored = true;
            work.authored_positions += 1;
            let position = offset + line_offset;
            while span_index < spans.len() && spans[span_index].end <= position {
                span_index += 1;
                work.span_advances += 1;
            }
            if spans
                .get(span_index)
                .is_none_or(|span| !span.contains(&position))
            {
                covered = false;
                break;
            }
        }
        if authored && covered {
            result.push(index + 1);
        }
        offset += line.len();
    }
    (result, work)
}

pub(super) fn project_lines(source: &str, spans: Vec<Range<usize>>) -> Vec<usize> {
    project_lines_with_work(source, spans).0
}

#[cfg(test)]
pub(super) fn projection_work(source: &str, spans: Vec<Range<usize>>) -> ProjectionWork {
    project_lines_with_work(source, spans).1
}
