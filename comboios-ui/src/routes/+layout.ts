// Static SPA (adapter-static with an index.html fallback behind nginx):
// render in the browser only, so dev/preview behave like production and
// loaders can fetch the API with relative URLs.
export const ssr = false;
