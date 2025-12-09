#!/usr/bin/env node
/**
 * Wise API Documentation Scraper
 *
 * Uses Playwright to scrape rendered documentation pages and save them as markdown.
 *
 * Usage:
 *   npx playwright install chromium  # first time only
 *   node scripts/scrape-docs.js
 */

const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');

const DOCS_DIR = path.join(__dirname, '..', 'docs');
const URLS_FILE = path.join(DOCS_DIR, 'urls.txt');

// Read URLs from file
function getUrls() {
  const content = fs.readFileSync(URLS_FILE, 'utf-8');
  return content.split('\n').filter(url => url.trim() && !url.startsWith('#'));
}

// Convert URL to file path
function urlToFilePath(url) {
  const urlObj = new URL(url);
  let pathname = urlObj.pathname;

  // Remove leading slash
  if (pathname.startsWith('/')) {
    pathname = pathname.substring(1);
  }

  // Handle root URL
  if (!pathname) {
    pathname = 'index';
  }

  return path.join(DOCS_DIR, pathname + '.md');
}

// Extract article content and convert to markdown-like text
async function extractContent(page) {
  return await page.evaluate(() => {
    const article = document.querySelector('article');
    if (!article) {
      return { title: document.title, content: '' };
    }

    // Get the page title
    const h1 = article.querySelector('h1');
    const title = h1 ? h1.textContent.replace('Copy', '').trim() : document.title;

    // Get raw text content (innerText preserves structure better than textContent)
    const content = article.innerText;

    return { title, content };
  });
}

// Ensure directory exists
function ensureDir(filePath) {
  const dir = path.dirname(filePath);
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
}

// Main scraping function
async function scrape() {
  const urls = getUrls();
  console.log(`Found ${urls.length} URLs to scrape`);

  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext();
  const page = await context.newPage();

  let successCount = 0;
  let errorCount = 0;

  for (let i = 0; i < urls.length; i++) {
    const url = urls[i];
    const filePath = urlToFilePath(url);

    // Skip if already scraped (for resumability)
    if (fs.existsSync(filePath)) {
      console.log(`[${i + 1}/${urls.length}] Skipping (exists): ${url}`);
      successCount++;
      continue;
    }

    try {
      console.log(`[${i + 1}/${urls.length}] Scraping: ${url}`);

      await page.goto(url, { waitUntil: 'networkidle', timeout: 30000 });

      // Wait a bit for JS to render
      await page.waitForTimeout(1000);

      const { title, content } = await extractContent(page);

      if (!content) {
        console.log(`  Warning: No content found`);
        errorCount++;
        continue;
      }

      // Create markdown file
      const markdown = `# ${title}\n\nSource: ${url}\n\n---\n\n${content}`;

      ensureDir(filePath);
      fs.writeFileSync(filePath, markdown, 'utf-8');

      console.log(`  Saved: ${filePath}`);
      successCount++;

      // Small delay to be nice to the server
      await page.waitForTimeout(500);

    } catch (error) {
      console.error(`  Error: ${error.message}`);
      errorCount++;
    }
  }

  await browser.close();

  console.log(`\nDone! Success: ${successCount}, Errors: ${errorCount}`);
}

// Run
scrape().catch(console.error);
