export interface Env {
  CLIPS: R2Bucket;
  ORIGIN_URL: string;
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    if (request.method !== "GET" && request.method !== "HEAD") {
      return new Response("Method not allowed", { status: 405 });
    }
    const key = new URL(request.url).pathname.slice(1);
    const obj = await env.CLIPS.get(key);
    if (obj) {
      const headers = new Headers();
      obj.writeHttpMetadata(headers);
      headers.set("etag", obj.httpEtag);
      headers.set("cache-control", "public, max-age=31536000, immutable");
      return new Response(obj.body, { headers });
    }
    // Not in R2 yet (e.g. still rendering): fall back to the origin.
    return fetch(new Request(`${env.ORIGIN_URL}/${key}`, request));
  },
} satisfies ExportedHandler<Env>;
