/**
 * R-PARSE Final Choreography — From Source to Structure Assembly
 * 
 * Pinned scroll-scrubbed assembly of R program syntax nodes, column provenance,
 * and unified dependencies into the final evidentiary architecture.
 * Visibly converges from dispersed program elements into a coherent analytical
 * structure centered on df$total, holds in a completely settled readable state,
 * and intentionally transitions into the orange finale without any dark gaps,
 * harsh horizontal lines, or clipped headlines.
 */

(function () {
  'use strict';

  const stage = document.getElementById('pinned-assembly-stage');
  const canvas = document.getElementById('assembly-3d-canvas');
  const fallback = document.getElementById('assembly-fallback');
  const headerBlock = document.getElementById('assembly-header-block');
  if (!stage || !canvas) return;

  const ctx = canvas.getContext('2d');
  if (!ctx) {
    if (fallback) fallback.style.display = 'block';
    return;
  }

  const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  if (prefersReducedMotion) {
    if (fallback) fallback.style.display = 'block';
    canvas.style.display = 'none';
    return;
  }

  let width = 0;
  let height = 0;
  let dpr = window.devicePixelRatio || 1;

  function resize() {
    const parent = canvas.parentElement;
    if (!parent) return;
    width = parent.clientWidth || window.innerWidth;
    height = parent.clientHeight || (window.innerHeight - 68);

    canvas.width = Math.floor(width * dpr);
    canvas.height = Math.floor(height * dpr);
    canvas.style.width = width + 'px';
    canvas.style.height = height + 'px';

    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.scale(dpr, dpr);
  }

  window.addEventListener('resize', resize, { passive: true });
  resize();

  // Define the structural assembly components with relative bounds
  // Responsive offsets scaled by canvas size so elements NEVER leave the viewport
  function getPieces(w, h) {
    const isSmall = w < 768;
    const isShort = h < 720;
    const sx = isSmall ? 0.55 : (w < 1100 ? 0.78 : 1.0);
    const sy = isSmall ? 0.58 : (isShort ? 0.72 : 1.0);

    return [
      // 1. Target node df$total (center-right focus, highlighted orange)
      {
        id: 'target-total',
        label: 'df$total (v1) [20, 60]',
        sub: 'KNOWN_STATIC · GROUND TRUTH',
        color: '#FF5A1F',
        textColor: '#FFFFFF',
        type: 'target',
        start: { x: 180 * sx, y: -100 * sy, rot: 0.15, scale: 0.9, opacity: 0.75 },
        target: { x: 170 * sx, y: 35 * sy, rot: 0, scale: 1.1, opacity: 1.0 },
        w: isSmall ? 180 : 210, h: isSmall ? 48 : 54
      },
      // 2. Input column price
      {
        id: 'col-price',
        label: 'price (v0) [10, 20]',
        sub: 'source column',
        color: '#EEEBE4',
        textColor: '#121212',
        type: 'input',
        start: { x: -240 * sx, y: -120 * sy, rot: -0.2, scale: 0.85, opacity: 0.75 },
        target: { x: -190 * sx, y: -20 * sy, rot: 0, scale: 1.0, opacity: 1.0 },
        w: isSmall ? 150 : 175, h: isSmall ? 42 : 46
      },
      // 3. Input column quantity
      {
        id: 'col-quantity',
        label: 'quantity (v0) [2, 3]',
        sub: 'source column',
        color: '#EEEBE4',
        textColor: '#121212',
        type: 'input',
        start: { x: -220 * sx, y: 130 * sy, rot: 0.18, scale: 0.85, opacity: 0.75 },
        target: { x: -190 * sx, y: 90 * sy, rot: 0, scale: 1.0, opacity: 1.0 },
        w: isSmall ? 150 : 175, h: isSmall ? 42 : 46
      },
      // 4. Operator node *
      {
        id: 'op-mult',
        label: '× (mutate)',
        sub: 'vectorized multiply',
        color: '#B8A6FF',
        textColor: '#121212',
        type: 'op',
        start: { x: -15 * sx, y: 140 * sy, rot: -0.15, scale: 0.8, opacity: 0.7 },
        target: { x: -10 * sx, y: 35 * sy, rot: 0, scale: 1.0, opacity: 1.0 },
        w: isSmall ? 110 : 125, h: isSmall ? 38 : 42
      },
      // 5. Statement 1: constructor
      {
        id: 'stmt-1',
        label: 'S1: df <- data.frame(...)',
        sub: 'retained in slice',
        color: '#282832',
        textColor: '#EEEBE4',
        type: 'stmt',
        start: { x: -180 * sx, y: -180 * sy, rot: 0.1, scale: 0.85, opacity: 0.7 },
        target: { x: -190 * sx, y: -95 * sy, rot: 0, scale: 0.95, opacity: 0.9 },
        w: isSmall ? 190 : 225, h: isSmall ? 36 : 40
      },
      // 6. Statement 2: title (unrelated, visibly pruned away)
      {
        id: 'stmt-2',
        label: 'S2: title <- "Q3 report"',
        sub: 'PRUNED FROM SLICE',
        color: '#1C1C22',
        textColor: '#767470',
        type: 'unrelated',
        start: { x: 220 * sx, y: 150 * sy, rot: -0.1, scale: 0.85, opacity: 0.7 },
        target: { x: 310 * sx, y: 150 * sy, rot: 0.15, scale: 0.7, opacity: 0.18 },
        w: isSmall ? 170 : 205, h: isSmall ? 34 : 38
      },
      // 7. Statement 3: assignment
      {
        id: 'stmt-3',
        label: 'S3: df$total <- ...',
        sub: 'criterion statement',
        color: '#282832',
        textColor: '#EEEBE4',
        type: 'stmt',
        start: { x: 190 * sx, y: -180 * sy, rot: -0.1, scale: 0.85, opacity: 0.7 },
        target: { x: 170 * sx, y: -45 * sy, rot: 0, scale: 0.95, opacity: 0.9 },
        w: isSmall ? 180 : 210, h: isSmall ? 36 : 40
      }
    ];
  }

  // Calculate current scroll scrub progress through pinned stage
  function getScrollProgress() {
    const rect = stage.getBoundingClientRect();
    const totalDist = stage.clientHeight - window.innerHeight;
    if (totalDist <= 0) return 0.65; // Default to assembled state if not scrollable
    const current = -rect.top;
    return Math.min(Math.max(current / totalDist, 0), 1);
  }

  function lerp(a, b, t) {
    return a + (b - a) * t;
  }

  function easeInOutCubic(t) {
    return t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2;
  }

  function easeOutCubic(t) {
    return 1 - Math.pow(1 - t, 3);
  }

  let currentProgress = 0;
  let pulsePhase = 0;

  function render() {
    ctx.clearRect(0, 0, width, height);

    // Responsive progress follow synchronized with Lenis without compounding lag
    const targetProgress = getScrollProgress();
    currentProgress += (targetProgress - currentProgress) * 0.35;
    const p = currentProgress;

    // Phase 1 (0.00 to 0.60): Convergence into architecture
    // Phase 2 (0.60 to 0.82): Fully assembled, settled, readable stable state
    // Phase 3 (0.82 to 1.00): Intentional transition into signal orange finale
    const assembleRatio = Math.min(Math.max(p / 0.60, 0), 1.0);
    const t = easeOutCubic(assembleRatio);

    const transRatio = Math.min(Math.max((p - 0.82) / 0.18, 0), 1.0);
    const easeTrans = transRatio * transRatio;

    // Interpolate parent stage background color seamlessly toward signal orange #FF5A1F
    if (transRatio > 0.001) {
      // From #16161A (rgb 22, 22, 26) to #FF5A1F (rgb 255, 90, 31)
      const r = Math.round(lerp(22, 255, easeTrans));
      const g = Math.round(lerp(22, 90, easeTrans));
      const b = Math.round(lerp(26, 31, easeTrans));
      stage.style.backgroundColor = `rgb(${r}, ${g}, ${b})`;
    } else {
      stage.style.backgroundColor = '#16161A';
    }

    // Dissolve section header into warming orange before container unpins
    if (headerBlock) {
      if (transRatio > 0.01) {
        headerBlock.style.opacity = Math.max(0, 1.0 - easeTrans * 1.1).toFixed(3);
        headerBlock.style.transform = `translateY(${-easeTrans * 16}px)`;
      } else {
        headerBlock.style.opacity = '';
        headerBlock.style.transform = '';
      }
    }

    pulsePhase += 0.035;

    // Center point calculated within the canvas stage
    const cx = width / 2;
    // Comfortable vertical centering leaving ample headroom for header
    const cy = Math.max(height * 0.50, 180);

    const pieces = getPieces(width, height);

    // Base background fill and subtle blueprint grid
    ctx.save();
    // Warm background smoothly toward signal orange in transition phase
    if (easeTrans > 0.001) {
      ctx.fillStyle = `rgba(255, 90, 31, ${easeTrans})`;
      ctx.fillRect(0, 0, width, height);
    }

    // Grid disappears as orange finale blooms
    const gridAlpha = (1.0 - easeTrans) * 0.04;
    if (gridAlpha > 0.005) {
      ctx.strokeStyle = `rgba(255, 255, 255, ${gridAlpha})`;
      ctx.lineWidth = 1;
      const gridSize = 44;
      for (let x = (cx % gridSize); x < width; x += gridSize) {
        ctx.beginPath();
        ctx.moveTo(x, 0);
        ctx.lineTo(x, height);
        ctx.stroke();
      }
      for (let y = (cy % gridSize); y < height; y += gridSize) {
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(width, y);
        ctx.stroke();
      }
    }
    ctx.restore();

    // Calculate interpolated positions
    const posMap = {};
    const contentAlpha = Math.max(0, 1.0 - easeTrans);

    pieces.forEach(p => {
      posMap[p.id] = {
        x: cx + lerp(p.start.x, p.target.x, t),
        y: cy + lerp(p.start.y, p.target.y, t),
        rot: lerp(p.start.rot, p.target.rot, t),
        scale: lerp(p.start.scale, p.target.scale, t),
        opacity: lerp(p.start.opacity, p.target.opacity, t) * contentAlpha
      };
    });

    if (contentAlpha > 0.01) {
      // Draw connecting dependency spline lines
      ctx.save();
      const lineAlpha = (0.25 + 0.75 * t) * contentAlpha;

      const price = posMap['col-price'];
      const qty = posMap['col-quantity'];
      const op = posMap['op-mult'];
      const total = posMap['target-total'];

      ctx.lineWidth = 2.0 + t * 0.5;
      ctx.lineCap = 'round';

      // Price -> Op
      ctx.beginPath();
      ctx.moveTo(price.x + 75, price.y);
      ctx.bezierCurveTo(
        price.x + 130, price.y,
        op.x - 70, op.y,
        op.x, op.y
      );
      ctx.strokeStyle = `rgba(255, 90, 31, ${0.75 * lineAlpha})`;
      ctx.stroke();

      // Quantity -> Op
      ctx.beginPath();
      ctx.moveTo(qty.x + 75, qty.y);
      ctx.bezierCurveTo(
        qty.x + 130, qty.y,
        op.x - 70, op.y,
        op.x, op.y
      );
      ctx.strokeStyle = `rgba(255, 90, 31, ${0.75 * lineAlpha})`;
      ctx.stroke();

      // Op -> Total
      ctx.beginPath();
      ctx.moveTo(op.x + 55, op.y);
      ctx.bezierCurveTo(
        op.x + 110, op.y,
        total.x - 85, total.y,
        total.x, total.y
      );
      ctx.strokeStyle = `rgba(255, 90, 31, ${0.9 * lineAlpha})`;
      ctx.lineWidth = 2.5 + t * 1.0;
      ctx.shadowColor = '#FF5A1F';
      ctx.shadowBlur = 10 * lineAlpha;
      ctx.stroke();
      ctx.restore();

      // Draw nodes
      pieces.forEach(p => {
        const pos = posMap[p.id];
        if (pos.opacity <= 0.005) return;

        ctx.save();
        ctx.translate(pos.x, pos.y);
        ctx.rotate(pos.rot);
        ctx.scale(pos.scale, pos.scale);
        ctx.globalAlpha = pos.opacity;

        const hw = p.w / 2;
        const hh = p.h / 2;

        // Glow for target node
        if (p.type === 'target') {
          ctx.shadowColor = '#FF5A1F';
          ctx.shadowBlur = (18 + Math.sin(pulsePhase * 2) * 8) * contentAlpha;
        }

        // Card body
        ctx.fillStyle = p.color;
        ctx.beginPath();
        if (ctx.roundRect) {
          ctx.roundRect(-hw, -hh, p.w, p.h, 6);
        } else {
          ctx.rect(-hw, -hh, p.w, p.h);
        }
        ctx.fill();

        // Card border
        ctx.strokeStyle = p.type === 'target' 
          ? '#FF5A1F' 
          : (p.type === 'unrelated' ? 'rgba(255, 255, 255, 0.08)' : 'rgba(255, 255, 255, 0.22)');
        ctx.lineWidth = p.type === 'target' ? 2 : 1;
        ctx.stroke();

        // Label text
        ctx.shadowBlur = 0;
        ctx.fillStyle = p.textColor;
        ctx.font = '600 11px "JetBrains Mono", monospace';
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';
        ctx.fillText(p.label, 0, p.sub ? -4 : 0);

        // Subtitle if present
        if (p.sub) {
          ctx.fillStyle = p.type === 'target' ? 'rgba(255, 255, 255, 0.85)' : (p.type === 'unrelated' ? '#5A5852' : '#8E8B82');
          ctx.font = '500 8.5px "JetBrains Mono", monospace';
          ctx.fillText(p.sub, 0, 12);
        }

        ctx.restore();
      });

      // Concentric evidence pulse rings when converged
      if (t > 0.4) {
        const ringAlpha = Math.min((t - 0.4) / 0.6, 1) * contentAlpha;
        const totalPos = posMap['target-total'];
        ctx.save();
        ctx.strokeStyle = `rgba(255, 90, 31, ${0.45 * ringAlpha})`;
        ctx.lineWidth = 1.5;

        const ringR = 60 + (pulsePhase * 18) % 52;
        ctx.beginPath();
        ctx.arc(totalPos.x, totalPos.y, ringR, 0, Math.PI * 2);
        ctx.stroke();
        ctx.restore();
      }
    }

    requestAnimationFrame(render);
  }

  requestAnimationFrame(render);
})();
