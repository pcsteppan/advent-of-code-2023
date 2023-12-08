use std::ops::Range;

fn main() {
    println!("Hello, world!");
}

struct Layer {
    range_maps: Vec<RangeMap>
}

impl Layer {
    fn compose(self, other: &Layer) -> Layer {
        Layer {
            range_maps: self.range_maps.into_iter().flat_map(|range_map| range_map.transform(other)).collect() 
        }
    }
}

#[derive(Debug)]
struct RangeMap {
    range: Range<i64>,
    offset: i64
}

impl RangeMap {
    fn transform(self, layer: &Layer) -> Vec<RangeMap> {
        let mut stack = vec![self];
        let mut results: Vec<RangeMap> = vec![];

        while !stack.is_empty() {
            let curr = stack.pop().unwrap();
            println!("stack: {:?}\r\ncurr: {:?}", stack, curr);

            for curr_map in layer.range_maps.as_slice() {
                let combined_offset = curr_map.offset + curr.offset;

                let left_inside = curr_map.range.contains(&curr.range.start);
                let right_inside = curr_map.range.contains(&curr.range.end);
                let has_no_overlap = curr.range.end < curr_map.range.start || curr.range.start > curr_map.range.end;

                if has_no_overlap {
                    continue;        
                }

                match (left_inside, right_inside) {
                    (false, false) => {
                        // split into three ranges
                        stack.push(RangeMap { range: curr.range.start..curr_map.range.start-1, offset: curr.offset });
                        results.push(RangeMap { range: curr_map.range.clone(), offset: combined_offset });
                        stack.push(RangeMap { range: curr_map.range.end+1..curr.range.end, offset: curr.offset });
                        break;
                    },
                    (false, true) => {
                        // split into two ranges
                        stack.push(RangeMap { range: curr.range.start..curr_map.range.start-1, offset: curr.offset });
                        results.push(RangeMap { range: curr_map.range.start..curr.range.end, offset: combined_offset });
                        break;
                    },
                    (true, false) => {
                        // split into two ranges
                        results.push(RangeMap { range: curr.range.start..curr_map.range.end, offset: combined_offset });
                        stack.push(RangeMap { range: curr_map.range.end+1..curr.range.end, offset: curr.offset });
                        break;
                    },
                    (true, true) => {
                        // range stays the same, just update the offset
                        results.push(RangeMap { range: curr.range.clone(), offset: combined_offset});
                        break;
                    }
                }
            }


        }

        results
    }
}

#[cfg(test)]
mod test {
    use crate::{RangeMap, Layer};

    #[test]
    fn test_transform() {
        let m = RangeMap {
            range: 10..20,
            offset: 1
        };

        let layer = Layer {
            range_maps: vec![
                RangeMap {
                    range: 0..15,
                    offset: 2
                }
            ]
        };

        let res = m.transform(&layer);

        println!("{:?}", res);
    }
}
