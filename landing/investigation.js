/**
 * R-PARSE — THE INVESTIGATION — Main Interaction & Motion Controller
 * 
 * Orchestrates Lenis smooth scrolling, the evidence loader, header scroll state,
 * scroll-scrubbed pinned sequences, evidence room exhibit tabs, and Studio launch modal.
 */

(function () {
  'use strict';

  // ==========================================================================
  // 1. LENIS SMOOTH SCROLLING (Calibrated for Trackpad & Mouse Wheels)
  // ==========================================================================
  let lenis = null;
  const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

  if (window.Lenis && !prefersReducedMotion) {
    lenis = new window.Lenis({
      lerp: 0.09,              // Frame-rate independent exponential damping (replaces duration-based lag)
      wheelMultiplier: 0.9,    // Controlled speed without runaway acceleration
      touchMultiplier: 1.0,    // Natural 1:1 touch response
      smoothWheel: true,
      infinite: false,
      autoRaf: false,
      // Normalize rapid trackpad momentum bursts to prevent delta accumulation slingshot
      virtualScroll: (data) => {
        if (data.event && data.event.deltaMode === 0) {
          // Trackpad wheel events emit high-frequency pixel deltas
          const maxDelta = 120;
          if (Math.abs(data.deltaY) > maxDelta) {
            data.deltaY = Math.sign(data.deltaY) * maxDelta;
          }
        }
        return true;
      }
    });

    // Make lenis globally accessible for testing and external synchronization
    window.lenis = lenis;

    function raf(time) {
      lenis.raf(time);
      requestAnimationFrame(raf);
    }
    requestAnimationFrame(raf);

    // Provide anchor link smooth scroll support with header offset compensation
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
      anchor.addEventListener('click', function (e) {
        const targetId = this.getAttribute('href');
        if (targetId && targetId !== '#') {
          const targetEl = document.querySelector(targetId);
          if (targetEl) {
            e.preventDefault();
            lenis.scrollTo(targetEl, { offset: -76 });
          }
        }
      });
    });
  }

  // ==========================================================================
  // 2. OPENING SEQUENCE — THE EVIDENCE LOADER
  // ==========================================================================
  const loader = document.getElementById('evidence-loader');
  const counterEl = document.getElementById('loader-counter');
  const glyphQuads = document.querySelectorAll('.glyph-quad');

  function runLoader() {
    if (!loader || !counterEl) return;

    if (prefersReducedMotion) {
      loader.classList.add('is-hidden');
      return;
    }

    // Step 1: Glyph pieces descend with squash-and-settle
    setTimeout(() => {
      glyphQuads.forEach((q, idx) => {
        setTimeout(() => q.classList.add('settled'), idx * 90);
      });
    }, 150);

    // Step 2: Numeric counter progression 000 -> 100
    let count = 0;
    const interval = setInterval(() => {
      count += Math.floor(Math.random() * 8) + 3;
      if (count >= 100) {
        count = 100;
        clearInterval(interval);
        completeLoader();
      }
      counterEl.textContent = String(count).padStart(3, '0');
    }, 45);

    // Safety watchdog: never freeze for more than 2.0s
    setTimeout(() => {
      clearInterval(interval);
      if (counterEl) counterEl.textContent = '100';
      completeLoader();
    }, 2000);
  }

  let loaderCompleted = false;
  function completeLoader() {
    if (loaderCompleted) return;
    loaderCompleted = true;

    // Trigger signal orange wipe curtain
    loader.classList.add('reveal-wipe');

    setTimeout(() => {
      loader.classList.add('is-hidden');
    }, 650);
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', runLoader);
  } else {
    runLoader();
  }

  // ==========================================================================
  // 3. FIXED HEADER SCROLL PROGRESS & CHAPTER TICKER
  // ==========================================================================
  const header = document.getElementById('site-header');
  const progressBar = document.getElementById('header-progress-bar');

  // Chapter tracking zones (dark header over the final assembly)
  const chapters = [
    { id: 'hero', dark: false },
    { id: 'section-question', dark: false },
    { id: 'section-case-opens', dark: false },
    { id: 'pinned-evidence-stage', dark: false },
    { id: 'section-central-reveal', dark: false },
    { id: 'section-evidence-room', dark: false },
    { id: 'section-limits', dark: false },
    { id: 'pinned-assembly-stage', dark: true }
  ];

  function updateHeader() {
    const scrollY = window.scrollY;
    const maxScroll = document.documentElement.scrollHeight - window.innerHeight;
    const progress = maxScroll > 0 ? (scrollY / maxScroll) * 100 : 0;

    if (progressBar) {
      progressBar.style.width = `${progress}%`;
    }

    // Determine active chapter
    let currentChapter = chapters[0];
    chapters.forEach(ch => {
      const el = document.getElementById(ch.id);
      if (el) {
        const rect = el.getBoundingClientRect();
        if (rect.top <= 200) {
          currentChapter = ch;
        }
      }
    });

    // Toggle header dark mode when over deep dark sections
    if (header) {
      if (currentChapter.dark) {
        header.classList.add('is-dark');
      } else {
        header.classList.remove('is-dark');
      }
    }
  }

  window.addEventListener('scroll', updateHeader, { passive: true });
  updateHeader();

  // If Lenis is active, synchronize header and pinned stage directly on Lenis ticks
  if (lenis) {
    lenis.on('scroll', () => {
      updateHeader();
      updatePinnedStage();
    });
  }

  // ==========================================================================
  // 3.5 NARRATIVE SECTION CHOREOGRAPHY (ENTER -> ASSEMBLE -> SETTLE -> TRANSITION)
  // ==========================================================================
  const revealElements = document.querySelectorAll('.section:not(.hero-section), .site-footer');
  if ('IntersectionObserver' in window && !prefersReducedMotion) {
    const sectionObserver = new IntersectionObserver((entries, obs) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          entry.target.classList.add('is-revealed');
          setTimeout(() => entry.target.classList.add('is-settled'), 850);
          obs.unobserve(entry.target);
        }
      });
    }, { threshold: 0.15, rootMargin: '0px 0px -50px 0px' });

    revealElements.forEach(sec => {
      sec.classList.add('reveal-on-scroll');
      sectionObserver.observe(sec);
    });
  } else {
    revealElements.forEach(sec => sec.classList.add('is-revealed', 'is-settled'));
  }

  // ==========================================================================
  // 4. SECTION THREE — PINNED SCROLL STAGE (FOLLOW THE EVIDENCE)
  // ==========================================================================
  const pinnedStage = document.getElementById('pinned-evidence-stage');
  const stmtRows = document.querySelectorAll('.code-stmt-row');
  const svgPaths = document.querySelectorAll('.svg-edge-path');
  const svgNodes = document.querySelectorAll('.svg-node-rect');
  const stageMetric = document.getElementById('pinned-metric-val');

  function updatePinnedStage() {
    if (!pinnedStage) return;

    const rect = pinnedStage.getBoundingClientRect();
    const totalDist = pinnedStage.clientHeight - window.innerHeight;
    if (totalDist <= 0) return;

    const progress = Math.min(Math.max(-rect.top / totalDist, 0), 1);

    // Step 1 (0.0 - 0.25): Raw state
    if (progress < 0.25) {
      stmtRows.forEach(r => {
        r.classList.remove('state-target', 'state-input', 'state-unrelated');
      });
      svgPaths.forEach(p => p.classList.remove('is-active'));
      if (stageMetric) stageMetric.textContent = '3 statements total';
    }
    // Step 2 (0.25 - 0.50): Focus locks onto df$total
    else if (progress < 0.50) {
      const r3 = document.getElementById('stmt-row-3');
      if (r3) r3.classList.add('state-target');
      document.getElementById('stmt-row-1')?.classList.remove('state-input', 'state-unrelated');
      document.getElementById('stmt-row-2')?.classList.remove('state-unrelated');
      svgPaths.forEach(p => p.classList.remove('is-active'));
      if (stageMetric) stageMetric.textContent = 'Target isolated: df$total';
    }
    // Step 3 (0.50 - 0.75): Inputs & Derivations illuminate
    else if (progress < 0.75) {
      document.getElementById('stmt-row-1')?.classList.add('state-input');
      document.getElementById('stmt-row-3')?.classList.add('state-target');
      document.getElementById('stmt-row-2')?.classList.remove('state-unrelated');
      svgPaths.forEach(p => p.classList.add('is-active'));
      if (stageMetric) stageMetric.textContent = 'price, quantity → total';
    }
    // Step 4 (0.75 - 1.0): Unrelated statement (title) recedes; 2 of 3 relevant
    else {
      document.getElementById('stmt-row-1')?.classList.add('state-input');
      document.getElementById('stmt-row-3')?.classList.add('state-target');
      document.getElementById('stmt-row-2')?.classList.add('state-unrelated');
      svgPaths.forEach(p => p.classList.add('is-active'));
      if (stageMetric) stageMetric.textContent = '2 of 3 statements relevant';
    }
  }

  window.addEventListener('scroll', updatePinnedStage, { passive: true });
  updatePinnedStage();

  // ==========================================================================
  // 5. SECTION FIVE — THE EVIDENCE ROOM (EXHIBITS SWITCHER)
  // ==========================================================================
  const exhibitTabs = document.querySelectorAll('.btn-exhibit-tab');
  const exhibitCards = document.querySelectorAll('.exhibit-card');

  exhibitTabs.forEach(tab => {
    tab.addEventListener('click', function () {
      const targetView = this.getAttribute('data-exhibit');
      exhibitTabs.forEach(t => t.classList.remove('is-active'));
      this.classList.add('is-active');

      const targetCard = document.getElementById(`exhibit-${targetView}`);
      if (targetCard) {
        if (lenis) {
          lenis.scrollTo(targetCard, { offset: -90 });
        } else {
          targetCard.scrollIntoView({ behavior: 'smooth', block: 'center' });
        }
      }
    });
  });



  // ==========================================================================
  // 7. NAVIGATION DRAWER (MENU)
  // ==========================================================================
  const menuToggle = document.getElementById('btn-menu-toggle');
  const navDrawer = document.getElementById('nav-drawer');
  const drawerClose = document.getElementById('drawer-close-btn');

  function toggleMenu() {
    if (navDrawer) navDrawer.classList.toggle('is-open');
  }

  if (menuToggle) menuToggle.addEventListener('click', toggleMenu);
  if (drawerClose) drawerClose.addEventListener('click', toggleMenu);

  document.querySelectorAll('.drawer-link').forEach(link => {
    link.addEventListener('click', () => {
      if (navDrawer) navDrawer.classList.remove('is-open');
    });
  });

  // ESC key handler for drawer
  window.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      if (navDrawer) navDrawer.classList.remove('is-open');
    }
  });
})();
