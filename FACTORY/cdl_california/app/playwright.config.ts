import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir: 'tests/e2e',
  timeout: 90_000,
  retries: 0,
  reporter: [['list']],
  use: { trace: 'off', screenshot: 'only-on-failure' },
  projects: [
    { name: 'mobile', use: { viewport: { width: 390, height: 844 }, browserName: 'chromium' } },
    { name: 'desktop', use: { viewport: { width: 1280, height: 800 }, browserName: 'chromium' } },
  ],
});
