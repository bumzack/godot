use core::fmt;

pub type PathSegment = (f32, f32);

pub struct PathWithTilts {
    path_segments: Vec<PathSegment>,
}

pub trait PathWithTiltsOps {
    fn add(&mut self, segment: PathSegment);
    fn get_segments(&self) -> &Vec<PathSegment>;
}

impl PathWithTiltsOps for PathWithTilts {
    fn add(&mut self, segment: PathSegment) {
        self.path_segments.push(segment);
    }

    fn get_segments(&self) -> &Vec<(f32, f32)> {
        &self.path_segments
    }
}

impl<'a> PathWithTilts {
    pub fn new() -> PathWithTilts {
        PathWithTilts {
            path_segments: Vec::new(),
        }
    }

//    pub fn iter (&self) -> PathWithTiltsIterHelper<'a> {
//        self.into_iter()
//}
}

impl fmt::Debug for PathWithTilts {
fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
for i in self.path_segments.iter() {
writeln ! (f, "len  {:?}   tilt {}", i.0, i.1) ?;
}
writeln ! (f, "")
}
}
//
//
//struct PathWithTiltsIterHelper<'a> {
//    iter:   &'a Vec<PathSegment>,
//    current: usize,
// }
//
//impl<'a> IntoIterator for &'a   PathWithTilts  {
//    type Item =   PathSegment;
//    type IntoIter = PathWithTiltsIterHelper<'a>;
//
//    // note that into_iter() is consuming self
//    fn into_iter(self) -> Self::IntoIter {
//        PathWithTiltsIterHelper {
//            iter: &self.path_segments,
//            current: 0
//        }
//    }
//}
//
//impl<'a>  Iterator for PathWithTiltsIterHelper <'a> {
//    type Item = PathSegment;
//    fn next(&mut self) -> Option<PathSegment> {
//        if self.current < self.path_segments.len() {
//            let e = self.path_segments[self.current];
//            self.current += 1;
//            return Some(e);
//        } else {
//            None
//        }
//    }
//}
//
//
///// Enumerate the pixels of an image.
//pub struct EnumeratePixels<'a, P: Pixel + 'a>
//    where
//        <P as Pixel>::Subpixel: 'a,
//{
//    pixels: Pixels<'a, P>,
//    x: u32,
//    y: u32,
//    width: u32,
//}
//
//impl<'a, P: Pixel + 'a> Iterator for EnumeratePixels<'a, P>
//    where
//        P::Subpixel: 'a,
//{
//    type Item = (u32, u32, &'a P);
//
//    #[inline(always)]
//    fn next(&mut self) -> Option<(u32, u32, &'a P)> {
//        if self.x >= self.width {
//            self.x = 0;
//            self.y += 1;
//        }
//        let (x, y) = (self.x, self.y);
//        self.x += 1;
//        self.pixels.next().map(|p| (x, y, p))
//    }
//}
//

//
//impl  IntoIterator for PathWithTilts  {
//    type Item = PathSegment;
//    type IntoIter = PathWithTiltsIntoIterHelper;
//
//    // note that into_iter() is consuming self
//    fn into_iter(self) -> Self::IntoIter {
//        PathWithTiltsIntoIterHelper {
//            iter: self.path_segments.into_iter(),
//        }
//    }
//}
//// now, implements Iterator trait for the helper struct, to be used by adapters
//impl Iterator for PathWithTilts{
//    type Item = PathSegment;
//
//    // just return the str reference
//    fn next(&mut self) -> Option<Self::Item> {
//        self.iter.next()
//    }
//}
//


//
//// structure helper for non-consuming iterator.
//struct PathWithTiltsIterHelper<'a> {
//    iter: ::std::slice::Iter<'a, &'a PathSegment>,
//}
//
//// implement the IntoIterator trait for a non-consuming iterator. Iteration will
//// borrow the Words structure
//impl<'a> IntoIterator for  &'a PathWithTilts {
//    type Item =   &'a PathSegment;
//    type IntoIter = PathWithTiltsIterHelper<'a>;
//
//    // note that into_iter() is consuming self
//    fn into_iter(self) -> Self::IntoIter {
//        PathWithTiltsIterHelper {
//            iter: self.path_segments.iter(),
//        }
//    }
//}
//
//// now, implements Iterator trait for the helper struct, to be used by adapters
//impl<'a> Iterator for PathWithTiltsIterHelper<'a> {
//    type Item =  PathSegment;
//
//    // just return the str reference
//    fn next(&mut self) -> Option<Self::Item> {
//        self.iter.next()
//    }
//}
