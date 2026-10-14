/**
 * R-PARSE Hero Analytical Instrument Controller (2D Refinement)
 * 
 * Replaces the old 3D perspective scene with a crisp, readable 2D analytical illustration
 * showing the core dependency relationship:
 * price [10, 20] ──┐
 *                  ├── × ──> df$total [20, 60] (KNOWN_STATIC)
 * quantity [2, 3] ─┘
 */

(function () {
  'use strict';

  // Delegate directly to the 2D Analytical Illustration Controller
  const container = document.getElementById('hero-analytical-instrument');
  const svg = document.getElementById('hero-connectors-svg');
  if (!container || !svg) return;

  const nodePrice = document.getElementById('hero-node-price');
  const nodeQty = document.getElementById('hero-node-quantity');
  const nodeOp = document.getElementById('hero-node-op');
  const nodeTotal = document.getElementById('hero-node-total');

  const pathPriceOp = document.getElementById('path-price-op');
  const pathQtyOp = document.getElementById('path-qty-op');
  const pathOpTotal = document.getElementById('path-op-total');
  const trailStatus = document.getElementById('hero-trail-status');

  function updateConnectors() {
    if (!nodePrice || !nodeQty || !nodeOp || !nodeTotal || !svg) return;

    const svgRect = svg.getBoundingClientRect();
    if (svgRect.width === 0 || svgRect.height === 0) return;

    function getAnchor(el, side) {
      const r = el.getBoundingClientRect();
      const x = (side === 'right' ? r.right : r.left) - svgRect.left;
      const y = r.top + r.height / 2 - svgRect.top;
      return { x, y };
    }

    const priceRight = getAnchor(nodePrice, 'right');
    const qtyRight = getAnchor(nodeQty, 'right');
    const opLeft = getAnchor(nodeOp, 'left');
    const opRight = getAnchor(nodeOp, 'right');
    const totalLeft = getAnchor(nodeTotal, 'left');

    function makeBezier(p1, p2) {
      const dx = Math.max(Math.abs(p2.x - p1.x) * 0.55, 30);
      return `M ${p1.x} ${p1.y} C ${p1.x + dx} ${p1.y}, ${p2.x - dx} ${p2.y}, ${p2.x} ${p2.y}`;
    }

    if (pathPriceOp) pathPriceOp.setAttribute('d', makeBezier(priceRight, opLeft));
    if (pathQtyOp) pathQtyOp.setAttribute('d', makeBezier(qtyRight, opLeft));
    if (pathOpTotal) pathOpTotal.setAttribute('d', makeBezier(opRight, totalLeft));
  }

  window.addEventListener('resize', updateConnectors, { passive: true });
  setTimeout(updateConnectors, 60);
  setTimeout(updateConnectors, 250);
  setTimeout(updateConnectors, 700);

  const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

  function runEntrance() {
    if (prefersReducedMotion) {
      container.classList.add('is-ready', 'entrance-complete');
      updateConnectors();
      return;
    }

    container.classList.add('is-ready');

    setTimeout(() => {
      nodePrice?.classList.add('has-entered');
      nodeQty?.classList.add('has-entered');
      updateConnectors();
    }, 150);

    setTimeout(() => {
      pathPriceOp?.classList.add('is-drawing');
      pathQtyOp?.classList.add('is-drawing');
      nodeOp?.classList.add('has-entered');
      updateConnectors();
    }, 450);

    setTimeout(() => {
      pathOpTotal?.classList.add('is-drawing');
      nodeTotal?.classList.add('has-entered');
      updateConnectors();
    }, 800);

    setTimeout(() => {
      container.classList.add('entrance-complete');
      updateConnectors();
    }, 1200);
  }

  const nodes = [
    { el: nodePrice, id: 'price' },
    { el: nodeQty, id: 'quantity' },
    { el: nodeOp, id: 'op' },
    { el: nodeTotal, id: 'total' }
  ];

  function setActiveBranch(activeId) {
    container.classList.remove('focus-price', 'focus-quantity', 'focus-op', 'focus-total');
    if (activeId) {
      container.classList.add(`focus-${activeId}`);
    }

    if (trailStatus) {
      if (activeId === 'price') {
        trailStatus.innerHTML = '<span class="trail-highlight">Branch: df$price</span> ──> [10, 20] feeds df$total directly';
      } else if (activeId === 'quantity') {
        trailStatus.innerHTML = '<span class="trail-highlight">Branch: df$quantity</span> ──> [2, 3] feeds df$total directly';
      } else if (activeId === 'op') {
        trailStatus.innerHTML = '<span class="trail-highlight">Operation: price × quantity</span> ──> Row 1: 20, Row 2: 60';
      } else if (activeId === 'total') {
        trailStatus.innerHTML = '<span class="trail-highlight">Target: df$total [20, 60]</span> ──> Backward slice isolates price & quantity';
      } else {
        trailStatus.innerHTML = '<span class="trail-step">S1 (price, quantity)</span> <span class="trail-arrow">──></span> <span class="trail-step">S3 (df$total &lt;- price * quantity)</span> <span class="trail-arrow">──></span> <span class="trail-result">60 [KNOWN_STATIC]</span>';
      }
    }
  }

  nodes.forEach(({ el, id }) => {
    if (!el) return;
    el.addEventListener('mouseenter', () => setActiveBranch(id));
    el.addEventListener('mouseleave', () => setActiveBranch(null));
    el.addEventListener('click', (e) => {
      e.stopPropagation();
      setActiveBranch(id);
    });
  });

  document.addEventListener('click', (e) => {
    if (!e.target.closest('#hero-analytical-instrument')) {
      setActiveBranch(null);
    }
  });

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => setTimeout(runEntrance, 400));
  } else {
    setTimeout(runEntrance, 400);
  }
})();
