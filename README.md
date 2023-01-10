# setvision_rs
Rust port of SetVision project

## TODO
- [x] Add basic datastructures
- [x] Unittest everything
- [x] Display a card nicely
- [x] Display a table of card nicely
- [x] Display a solution/set nicely within a table (eg by highlighting)
- [x] Basic logic to check for sets
- [ ] Parse table of cards from cmdline/file input
- [ ] All the computer vision stuff :-)
  - [ ] See https://github.com/LoyVanBeek/SetVision/blob/master/SetVision/Window1.xaml.cs#L42
  - [ ] Convert to grayscale
  - [ ] Do canny edge detection with thresholds: https://github.com/LoyVanBeek/SetVision/blob/master/SetVision/Vision/ContourAnalyzer.cs#L46
  - [ ] Closing on that image with a structuring element: https://github.com/LoyVanBeek/SetVision/blob/master/SetVision/Vision/ContourAnalyzer.cs#L65
  - [ ] Then contour finding
  - [ ] Filter the contours to determine their shape
  - [ ] etc.
