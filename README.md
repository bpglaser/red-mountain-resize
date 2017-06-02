# red-mountain-resize
An image resize tool utilizing seam carving.

### Roadmap
- [x] Energy grid calculation
- [x] Image Reduction
- [x] UX finalized
- [x] Image Enlargement
- [x] Reduction/Enlargement in both directions
- [ ] Investigate multithreading options via Rayon.

### Example

Original:

<img src="images/castle.jpg" width="512">

Seams selected:

<img src="images/debug.png" width="512">

100 seams removed:

<img src="images/shrink.jpg" width="487">

100 seams added:

<img src="images/grow.jpg" width="537">
