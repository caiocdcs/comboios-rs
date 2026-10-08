FROM node:22-slim AS builder

WORKDIR /app

RUN npm install -g bun

COPY package.json bun.lock ./
RUN bun install --frozen-lockfile

COPY . .
ARG COMMIT_HASH=dev
ENV COMMIT_HASH=$COMMIT_HASH
RUN bun run build

FROM nginx:alpine

RUN apk add --no-cache gettext

COPY --from=builder /app/build /usr/share/nginx/html

COPY <<'EOF' /etc/nginx/conf.d/default.conf.template
server {
    listen 80;
    server_name _;
    root /usr/share/nginx/html;
    index index.html;

    location ~ ^/(stations|trains|ping|diagnostics|refresh) {
        proxy_pass ${API_PROXY_PASS};
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # The service worker and manifest must be revalidated so updates reach installed apps
    location = /service-worker.js {
        add_header Cache-Control "no-cache";
    }

    location = /manifest.webmanifest {
        types { application/manifest+json webmanifest; }
        add_header Cache-Control "no-cache";
    }

    # Content-hashed build output can be cached forever
    location /_app/immutable/ {
        add_header Cache-Control "public, max-age=31536000, immutable";
    }

    location / {
        add_header Cache-Control "no-cache";
        try_files $uri $uri/ /index.html;
    }
}
EOF

EXPOSE 80

CMD envsubst '${API_PROXY_PASS}' < /etc/nginx/conf.d/default.conf.template > /etc/nginx/conf.d/default.conf && nginx -g 'daemon off;'
