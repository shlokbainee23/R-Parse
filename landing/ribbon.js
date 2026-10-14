/**
 * R-PARSE Continuous Visual Thread (Paper Ribbon)
 * 
 * Draws a subtle, background-only procedural thread that travels strictly behind
 * editorial content in outer margin gutters, tracing the investigation without
 * ever crossing or obscuring headlines, body copy, or data cards.
 */

(function () {
  'use strict';

  const canvas = document.getElementById('ribbon-canvas');
  if (!canvas) return;

  const ctx = canvas.getContext('2d');
  let width = 0;
  let height = 0;
  let dpr = window.devicePixelRatio || 1;

  // Reduced motion check
  const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  if (prefersReducedMotion) {
    canvas.style.display = 'none';
    return;
  }

  function resize() {
    width = window.innerWidth;
    height = window.innerHeight;
    canvas.width = width * dpr;
    canvas.height = height * dpr;
    ctx.scale(dpr, dpr);
  }

  window.addEventListener('resize', resize, { passive: true });
  resize();

  // Anchors routed strictly through outer gutters & negative space
  function getAnchors() {
    const isMobile = width < 768;

    let pts;
    if (isMobile) {
      // On mobile: strictly along far-right gutter (95-97%), never crossing text column
      pts = [
        { x: width * 0.96, y: -40 },
        { x: width * 0.94, y: height * 0.25 },
        { x: width * 0.97, y: height * 0.50 },
        { x: width * 0.95, y: height * 0.75 },
        { x: width * 0.96, y: height * 1.05 }
      ];
    } else {
      // On desktop: routed through outer right gutter and negative margins
      pts = [
        { x: width * 0.93, y: -40 },
        { x: width * 0.95, y: height * 0.24 },
        { x: width * 0.91, y: height * 0.48 },
        { x: width * 0.94, y: height * 0.74 },
        { x: width * 0.92, y: height * 1.05 }
      ];
    }

    return pts;
  }

  let waveOffset = 0;

  function render() {
    ctx.clearRect(0, 0, width, height);

    const pts = getAnchors();
    waveOffset += 0.007;

    ctx.save();
    ctx.lineCap = 'round';
    ctx.lineJoin = 'round';

    // Outer subtle ambient glow
    ctx.shadowColor = 'rgba(255, 90, 31, 0.15)';
    ctx.shadowBlur = 8;

    // Primary signal-orange thread (restrained background thread)
    ctx.beginPath();
    ctx.moveTo(pts[0].x, pts[0].y);

    for (let i = 1; i < pts.length; i++) {
      const prev = pts[i - 1];
      const curr = pts[i];
      const wobble = Math.sin(waveOffset + i * 1.1) * 5;
      const cpx = (prev.x + curr.x) / 2 + wobble;
      const cpy = (prev.y + curr.y) / 2;
      ctx.quadraticCurveTo(cpx, cpy, curr.x + wobble * 0.4, curr.y);
    }

    ctx.strokeStyle = 'rgba(255, 90, 31, 0.42)';
    ctx.lineWidth = 2.0;
    ctx.stroke();

    // Secondary fine lilac companion strand
    ctx.beginPath();
    ctx.moveTo(pts[0].x - 10, pts[0].y - 8);
    for (let i = 1; i < pts.length; i++) {
      const prev = pts[i - 1];
      const curr = pts[i];
      const wobble = Math.sin(waveOffset * 1.1 + i * 1.3) * 4;
      const cpx = (prev.x + curr.x) / 2 - 10 + wobble;
      const cpy = (prev.y + curr.y) / 2;
      ctx.quadraticCurveTo(cpx, cpy, curr.x - 10, curr.y);
    }
    ctx.strokeStyle = 'rgba(184, 166, 255, 0.28)';
    ctx.lineWidth = 1.0;
    ctx.stroke();

    // Subtle forensic pulse bead travelling along the line
    const beadProgress = (waveOffset * 0.25) % 1;
    const beadIndex = Math.min(Math.floor(beadProgress * (pts.length - 1)), pts.length - 2);
    const p1 = pts[beadIndex];
    const p2 = pts[beadIndex + 1];
    const t = (beadProgress * (pts.length - 1)) - beadIndex;
    const bx = p1.x + (p2.x - p1.x) * t;
    const by = p1.y + (p2.y - p1.y) * t;

    ctx.beginPath();
    ctx.arc(bx, by, 3, 0, Math.PI * 2);
    ctx.fillStyle = 'rgba(255, 90, 31, 0.75)';
    ctx.shadowColor = '#FF5A1F';
    ctx.shadowBlur = 10;
    ctx.fill();

    ctx.restore();

    requestAnimationFrame(render);
  }

  requestAnimationFrame(render);
})();
