FROM node:20-bookworm

WORKDIR /app

COPY package*.json ./
RUN npm install

COPY . .

# Install Playwright browsers
RUN npx playwright install --with-deps chromium

CMD ["npx", "ts-node", "index.ts"]
