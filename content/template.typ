// Shared template for tobydavis.dev blog posts.
// Import with: #import "/template.typ": *

// --- Callout boxes ---
// These produce semantic HTML divs styled by blog.css

#let note(body) = html.elem("div", attrs: (class: "callout callout-note"), {
  [*Note:* ]
  body
})

#let tip(body) = html.elem("div", attrs: (class: "callout callout-tip"), {
  [*Tip:* ]
  body
})

#let warning(body) = html.elem("div", attrs: (class: "callout callout-warning"), {
  [*Warning:* ]
  body
})

#let success(body) = html.elem("div", attrs: (class: "callout callout-success"), {
  [*Success:* ]
  body
})
