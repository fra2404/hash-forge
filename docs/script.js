// Tab functionality
function showTab(tabName) {
  // Hide all tab contents
  const tabContents = document.querySelectorAll(".tab-content");
  tabContents.forEach((content) => {
    content.classList.remove("active");
  });

  // Remove active class from all tab buttons
  const tabButtons = document.querySelectorAll(".tab-button");
  tabButtons.forEach((button) => {
    button.classList.remove("active");
  });

  // Show selected tab content
  document.getElementById(tabName + "-tab").classList.add("active");

  // Add active class to clicked button
  event.target.classList.add("active");
}

// Smooth scrolling for anchor links
document.querySelectorAll('a[href^="#"]').forEach((anchor) => {
  anchor.addEventListener("click", function (e) {
    e.preventDefault();

    const targetId = this.getAttribute("href");
    const targetElement = document.querySelector(targetId);

    if (targetElement) {
      targetElement.scrollIntoView({
        behavior: "smooth",
        block: "start",
      });
    }
  });
});

// Copy code blocks functionality
document.addEventListener("DOMContentLoaded", function () {
  const codeBlocks = document.querySelectorAll(".code-block");

  codeBlocks.forEach((block) => {
    // Add copy button
    const copyButton = document.createElement("button");
    copyButton.innerHTML = "📋 Copy";
    copyButton.className = "copy-button";
    copyButton.style.cssText = `
            position: absolute;
            top: 10px;
            right: 10px;
            background: var(--primary-color);
            color: white;
            border: none;
            padding: 5px 10px;
            border-radius: 4px;
            font-size: 0.8rem;
            cursor: pointer;
            opacity: 0;
            transition: opacity 0.3s ease;
        `;

    // Make parent relative for absolute positioning
    block.style.position = "relative";
    block.appendChild(copyButton);

    // Show/hide copy button on hover
    block.addEventListener("mouseenter", () => {
      copyButton.style.opacity = "1";
    });

    block.addEventListener("mouseleave", () => {
      copyButton.style.opacity = "0";
    });

    // Copy functionality
    copyButton.addEventListener("click", () => {
      const code = block.querySelector("code");
      const text = code.textContent || code.innerText;

      navigator.clipboard
        .writeText(text)
        .then(() => {
          copyButton.innerHTML = "✅ Copied!";
          setTimeout(() => {
            copyButton.innerHTML = "📋 Copy";
          }, 2000);
        })
        .catch(() => {
          // Fallback for older browsers
          const textArea = document.createElement("textarea");
          textArea.value = text;
          document.body.appendChild(textArea);
          textArea.select();
          document.execCommand("copy");
          document.body.removeChild(textArea);

          copyButton.innerHTML = "✅ Copied!";
          setTimeout(() => {
            copyButton.innerHTML = "📋 Copy";
          }, 2000);
        });
    });
  });
});

// Animated counters for statistics (if needed in future)
function animateCounter(element, target, duration = 2000) {
  let start = 0;
  const increment = target / (duration / 16);

  const timer = setInterval(() => {
    start += increment;
    element.textContent = Math.floor(start);

    if (start >= target) {
      element.textContent = target;
      clearInterval(timer);
    }
  }, 16);
}

// Intersection Observer for animations
const observerOptions = {
  threshold: 0.1,
  rootMargin: "0px 0px -50px 0px",
};

const observer = new IntersectionObserver((entries) => {
  entries.forEach((entry) => {
    if (entry.isIntersecting) {
      entry.target.style.opacity = "1";
      entry.target.style.transform = "translateY(0)";
    }
  });
}, observerOptions);

// Add animation classes to elements when page loads
document.addEventListener("DOMContentLoaded", function () {
  const animatedElements = document.querySelectorAll(
    ".feature-card, .algorithm-category, .download-card, .install-method, .example, .gui-feature"
  );

  animatedElements.forEach((element) => {
    element.style.opacity = "0";
    element.style.transform = "translateY(20px)";
    element.style.transition = "opacity 0.6s ease, transform 0.6s ease";
    observer.observe(element);
  });
});

// Download button analytics (placeholder for future implementation)
function trackDownload(platform) {
  // Analytics tracking would go here
  console.log(`Download tracked: ${platform}`);

  // Optional: Show a thank you message
  const button = event.target;
  const originalText = button.textContent;
  button.textContent = "✅ Download Started!";

  setTimeout(() => {
    button.textContent = originalText;
  }, 3000);
}
