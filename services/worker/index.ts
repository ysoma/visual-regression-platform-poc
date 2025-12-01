import express from 'express';
import { chromium, Browser, BrowserContext } from 'playwright';
import { S3Client, PutObjectCommand } from '@aws-sdk/client-s3';
import { v4 as uuidv4 } from 'uuid';

const app = express();
app.use(express.json());

const PORT = process.env.PORT || 3000;
const S3_ENDPOINT = process.env.S3_ENDPOINT || 'http://minio:9000';
const S3_BUCKET = process.env.S3_BUCKET || 'screenshots';
const S3_REGION = process.env.S3_REGION || 'us-east-1';

const s3 = new S3Client({
  region: S3_REGION,
  endpoint: S3_ENDPOINT,
  forcePathStyle: true,
  credentials: {
    accessKeyId: process.env.AWS_ACCESS_KEY_ID || 'minioadmin',
    secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY || 'minioadmin',
  },
});

let browser: Browser;

async function initBrowser() {
  browser = await chromium.launch({
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox'],
  });
  console.log('Browser launched');
}

app.post('/screenshot', async (req, res) => {
  const { url, options } = req.body;

  if (!url) {
    return res.status(400).json({ error: 'URL is required' });
  }

  let context: BrowserContext | null = null;
  let page = null;

  try {
    context = await browser.newContext(options?.contextOptions);
    page = await context.newPage();

    if (options?.viewport) {
      await page.setViewportSize(options.viewport);
    }

    await page.goto(url, { waitUntil: 'networkidle' });

    const screenshotBuffer = await page.screenshot({
      fullPage: options?.fullPage || true,
    });

    const key = `${uuidv4()}.png`;

    await s3.send(
      new PutObjectCommand({
        Bucket: S3_BUCKET,
        Key: key,
        Body: screenshotBuffer,
        ContentType: 'image/png',
      })
    );

    res.json({
      success: true,
      url,
      s3Key: key,
      s3Url: `${S3_ENDPOINT}/${S3_BUCKET}/${key}`,
    });
  } catch (error: any) {
    console.error('Screenshot failed:', error);
    res.status(500).json({ error: error.message });
  } finally {
    if (page) await page.close();
    if (context) await context.close();
  }
});

app.get('/health', (req, res) => {
  res.json({ status: 'ok' });
});

initBrowser().then(() => {
  app.listen(PORT, () => {
    console.log(`Worker service running on port ${PORT}`);
  });
});
