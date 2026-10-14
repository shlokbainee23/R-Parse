/**
 * R-PARSE — THE INVESTIGATION — Main Interaction & Motion Controller
 * 
 * Orchestrates Lenis smooth scrolling, the evidence loader, header chapter ticker,
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
  const tickerText = document.getElementById('ticker-chapter-text');

  // Chapter tracking zones
  const chapters = [
    { id: 'hero', name: 'CASE 000 · ORIGIN OF RESULT', dark: false },
    { id: 'section-question', name: 'CASE FILE 001 · df$total', dark: false },
    { id: 'section-case-opens', name: 'EXHIBIT · SOURCE AS SCENE', dark: false },
    { id: 'pinned-evidence-stage', name: 'TRAIL · COLUMN DERIVATION', dark: false },
    { id: 'section-central-reveal', name: 'SLICE · THE EVIDENCE SHORT', dark: false },
    { id: 'section-evidence-room', name: 'SECTION 05 · FIVE EXHIBITS', dark: false },
    { id: 'section-limits', name: 'BOUNDARY · UNKNOWN_STATIC', dark: false },
    { id: 'pinned-assembly-stage', name: 'FINAL ARCHITECTURE ASSEMBLY', dark: true }
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

    if (tickerText && tickerText.textContent !== currentChapter.name) {
      tickerText.textContent = currentChapter.name;
    }

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
  // 6. STUDIO LAUNCH & CODE-ENTRY TRANSITION CONTROLLER (REFINED EDITORIAL)
  // ==========================================================================
  const studioModal = document.getElementById('studio-modal');
  const studioDialog = document.getElementById('studio-dialog');
  const studioLaunchView = document.getElementById('studio-launch-view');
  const studioEntryView = document.getElementById('studio-entry-view');
  const btnCloseModal = document.getElementById('modal-close-btn');
  const btnOpenStudio = document.getElementById('btn-open-studio-launch');
  const btnStudioBack = document.getElementById('btn-studio-back');
  const entryTextarea = document.getElementById('entry-source-textarea');
  const entryStats = document.getElementById('entry-char-count');
  const btnAnalyze = document.getElementById('btn-analyze-entry');
  const btnAnalyzeText = document.getElementById('btn-analyze-text');
  const exampleSelect = document.getElementById('entry-example-select');
  const fileInput = document.getElementById('entry-file-input');
  const btnLoadFile = document.getElementById('btn-entry-load-file');
  const validationError = document.getElementById('entry-validation-error');
  const feedbackBanner = document.getElementById('studio-analysis-feedback');
  const launchHint = document.getElementById('studio-launch-hint');

  // Real Canonical R-PARSE Pipeline Fixtures
  const EXAMPLES = {
    showcase: `# Showcase Pipeline: Multi-step dataframe transformation\ndf <- data.frame(price = 100, quantity = 2, age = 30)\nfiltered <- filter(df, price > 0)\ndf2 <- mutate(filtered, total = price * quantity, discount = age * 0.05)\ndf3 <- mutate(df2, tax = total * 0.18)\n`,
    filter: `# Filter Provenance: Sequential row filtering\ndf <- data.frame(price = 10, quantity = 5, status = 1)\ndf1 <- filter(df, price > 0)\ndf2 <- filter(df1, status == 1)\ndf3 <- mutate(df2, total = price * quantity)\n`,
    canonical: `# Canonical Lineage: Direct column derivation\ndf <- data.frame(price = 10, quantity = 5)\ndf$total <- df$price * df$quantity\ndf2 <- mutate(df, tax = total * 0.1)\n`,
    adversarial: `# Adversarial Alias Renaming\ndf <- data.frame(val = 1:5, cat = c("a", "b", "a", "b", "a"))\ndf_alias <- df\ndf_alias$score <- df_alias$val * 2\nres <- summarise(group_by(df_alias, cat), mean_score = mean(score))\n`
  };

  let currentState = 'launch'; // 'launch' | 'entry'
  let isTransitioning = false;
  let transitionTimers = [];
  let lastActiveTrigger = null;
  let isAnalyzing = false;

  function updateEntryStats() {
    if (!entryTextarea || !entryStats) return;
    const text = entryTextarea.value;
    const lines = text ? text.split(/\r?\n/).length : 0;
    const chars = text.length;
    entryStats.textContent = `${lines} lines · ${chars} chars`;
  }

  function hideError() {
    if (validationError) {
      validationError.style.display = 'none';
      validationError.textContent = '';
    }
  }

  function showError(msg) {
    if (validationError) {
      validationError.textContent = msg;
      validationError.style.display = 'block';
    }
  }

  function hideFeedback() {
    if (feedbackBanner) {
      feedbackBanner.style.display = 'none';
      feedbackBanner.className = 'studio-feedback-banner';
      feedbackBanner.innerHTML = '';
    }
  }

  function showFeedback(htmlContent, isError = false) {
    if (feedbackBanner) {
      feedbackBanner.innerHTML = htmlContent;
      feedbackBanner.className = `studio-feedback-banner${isError ? ' is-error' : ''}`;
      feedbackBanner.style.display = 'flex';
    }
  }

  // Stage-coordinated Transition from Launch Screen to Code Entry
  function transitionToEntry() {
    if (isTransitioning || currentState === 'entry') return;
    isTransitioning = true;

    // Stage 1: Immediate button feedback & guard
    if (btnOpenStudio) {
      btnOpenStudio.classList.add('is-loading');
      btnOpenStudio.disabled = true;
    }

    const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

    if (reducedMotion) {
      if (studioLaunchView) studioLaunchView.style.display = 'none';
      if (studioEntryView) studioEntryView.style.display = 'block';
      if (studioDialog) studioDialog.classList.add('is-expanded');
      if (btnStudioBack) btnStudioBack.style.display = 'inline-flex';
      currentState = 'entry';
      isTransitioning = false;
      if (btnOpenStudio) {
        btnOpenStudio.classList.remove('is-loading');
        btnOpenStudio.disabled = false;
      }
      if (entryTextarea) {
        entryTextarea.focus();
        updateEntryStats();
      }
      return;
    }

    // Stage 2: Launch content moves aside (100–450ms)
    if (studioLaunchView) studioLaunchView.classList.add('is-exiting');
    if (studioDialog) studioDialog.classList.add('is-expanded');

    // Stage 3: The editor emerges (320–700ms)
    const t1 = setTimeout(() => {
      if (studioLaunchView) {
        studioLaunchView.style.display = 'none';
        studioLaunchView.classList.remove('is-exiting');
      }
      if (studioEntryView) {
        studioEntryView.style.display = 'block';
      }
      if (btnStudioBack) {
        btnStudioBack.style.display = 'inline-flex';
      }
      currentState = 'entry';
    }, 320);
    transitionTimers.push(t1);

    // Stage 4: Layout settles and editor receives focus (650–850ms)
    const t2 = setTimeout(() => {
      isTransitioning = false;
      if (btnOpenStudio) {
        btnOpenStudio.classList.remove('is-loading');
        btnOpenStudio.disabled = false;
      }
      if (entryTextarea) {
        entryTextarea.focus();
        updateEntryStats();
      }
    }, 720);
    transitionTimers.push(t2);
  }

  // Smooth Return Transition from Code Entry to Launch Screen (Preserves entered code)
  function transitionToLaunch() {
    if (isTransitioning || currentState === 'launch') return;
    isTransitioning = true;

    hideError();
    hideFeedback();

    const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

    if (reducedMotion) {
      if (studioEntryView) studioEntryView.style.display = 'none';
      if (studioLaunchView) studioLaunchView.style.display = 'grid';
      if (studioDialog) studioDialog.classList.remove('is-expanded');
      if (btnStudioBack) btnStudioBack.style.display = 'none';
      currentState = 'launch';
      isTransitioning = false;
      if (btnOpenStudio) btnOpenStudio.focus();
      return;
    }

    // Reverse motion
    if (studioEntryView) {
      studioEntryView.style.opacity = '0';
      studioEntryView.style.transform = 'translateY(12px)';
      studioEntryView.style.transition = 'opacity 0.22s ease, transform 0.22s var(--studio-ease)';
    }
    if (studioDialog) studioDialog.classList.remove('is-expanded');

    const t1 = setTimeout(() => {
      if (studioEntryView) {
        studioEntryView.style.display = 'none';
        studioEntryView.style.opacity = '';
        studioEntryView.style.transform = '';
        studioEntryView.style.transition = '';
      }
      if (btnStudioBack) btnStudioBack.style.display = 'none';
      if (studioLaunchView) studioLaunchView.style.display = 'grid';
      currentState = 'launch';
    }, 240);
    transitionTimers.push(t1);

    const t2 = setTimeout(() => {
      isTransitioning = false;
      if (btnOpenStudio) btnOpenStudio.focus();
    }, 500);
    transitionTimers.push(t2);
  }

  function openStudioModal(triggerEl) {
    if (studioModal) {
      lastActiveTrigger = triggerEl || document.activeElement;
      studioModal.classList.add('is-open');
      studioModal.style.display = 'flex';

      // Focus management based on state
      setTimeout(() => {
        if (currentState === 'entry' && entryTextarea) {
          entryTextarea.focus();
        } else if (btnOpenStudio) {
          btnOpenStudio.focus();
        }
      }, 50);

      checkStudioDaemonQuiet();
    }
  }

  function closeStudioModal() {
    if (studioModal) {
      // Clear ongoing transition timers safely
      transitionTimers.forEach(clearTimeout);
      transitionTimers = [];
      isTransitioning = false;

      if (btnOpenStudio) {
        btnOpenStudio.classList.remove('is-loading');
        btnOpenStudio.disabled = false;
      }

      studioModal.classList.remove('is-open');
      studioModal.style.display = 'none';

      if (lastActiveTrigger && typeof lastActiveTrigger.focus === 'function') {
        lastActiveTrigger.focus();
      }
    }
  }

  // Delegated click listener for studio modal triggers across page
  document.addEventListener('click', (e) => {
    const trigger = e.target.closest('.trigger-studio-modal');
    if (trigger) {
      e.preventDefault();
      openStudioModal(trigger);
    }
  });

  if (btnCloseModal) btnCloseModal.addEventListener('click', closeStudioModal);
  if (btnOpenStudio) btnOpenStudio.addEventListener('click', transitionToEntry);
  if (btnStudioBack) btnStudioBack.addEventListener('click', transitionToLaunch);

  // Close when clicking modal backdrop outside dialog
  if (studioModal) {
    studioModal.addEventListener('click', (e) => {
      if (e.target === studioModal) closeStudioModal();
    });

    // Keyboard focus trap & Escape handling inside modal
    studioModal.addEventListener('keydown', (e) => {
      if (e.key === 'Escape') {
        e.preventDefault();
        closeStudioModal();
        return;
      }
      if (e.key === 'Tab') {
        // Collect visible focusable elements within current modal state
        const focusables = Array.from(studioModal.querySelectorAll(
          'button:not([disabled]), [href]:not(.is-disabled), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])'
        )).filter(el => {
          return el.offsetParent !== null && !el.closest('[style*="display: none"]');
        });

        if (focusables.length === 0) return;
        const first = focusables[0];
        const last = focusables[focusables.length - 1];

        if (e.shiftKey && document.activeElement === first) {
          e.preventDefault();
          last.focus();
        } else if (!e.shiftKey && document.activeElement === last) {
          e.preventDefault();
          first.focus();
        }
      }
    });
  }

  // Code editor interactions
  if (entryTextarea) {
    entryTextarea.addEventListener('input', () => {
      updateEntryStats();
      hideError();
      hideFeedback();
    });

    entryTextarea.addEventListener('keydown', (e) => {
      // Shift+Tab or Tab indentation handling
      if (e.key === 'Tab' && !e.shiftKey && !e.ctrlKey && !e.metaKey) {
        e.preventDefault();
        const start = entryTextarea.selectionStart;
        const end = entryTextarea.selectionEnd;
        entryTextarea.value = entryTextarea.value.substring(0, start) + '  ' + entryTextarea.value.substring(end);
        entryTextarea.selectionStart = entryTextarea.selectionEnd = start + 2;
        updateEntryStats();
        return;
      }

      // Ctrl+Enter or Cmd+Enter to execute analysis
      if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
        e.preventDefault();
        executeStudioAnalysis();
      }
    });
  }

  // Load Example script handler
  if (exampleSelect) {
    exampleSelect.addEventListener('change', (e) => {
      const val = e.target.value;
      if (!val) return;
      if (EXAMPLES[val] && entryTextarea) {
        entryTextarea.value = EXAMPLES[val];
        updateEntryStats();
        hideError();
        hideFeedback();
        entryTextarea.focus();
      }
    });
  }

  // Load File handler
  if (btnLoadFile && fileInput) {
    btnLoadFile.addEventListener('click', () => fileInput.click());
    fileInput.addEventListener('change', (e) => {
      const file = e.target.files[0];
      if (!file) return;
      const reader = new FileReader();
      reader.onload = (evt) => {
        if (entryTextarea) {
          entryTextarea.value = evt.target.result;
          updateEntryStats();
          hideError();
          hideFeedback();
          entryTextarea.focus();
        }
      };
      reader.readAsText(file);
    });
  }

  // Primary Analyze Action Execution
  function executeStudioAnalysis() {
    if (!entryTextarea) return;
    const code = entryTextarea.value.trim();
    if (!code) {
      showError('Please enter or load R code to analyze.');
      entryTextarea.focus();
      return;
    }

    if (isAnalyzing) return;
    isAnalyzing = true;
    hideError();
    hideFeedback();

    if (btnAnalyze) {
      btnAnalyze.disabled = true;
      if (btnAnalyzeText) btnAnalyzeText.textContent = 'Analyzing...';
    }

    const isHttp = window.location.protocol.startsWith('http');
    const analyzeUrl = isHttp ? '/api/analyze' : 'http://127.0.0.1:8080/api/analyze';
    const studioUrl = isHttp ? '/studio' : 'http://127.0.0.1:8080/studio';

    fetch(analyzeUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json'
      },
      body: JSON.stringify({
        source: code,
        source_code: code
      })
    })
      .then(res => {
        if (!res.ok) {
          throw new Error(`HTTP ${res.status}`);
        }
        return res.json();
      })
      .then(data => {
        if (data.status === 'ok' || data.summary) {
          const stmts = data.summary?.statement_count ?? 0;
          const cols = data.summary?.column_version_count ?? 0;
          const nodes = data.summary?.pdg_node_count ?? (data.pdg?.nodes?.length ?? 0);
          showFeedback(
            `<span>✓ Analysis ready: <strong>${stmts}</strong> statements, <strong>${cols}</strong> columns, <strong>${nodes}</strong> PDG nodes.</span>` +
            `<a href="${studioUrl}" target="_blank" class="studio-feedback-link">Open Studio Workspace ↗</a>`
          );
        } else if (data.diagnostics && data.diagnostics.length > 0) {
          showFeedback(`Syntax / analysis note: ${data.diagnostics[0].message || 'Unrecognized syntax.'}`, true);
        } else {
          showFeedback(`Analysis complete. <a href="${studioUrl}" target="_blank" class="studio-feedback-link">Open Studio Workspace ↗</a>`);
        }
      })
      .catch(err => {
        showFeedback(`Analysis backend unavailable. Check service status or connectivity.`, true);
      })
      .finally(() => {
        isAnalyzing = false;
        if (btnAnalyze) {
          btnAnalyze.disabled = false;
          if (btnAnalyzeText) btnAnalyzeText.textContent = 'Analyze Code';
        }
      });
  }

  if (btnAnalyze) {
    btnAnalyze.addEventListener('click', executeStudioAnalysis);
  }

  // Quiet daemon check (non-blocking, sets informative subtle hint only)
  function checkStudioDaemonQuiet() {
    const isHttp = window.location.protocol.startsWith('http');
    const healthUrl = isHttp ? '/api/health' : 'http://127.0.0.1:8080/api/health';

    fetch(healthUrl, { method: 'GET', mode: 'cors' })
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then(data => {
        if (data.status === 'ok' && launchHint) {
          launchHint.textContent = `Backend online (v${data.version || '0.1.0'}) · Engine ready`;
          launchHint.style.color = '#10B981';
        }
      })
      .catch(() => {
        if (launchHint) {
          launchHint.textContent = 'Backend offline';
          launchHint.style.color = '#EF4444';
        }
      });
  }

  // Initial quiet health probe
  checkStudioDaemonQuiet();

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

  // ESC key handler for modals and drawer
  window.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      closeStudioModal();
      if (navDrawer) navDrawer.classList.remove('is-open');
    }
  });
})();
