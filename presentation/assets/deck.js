(function () {
  var DECK_WIDTH = 1280;
  var DECK_HEIGHT = 720;
  var LANGUAGE_STORAGE_KEY = "slides-language";

  var hints = {
    en: "← → to navigate · F for fullscreen · T for theme · L for language",
    pl: "← → nawigacja · F pełny ekran · T motyw · L język"
  };

  var rustKeywords = "as|break|const|continue|crate|else|enum|fn|for|if|impl|in|let|loop|match|mod|move|mut|pub|ref|return|self|Self|static|struct|super|trait|type|unsafe|use|where|while|true|false";
  var rustPrimitives = "i8|i16|i32|i64|i128|isize|u8|u16|u32|u64|u128|usize|f32|f64|bool|char|str";
  var rustNumber = "0x[0-9a-fA-F_]+|0o[0-7_]+|0b[01_]+|\\d[\\d_]*(?:\\.\\d+)?(?:_?(?:" + rustPrimitives + "))?";

  function escapeHtml(text) {
    return text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
  }

  function wrap(className, text) {
    return '<span class="' + className + '">' + text + "</span>";
  }

  function highlightRust(source) {
    var pattern = new RegExp(
      "(\\/\\/[^\\n]*|\\/\\*[\\s\\S]*?\\*\\/)" +
      '|("(?:\\\\.|[^"\\\\])*")' +
      "|('(?:\\\\.|[^'\\\\])')" +
      "|\\b(" + rustKeywords + ")\\b" +
      "|\\b([a-z_][a-z0-9_]*!)" +
      "|\\b(" + rustNumber + ")\\b" +
      "|\\b(" + rustPrimitives + "|[A-Z][A-Za-z0-9_]*)\\b",
      "gu"
    );
    return source.replace(pattern, function (match, comment, string, character, keyword, macro, number, type) {
      if (comment) return wrap("tok-comment", comment);
      if (string) return wrap("tok-string", string);
      if (character) return wrap("tok-string", character);
      if (keyword) return wrap("tok-keyword", keyword);
      if (macro) return wrap("tok-macro", macro);
      if (number) return wrap("tok-number", number);
      return wrap("tok-type", type);
    });
  }

  function highlightToml(source) {
    return source.split("\n").map(function (line) {
      if (/^\s*\[.*\]\s*$/.test(line)) return wrap("tok-section", line);
      var keyValue = line.match(/^(\s*[A-Za-z0-9_-]+)(\s*=\s*)(.*)$/);
      if (!keyValue) return line;
      var value = keyValue[3].replace(/("[^"]*")/g, function (s) { return wrap("tok-string", s); });
      return wrap("tok-key", keyValue[1]) + keyValue[2] + value;
    }).join("\n");
  }

  function highlightShell(source) {
    return source.split("\n").map(function (line) {
      return line.replace(/^\$ /, wrap("tok-prompt", "$ "));
    }).join("\n");
  }

  function highlightError(source) {
    return source.split("\n").map(function (line) {
      if (/^error/.test(line)) return wrap("tok-error", line);
      if (/^warning/.test(line)) return wrap("tok-warning", line);
      return line.replace(/(help: .*)$/, function (s) { return wrap("tok-help", s); });
    }).join("\n");
  }

  var highlighters = {
    "lang-rust": highlightRust,
    "lang-toml": highlightToml,
    "lang-shell": highlightShell,
    "lang-error": highlightError
  };

  function highlightCodeBlocks() {
    document.querySelectorAll("pre code").forEach(function (block) {
      var source = escapeHtml(block.textContent);
      Object.keys(highlighters).forEach(function (language) {
        if (block.classList.contains(language)) source = highlighters[language](source);
      });
      block.innerHTML = source;
    });
  }

  function createElement(tag, className, parent) {
    var element = document.createElement(tag);
    if (className) element.className = className;
    parent.appendChild(element);
    return element;
  }

  var deck = document.getElementById("deck");
  var languages = Array.prototype.map.call(document.querySelectorAll(".slides[data-lang]"), function (group) {
    return group.dataset.lang;
  });
  var titles = {};
  languages.forEach(function (lang) {
    var heading = document.querySelector('.slides[data-lang="' + lang + '"] .slide h1');
    titles[lang] = heading ? heading.textContent.trim() : document.title;
  });

  var footer = createElement("div", "footer", document.body);
  var progress = createElement("div", "progress", footer);
  var counter = createElement("div", "counter", document.body);
  var hint = createElement("div", "help-hint", document.body);
  var languageSwitch = createElement("div", "lang-switch", document.body);
  languageSwitch.setAttribute("role", "group");
  languageSwitch.setAttribute("aria-label", "Language");
  var languageButtons = languages.map(function (lang) {
    var button = createElement("button", "", languageSwitch);
    button.type = "button";
    button.dataset.lang = lang;
    button.textContent = lang.toUpperCase();
    button.addEventListener("click", function () { setLanguage(lang); });
    return button;
  });
  if (languages.length < 2) languageSwitch.hidden = true;

  var slides = [];
  var current = 0;
  var language = languages[0];

  function readSavedLanguage() {
    var fromUrl = new URLSearchParams(location.search).get("lang");
    if (languages.indexOf(fromUrl) !== -1) return fromUrl;
    try {
      var saved = localStorage.getItem(LANGUAGE_STORAGE_KEY);
      if (languages.indexOf(saved) !== -1) return saved;
    } catch (error) {}
    return languages[0];
  }

  function setLanguage(newLanguage) {
    language = newLanguage;
    try { localStorage.setItem(LANGUAGE_STORAGE_KEY, language); } catch (error) {}
    document.documentElement.lang = language;
    document.title = titles[language];
    hint.textContent = hints[language] || hints.en;
    document.querySelectorAll(".slides").forEach(function (group) {
      group.classList.toggle("current", group.dataset.lang === language);
    });
    languageButtons.forEach(function (button) {
      button.setAttribute("aria-pressed", String(button.dataset.lang === language));
    });
    slides = document.querySelectorAll('.slides[data-lang="' + language + '"] .slide');
    show(current);
  }

  function nextLanguage() {
    return languages[(languages.indexOf(language) + 1) % languages.length];
  }

  function readSlideFromHash() {
    var number = parseInt(location.hash.replace("#", ""), 10);
    if (isNaN(number)) return 0;
    return Math.max(number - 1, 0);
  }

  function show(index) {
    current = Math.min(Math.max(index, 0), slides.length - 1);
    slides.forEach(function (slide, i) {
      slide.classList.toggle("active", i === current);
    });
    counter.textContent = (current + 1) + " / " + slides.length;
    progress.style.width = ((current + 1) / slides.length * 100) + "%";
    if (location.hash !== "#" + (current + 1)) history.replaceState(null, "", "#" + (current + 1));
  }

  function fit() {
    var scale = Math.min(window.innerWidth / DECK_WIDTH, (window.innerHeight - 40) / DECK_HEIGHT);
    deck.style.transform = "scale(" + scale + ")";
  }

  function toggleTheme() {
    var root = document.documentElement;
    var dark = root.dataset.theme
      ? root.dataset.theme === "dark"
      : window.matchMedia("(prefers-color-scheme: dark)").matches;
    root.dataset.theme = dark ? "light" : "dark";
  }

  function toggleFullscreen() {
    if (document.fullscreenElement) document.exitFullscreen();
    else document.documentElement.requestFullscreen();
  }

  document.addEventListener("keydown", function (event) {
    if (event.ctrlKey || event.metaKey || event.altKey) return;
    var key = event.key;
    if (key === "ArrowRight" || key === "PageDown" || key === " ") show(current + 1);
    else if (key === "ArrowLeft" || key === "PageUp") show(current - 1);
    else if (key === "Home") show(0);
    else if (key === "End") show(slides.length - 1);
    else if (key === "f" || key === "F") toggleFullscreen();
    else if (key === "t" || key === "T") toggleTheme();
    else if (key === "l" || key === "L") setLanguage(nextLanguage());
    else return;
    event.preventDefault();
  });

  document.addEventListener("click", function (event) {
    if (event.target.closest("a, button")) return;
    if (event.clientX > window.innerWidth / 2) show(current + 1);
    else show(current - 1);
  });

  window.addEventListener("hashchange", function () { show(readSlideFromHash()); });
  window.addEventListener("resize", fit);

  highlightCodeBlocks();
  fit();
  current = readSlideFromHash();
  setLanguage(readSavedLanguage());
})();
