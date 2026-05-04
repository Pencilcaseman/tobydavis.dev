// Template for blog posts
// #import "/template.typ": *
//
// Might be worth creating versioned templates in the future so as to not break
// old posts with new features/changes.

#let template(body) = {
  show math.equation.where(block: false): it => {
    if target() == "html" {
      html.elem("span", attrs: (role: "math"), html.frame(it))
    } else {
      it
    }
  }

  show math.equation.where(block: true): it => {
    if target() == "html" {
      html.elem("figure", attrs: (role: "math"), html.frame(it))
    } else {
      it
    }
  }

  // Render any non-table/non-raw figure body as inline SVG in HTML mode. CeTZ
  // canvases, fletcher diagrams, and arbitrary drawings all work via html.frame.
  // Tables and raw blocks pass through to Typst's native HTML emitters.
  show figure: it => {
    if target() == "html" and it.kind != table and it.kind != raw {
      html.elem("figure", attrs: (class: "diagram"), {
        html.frame(it.body)
        if it.caption != none {
          html.elem("figcaption", it.caption.body)
        }
      })
    } else {
      it
    }
  }

  body
}

// Wrap arbitrary graphical content so it renders as inline SVG under HTML. Pass
// through to `figure` otherwise.
#let diagram(body, caption: none) = context {
  if target() == "html" {
    html.elem("figure", attrs: (class: "diagram"), {
      html.frame(body)
      if caption != none {
        html.elem("figcaption", caption)
      }
    })
  } else {
    figure(body, caption: caption)
  }
}

// Callouts
#let note(body) = html.elem("div", attrs: (class: "callout callout-note"), {
  [*Note:* ]
  body
})

#let tip(body) = html.elem("div", attrs: (class: "callout callout-tip"), {
  [*Tip:* ]
  body
})

#let warning(body) = html.elem(
  "div",
  attrs: (class: "callout callout-warning"),
  {
    [*Warning:* ]
    body
  },
)

#let success(body) = html.elem(
  "div",
  attrs: (class: "callout callout-success"),
  {
    [*Success:* ]
    body
  },
)
