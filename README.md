# red-mountain-resize
An image resize tool utilizing seam carving.

### Roadmap
- [x] Energy grid calculation
- [x] Image Reduction
- [ ] UX finalized
- [ ] Image Enlargement
- [ ] Reduction/Enlargement in both directions
- [ ] Investigate multithreading options via Rayon.

### Example

Before:

<img src="images/castle.jpg" width="512">

100 seams removed:

<img src="images/debug.png" width="512">

After:

<img src="images/output.jpg" width="487">
